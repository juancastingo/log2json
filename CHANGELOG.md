# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-10-06

### Added
- Initial release of **Log2JSON** streaming log normalizer.
- Automatic detection of ISO 8601, RFC 3339, Syslog, and Common Log Format timestamps.
- Log level detection (`TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `FATAL`, `CRITICAL`).
- IPv4 and IPv6 address parsing.
- HTTP request method, path, and status code extraction.
- Key-value attribute extraction into structured JSON properties.
- Custom regex support with named capture groups (`--pattern`).
- High-performance buffered streaming for infinite pipes and multi-GB files.
