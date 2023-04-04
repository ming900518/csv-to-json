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
[83% 性能提升！CSV 至 JSON 轉換工具優化記錄](https://mingchang.tw/blog/Journey-of-Csv-to-Json-Optimization.md)
