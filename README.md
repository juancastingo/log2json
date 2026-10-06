# 📜 Log2JSON

[![CI](https://github.com/juancastingo/log2json/actions/workflows/ci.yml/badge.svg)](https://github.com/juancastingo/log2json/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/log2json.svg)](https://crates.io/crates/log2json)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)](https://github.com/juancastingo/log2json)

**Log2JSON** is a fast, streaming command-line utility written in Rust that converts plain text and semi-structured logs into normalized **JSON / JSONL**.

It automatically recognizes timestamps, severity levels, IPv4/IPv6 addresses, HTTP status codes/methods, and `key=value` attributes, turning unstructured application and server logs into structured data ready for `jq`, Elasticsearch, ClickHouse, or BigQuery.

---

## 📸 Preview

```bash
# Pipe any raw log line:
tail -f /var/log/app.log | log2json

# Input:
# 2026-10-06T12:00:00Z [INFO] User login user_id=42 latency_ms=18.5 ip=192.168.1.50

# Output (JSONL):
# {"timestamp":"2026-10-06T12:00:00Z","level":"INFO","ip":"192.168.1.50","fields":{"ip":"192.168.1.50","latency_ms":18.5,"user_id":42},"message":"2026-10-06T12:00:00Z [INFO] User login user_id=42 latency_ms=18.5 ip=192.168.1.50","raw":"2026-10-06T12:00:00Z [INFO] User login user_id=42 latency_ms=18.5 ip=192.168.1.50"}
```

---

## ✨ Features

- **High-Throughput Streaming**: Streams input line-by-line using buffered I/O with constant memory footprint—processes multi-gigabyte log files without exhausting RAM.
- **Smart Field Detection**:
  - **Timestamps**: ISO 8601, RFC 3339, Syslog, Apache / Nginx Common Log Format.
  - **Log Levels**: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `FATAL`, `CRITICAL`.
  - **Network**: IPv4 and IPv6 addresses.
  - **HTTP Requests**: Method (`GET`, `POST`, `PUT`, etc.), URL path, and HTTP response code (`200`, `404`, `500`).
  - **Key-Value Pairs**: Automatic extraction of `key=value` and `key="quoted value"`.
- **Custom Parsing Rules**: Supports `--pattern` with named regex capture groups (`(?P<timestamp>...) (?P<level>...)`).
- **Flexible Formats**: Outputs compact line-delimited JSON (`JSONL`) by default, or `--pretty` indented JSON.
- **Pipe & Unix Friendly**: Reads from `stdin` or files, writes to `stdout` or `--output <file>`.

---

## 🚀 Installation

### Using Cargo
```bash
cargo install log2json
```

### Pre-compiled Binaries
Download standalone binaries for Linux, macOS, and Windows from the [Releases](https://github.com/juancastingo/log2json/releases) page.

---

## 📖 Usage Examples

### 1. Process Large Log Files to JSONL
```bash
log2json access.log -o access.jsonl
```

### 2. Stream Real-Time Logs from Docker / Kubernetes
```bash
docker logs -f my_container | log2json | jq '.level, .fields'
```

### 3. Pretty-Printed JSON
```bash
log2json app.log --pretty
```

### 4. Custom Regex with Named Captures
```bash
log2json syslog --pattern '^(?P<time>[A-Z]{3} \d{2} \d{2}:\d{2}:\d{2}) (?P<host>\S+) (?P<service>\S+): (?P<msg>.*)$'
```

### 5. Omit Raw String to Reduce Output Size
```bash
log2json production.log --omit-raw -o compact.jsonl
```

---

## ⚙️ CLI Options

| Flag | Short | Description |
|---|---|---|
| `[FILE]` | | Input log file (default: stdin) |
| `--output <FILE>` | `-o` | Output file path (default: stdout) |
| `--pretty` | `-p` | Format JSON with indentation |
| `--omit-raw` | | Omit the `raw` original log line from output |
| `--pattern <REGEX>` | | Custom regex with named capture groups |

---

## 🧪 Testing

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

---

## 📄 License

MIT or Apache 2.0 © [Juan Castiñeira](https://github.com/juancastingo)
