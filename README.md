# CSV-to-JSON

我晚上花了一個小時搓出來的 CSV 轉 JSON CLI 工具。

## Usage

```
CSV to JSON - A simple CLI tool for converting CSV file content to JSON.

Usage: csv-to-json [OPTIONS] --input <CSV FILE>

Options:
  -i, --input <CSV FILE>    Specify CSV input file. Required
  -o, --output <JSON FILE>  Specify JSON output file. Optional. (If not set, the result will be printed out directly.)
  -h, --help                Print help
  -V, --version             Print version
```

## Parallelism

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
