# fast-csv-to-json

A simple CLI tool for converting CSV file content to JSON.

我花了一個小時搓出來，接著優化了兩天的快速 CSV 轉 JSON CLI 小工具

## Installation

> Install Rust with [rustup](https://rustup.rs) first.

Use [`cargo` command](https://crates.io) to install this tool.

```
cargo install fast-csv-to-json
```

> SIMD optimization has been disabled due to Rust nightly toolchain requirement,
> you can still grab SIMD enabled code from
> [simd-enabled branch](https://github.com/ming900518/csv-to-json/tree/simd-enabled)
> and compile this tool manually.

## Usage

```
Fast CSV to JSON - A simple CLI tool for converting CSV file content to JSON.

Usage: fast-csv-to-json [OPTIONS] --input <CSV FILE>

Options:
  -i, --input <CSV FILE>    Specify CSV input file. Required
  -o, --output <JSON FILE>  Specify JSON output file. Optional (If not set, the result will be printed out directly.)
  -h, --help                Print help
  -V, --version             Print version
```

## About this Project

[83% 性能提升！CSV 至 JSON 轉換工具（fast-csv-to-json）優化記錄 (Blog post written in Chinese)](https://mingchang.tw/blog/Journey-of-Csv-to-Json-Optimization.md)
