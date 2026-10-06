use anyhow::Result;
use std::io::{BufRead, Write};

use crate::parser::{parse_log_line, ParserOptions};

pub fn process_stream<R: BufRead, W: Write>(
    reader: R,
    mut writer: W,
    options: &ParserOptions,
    pretty: bool,
) -> Result<usize> {
    let mut count = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        }

        let parsed = parse_log_line(&line, options);
        if pretty {
            let json_str = serde_json::to_string_pretty(&parsed)?;
            writeln!(writer, "{}", json_str)?;
        } else {
            let json_str = serde_json::to_string(&parsed)?;
            writeln!(writer, "{}", json_str)?;
        }
        count += 1;
    }

    writer.flush()?;
    Ok(count)
}
