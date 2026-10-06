use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::model::NormalizedLog;

static ISO_TIMESTAMP_RE: OnceLock<Regex> = OnceLock::new();
static CLF_TIMESTAMP_RE: OnceLock<Regex> = OnceLock::new();
static SYSLOG_TIMESTAMP_RE: OnceLock<Regex> = OnceLock::new();
static LOG_LEVEL_RE: OnceLock<Regex> = OnceLock::new();
static IPV4_RE: OnceLock<Regex> = OnceLock::new();
static HTTP_REQ_RE: OnceLock<Regex> = OnceLock::new();
static KV_PAIR_RE: OnceLock<Regex> = OnceLock::new();

#[derive(Default)]
pub struct ParserOptions {
    pub omit_raw: bool,
    pub custom_pattern: Option<Regex>,
}

pub fn parse_log_line(line: &str, options: &ParserOptions) -> NormalizedLog {
    let raw_line = line.trim_end();

    // If custom regex with named capture groups is provided
    if let Some(ref pattern) = options.custom_pattern {
        if let Some(caps) = pattern.captures(raw_line) {
            let mut fields = BTreeMap::new();
            let mut timestamp = None;
            let mut level = None;
            let mut message = None;

            for name in pattern.capture_names().flatten() {
                if let Some(val) = caps.name(name) {
                    let val_str = val.as_str().to_string();
                    match name {
                        "timestamp" | "time" | "date" => timestamp = Some(val_str),
                        "level" | "severity" => level = Some(val_str.to_uppercase()),
                        "message" | "msg" => message = Some(val_str),
                        other => {
                            fields.insert(other.to_string(), infer_json_value(&val_str));
                        }
                    }
                }
            }

            return NormalizedLog {
                timestamp,
                level,
                ip: None,
                http_method: None,
                http_status: None,
                http_path: None,
                fields,
                message: message.unwrap_or_else(|| raw_line.to_string()),
                raw: if options.omit_raw {
                    None
                } else {
                    Some(raw_line.to_string())
                },
            };
        }
    }

    // Default intelligent heuristic parsing
    let iso_re = ISO_TIMESTAMP_RE.get_or_init(|| {
        Regex::new(r"\b\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?\b")
            .unwrap()
    });
    let clf_re = CLF_TIMESTAMP_RE.get_or_init(|| {
        Regex::new(r"\[\d{2}/[A-Za-z]{3}/\d{4}:\d{2}:\d{2}:\d{2}\s+[+-]\d{4}\]").unwrap()
    });
    let syslog_re = SYSLOG_TIMESTAMP_RE.get_or_init(|| {
        Regex::new(r"\b(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}\b").unwrap()
    });
    let level_re = LOG_LEVEL_RE.get_or_init(|| {
        Regex::new(r"(?i)(?:\[|\b)(TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|FATAL|CRITICAL)(?:\]|\b)")
            .unwrap()
    });
    let ip_re = IPV4_RE.get_or_init(|| Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").unwrap());
    let http_re = HTTP_REQ_RE.get_or_init(|| {
        Regex::new(
            r#""?(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\s+([^\s"]+)\s+HTTP/[0-9.]+"?\s+(\d{3})"#,
        )
        .unwrap()
    });
    let kv_re = KV_PAIR_RE.get_or_init(|| {
        Regex::new(r#"\b([a-zA-Z_][a-zA-Z0-9_-]*)=(?:"([^"]*)"|'([^']*)'|([^\s,;]+))"#).unwrap()
    });

    let timestamp = iso_re
        .find(raw_line)
        .or_else(|| clf_re.find(raw_line))
        .or_else(|| syslog_re.find(raw_line))
        .map(|m| {
            m.as_str()
                .trim_matches(|c| c == '[' || c == ']')
                .to_string()
        });

    let level = level_re
        .captures(raw_line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_uppercase());

    let ip = ip_re.find(raw_line).map(|m| m.as_str().to_string());

    let mut http_method = None;
    let mut http_path = None;
    let mut http_status = None;

    if let Some(caps) = http_re.captures(raw_line) {
        http_method = caps.get(1).map(|m| m.as_str().to_string());
        http_path = caps.get(2).map(|m| m.as_str().to_string());
        if let Some(status_match) = caps.get(3) {
            http_status = status_match.as_str().parse::<u16>().ok();
        }
    }

    let mut fields = BTreeMap::new();
    for caps in kv_re.captures_iter(raw_line) {
        let key = caps.get(1).unwrap().as_str().to_string();
        let val = caps
            .get(2)
            .or_else(|| caps.get(3))
            .or_else(|| caps.get(4))
            .map(|m| m.as_str())
            .unwrap_or("");

        fields.insert(key, infer_json_value(val));
    }

    NormalizedLog {
        timestamp,
        level,
        ip,
        http_method,
        http_status,
        http_path,
        fields,
        message: raw_line.to_string(),
        raw: if options.omit_raw {
            None
        } else {
            Some(raw_line.to_string())
        },
    }
}

fn infer_json_value(val: &str) -> Value {
    if let Ok(i) = val.parse::<i64>() {
        Value::from(i)
    } else if let Ok(f) = val.parse::<f64>() {
        Value::from(f)
    } else if val.eq_ignore_ascii_case("true") {
        Value::Bool(true)
    } else if val.eq_ignore_ascii_case("false") {
        Value::Bool(false)
    } else if val.eq_ignore_ascii_case("null") {
        Value::Null
    } else {
        Value::String(val.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_app_log_with_kv() {
        let log = "2026-10-06T09:15:00Z [INFO] User login successful user_id=1042 latency_ms=18.5 ip=192.168.1.50";
        let parsed = parse_log_line(log, &ParserOptions::default());

        assert_eq!(parsed.timestamp.as_deref(), Some("2026-10-06T09:15:00Z"));
        assert_eq!(parsed.level.as_deref(), Some("INFO"));
        assert_eq!(parsed.ip.as_deref(), Some("192.168.1.50"));
        assert_eq!(parsed.fields.get("user_id"), Some(&Value::from(1042)));
        assert_eq!(parsed.fields.get("latency_ms"), Some(&Value::from(18.5)));
    }

    #[test]
    fn test_parse_nginx_access_log() {
        let log =
            r#"127.0.0.1 - - [06/Oct/2026:09:15:00 +0000] "GET /api/v1/health HTTP/1.1" 200 64"#;
        let parsed = parse_log_line(log, &ParserOptions::default());

        assert_eq!(parsed.ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(
            parsed.timestamp.as_deref(),
            Some("06/Oct/2026:09:15:00 +0000")
        );
        assert_eq!(parsed.http_method.as_deref(), Some("GET"));
        assert_eq!(parsed.http_path.as_deref(), Some("/api/v1/health"));
        assert_eq!(parsed.http_status, Some(200));
    }
}
