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

### Parallelism

想說程式都寫了，那來試試 [Rayon](https://crates.io/crates/rayon)
這個神奇套件能帶來多少的性能提升吧：

- 硬體：M1 MacBook Air, macOS 13.3
- 測試用 CSV 檔：就是一個全都是字， 573.5 MB 的 CSV
- 測試指令：`hyperfine --warmup 3 --runs 5 './csv-to-json -i test.csv -o output-1.json' './csv-to-json-rayon -i test.csv -o output-2.json'`

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

性能提升總是讓人開心

但......還能不能再更快一點呢？

（待續）
