# CSV-to-JSON

我晚上花了一個小時搓出來的 CSV 轉 JSON CLI 工具。

（編輯：似乎變成了性能提升記錄 😂 一小時寫扣一週改扣是吧）

### Usage

```
CSV to JSON - A simple CLI tool for converting CSV file content to JSON.

Usage: csv-to-json [OPTIONS] --input <CSV FILE>

Options:
  -i, --input <CSV FILE>    Specify CSV input file. Required
  -o, --output <JSON FILE>  Specify JSON output file. Optional. (If not set, the result will be printed out directly.)
  -h, --help                Print help
  -V, --version             Print version
```

## 優化記錄

稍微記錄下優化這個工具的歷程。

- 硬體：M1 MacBook Air, macOS 13.3
- 測試用 CSV 檔：就是一個全都是字， 573.5 MB 的 CSV
- 測試工具：[`hyperfine`](https://github.com/sharkdp/hyperfine)

### Parallelism

想說程式都寫了，那來試試 [Rayon](https://crates.io/crates/rayon)
這個神奇套件能帶來多少的性能提升吧：

> csv-to-json 用了 std 裡的傳統
> [`IntoIterator`](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html)
> ，而 csv-to-json-rayon 則用了 Rayon 的
> [`IntoParallelIterator`](https://docs.rs/rayon/1.7.0/rayon/iter/trait.IntoParallelIterator.html)

```
Benchmark 1: ./csv-to-json -i test.csv -o output-1.json
  Time (mean ± σ):      3.286 s ±  0.021 s    [User: 2.616 s, System: 0.585 s]
  Range (min … max):    3.260 s …  3.314 s    5 runs

Benchmark 2: ./csv-to-json-rayon -i test.csv -o output-2.json
  Time (mean ± σ):      2.794 s ±  0.014 s    [User: 4.537 s, System: 0.827 s]
  Range (min … max):    2.777 s …  2.814 s    5 runs

Summary
  './csv-to-json-rayon -i test.csv -o output-2.json' ran 1.18 ± 0.01 times faster than './csv-to-json -i test.csv -o output-1.json'
```

大概 18% 的性能進步。

### IndexMap

由於需要保留原始 CSV 的欄位排序，所以無法採用 std 中的
[`HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html)
，而是用了
[`BTreeMap`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html)

不過如果仔細看下
[When Should You Use Which Collection?](https://doc.rust-lang.org/stable/std/collections/index.html)
，會發現 `BTreeMap` 根本不是幹這個的

所以我又去翻了萬能的 crates.io ，看到了
[indexmap](https://crates.io/crates/indexmap) ，文件中宣稱 Fast to iterate 和
Preserves insertion order as long as you don't call `.remove()`

啊呀，剛好符合我的需求啊！

測試結果如下

```
Benchmark 1: ./csv-to-json -i test.csv -o output-1.json
  Time (mean ± σ):      3.248 s ±  0.014 s    [User: 2.614 s, System: 0.573 s]
  Range (min … max):    3.224 s …  3.259 s    5 runs

Benchmark 2: ./csv-to-json-rayon -i test.csv -o output-2.json
  Time (mean ± σ):      2.806 s ±  0.016 s    [User: 4.547 s, System: 0.842 s]
  Range (min … max):    2.786 s …  2.826 s    5 runs

Benchmark 3: ./csv-to-json-indexmap -i test.csv -o output-3.json
  Time (mean ± σ):      2.621 s ±  0.029 s    [User: 4.574 s, System: 0.701 s]
  Range (min … max):    2.599 s …  2.671 s    5 runs

Summary
  './csv-to-json-indexmap -i test.csv -o output-3.json' ran
    1.07 ± 0.01 times faster than './csv-to-json-rayon -i test.csv -o output-2.json'
    1.24 ± 0.01 times faster than './csv-to-json -i test.csv -o output-1.json'
```

比最原始的版本快了 24% ！

### DataFrame

性能提升總是讓人開心，但......還能不能再更快一點呢？

其實我原本並沒有打算要把 CSV
轉成其他格式，但網際網路就是這麼的神奇，常常在亂晃的時候看到可能有價值的東西

這邊用了 [Polars](https://www.pola.rs) ，可以將 CSV 轉換為 DataFrame ，號稱
lightning fast ，那就試試吧

> 根據 Polars API 文件中標示的說明：
> [get_row](https://pola-rs.github.io/polars/polars/frame/struct.DataFrame.html#method.get_row)
> 這個 method 因性能不好而不建議使用，所以這邊應該還有改進的可能（？）

```
Benchmark 1: ./csv-to-json -i test.csv -o output-1.json
  Time (mean ± σ):      3.258 s ±  0.024 s    [User: 2.630 s, System: 0.563 s]
  Range (min … max):    3.235 s …  3.292 s    5 runs

Benchmark 2: ./csv-to-json-rayon -i test.csv -o output-2.json
  Time (mean ± σ):      2.757 s ±  0.015 s    [User: 4.480 s, System: 0.787 s]
  Range (min … max):    2.735 s …  2.774 s    5 runs

Benchmark 3: ./csv-to-json-indexmap -i test.csv -o output-3.json
  Time (mean ± σ):      2.569 s ±  0.019 s    [User: 4.537 s, System: 0.650 s]
  Range (min … max):    2.547 s …  2.594 s    5 runs

Benchmark 4: ./csv-to-json-polar -i test.csv -o output-4.json
  Time (mean ± σ):      2.106 s ±  0.013 s    [User: 2.552 s, System: 0.573 s]
  Range (min … max):    2.086 s …  2.118 s    5 runs

Summary
  './csv-to-json-polar -i test.csv -o output-4.json' ran
    1.22 ± 0.01 times faster than './csv-to-json-indexmap -i test.csv -o output-3.json'
    1.31 ± 0.01 times faster than './csv-to-json-rayon -i test.csv -o output-2.json'
    1.55 ± 0.01 times faster than './csv-to-json -i test.csv -o output-1.json'
```

得到了 55% 的性能提升！

### SIMD

正當我滿意的準備 `git push` 時， Polars
的文件有個[小段落](https://pola-rs.github.io/polars/polars/index.html#simd)引起了我的注意

SIMD？？？你說辣個被 Intel 砍掉的 AVX-512 還能拿來加速？

> 我知道 M1 MacBook 沒有 AVX 系列指令集，不過如果我寫 ARM NEON
> 大概就沒幾個人知道那是啥了

裝上 nightly Rust toolchain ，features 裝好，`RUSTFLAGS="-C target-cpu=native" cargo +nightly build --release`！

```
Benchmark 1: ./csv-to-json -i test.csv -o output-1.json
  Time (mean ± σ):      3.271 s ±  0.009 s    [User: 2.617 s, System: 0.566 s]
  Range (min … max):    3.260 s …  3.284 s    5 runs

Benchmark 2: ./csv-to-json-rayon -i test.csv -o output-2.json
  Time (mean ± σ):      2.780 s ±  0.010 s    [User: 4.516 s, System: 0.789 s]
  Range (min … max):    2.770 s …  2.792 s    5 runs

Benchmark 3: ./csv-to-json-indexmap -i test.csv -o output-3.json
  Time (mean ± σ):      2.582 s ±  0.013 s    [User: 4.558 s, System: 0.649 s]
  Range (min … max):    2.571 s …  2.598 s    5 runs

Benchmark 4: ./csv-to-json-polar -i test.csv -o output-4.json
  Time (mean ± σ):      2.114 s ±  0.023 s    [User: 2.561 s, System: 0.574 s]
  Range (min … max):    2.083 s …  2.142 s    5 runs

Benchmark 5: ./csv-to-json-simd -i test.csv -o output-5.json
  Time (mean ± σ):      2.046 s ±  0.066 s    [User: 2.483 s, System: 0.554 s]
  Range (min … max):    2.004 s …  2.163 s    5 runs

Summary
  './csv-to-json-simd -i test.csv -o output-5.json' ran
    1.03 ± 0.04 times faster than './csv-to-json-polar -i test.csv -o output-4.json'
    1.26 ± 0.04 times faster than './csv-to-json-indexmap -i test.csv -o output-3.json'
    1.36 ± 0.04 times faster than './csv-to-json-rayon -i test.csv -o output-2.json'
    1.60 ± 0.05 times faster than './csv-to-json -i test.csv -o output-1.json'
```

只比沒開多 3% ，比我預期中的秒天秒地差了不少

不過到目前為止靠軟體的修改達到了 60% 的性能進步，這點依然讓我十分開心。

