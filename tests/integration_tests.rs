use log2json::parser::{parse_log_line, ParserOptions};
use log2json::process_stream;
use regex::Regex;
use serde_json::Value;
use std::io::Cursor;

#[test]
fn test_app_log_extraction() {
    let line = "2026-10-06T12:00:00Z [WARN] Failed database connection attempt=3 duration_ms=450.2 ip=10.0.0.1";
    let parsed = parse_log_line(line, &ParserOptions::default());

    assert_eq!(parsed.timestamp.as_deref(), Some("2026-10-06T12:00:00Z"));
    assert_eq!(parsed.level.as_deref(), Some("WARN"));
    assert_eq!(parsed.ip.as_deref(), Some("10.0.0.1"));
    assert_eq!(parsed.fields.get("attempt"), Some(&Value::from(3)));
    assert_eq!(parsed.fields.get("duration_ms"), Some(&Value::from(450.2)));
    assert_eq!(parsed.raw.as_deref(), Some(line));
}

#[test]
fn test_custom_regex_named_captures() {
    let line = "OCT 06 12:30:15 app_srv[4012]: [CRITICAL] Core dump triggered";
    let re = Regex::new(r"^(?P<time>[A-Z]{3} \d{2} \d{2}:\d{2}:\d{2}) (?P<service>[a-zA-Z0-9_-]+)\[(?P<pid>\d+)\]: \[(?P<level>[A-Z]+)\] (?P<msg>.*)$").unwrap();

    let options = ParserOptions {
        omit_raw: false,
        custom_pattern: Some(re),
    };

    let parsed = parse_log_line(line, &options);
    assert_eq!(parsed.timestamp.as_deref(), Some("OCT 06 12:30:15"));
    assert_eq!(parsed.level.as_deref(), Some("CRITICAL"));
    assert_eq!(parsed.message, "Core dump triggered");
    assert_eq!(parsed.fields.get("service"), Some(&Value::from("app_srv")));
    assert_eq!(parsed.fields.get("pid"), Some(&Value::from(4012)));
}

#[test]
fn test_streaming_pipeline() {
    let input_logs = "2026-10-06 09:00:00 [INFO] Starting service\n2026-10-06 09:00:01 [ERROR] Service crashed\n";
    let reader = Cursor::new(input_logs);
    let mut writer = Cursor::new(Vec::new());

    let count = process_stream(reader, &mut writer, &ParserOptions::default(), false).unwrap();
    assert_eq!(count, 2);

    let output_str = String::from_utf8(writer.into_inner()).unwrap();
    let lines: Vec<&str> = output_str.trim().lines().collect();
    assert_eq!(lines.len(), 2);

    let val1: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(val1["level"], "INFO");

    let val2: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(val2["level"], "ERROR");
}
