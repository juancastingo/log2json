use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "log2json",
    author = "Juan Castiñeira <juancastingo@gmail.com>",
    version,
    about = "Stream-convert semi-structured logs into normalized JSON / JSONL.",
    long_about = "log2json converts plain text and semi-structured log files or stdin streams into normalized JSON / JSONL. It automatically extracts timestamps, log levels, IP addresses, HTTP requests, and key=value attributes while preserving the raw message."
)]
pub struct CliArgs {
    /// Input log file to parse (reads from stdin if omitted or '-')
    #[arg(value_name = "FILE")]
    pub input: Option<String>,

    /// Write output to a file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "OUT_FILE")]
    pub output: Option<String>,

    /// Format output as indented pretty JSON instead of compact JSONL
    #[arg(short = 'p', long = "pretty")]
    pub pretty: bool,

    /// Omit the 'raw' original log line from output JSON to reduce payload size
    #[arg(long = "omit-raw")]
    pub omit_raw: bool,

    /// Custom regex pattern with named capture groups (e.g. '(?P<time>...) (?P<level>...)')
    #[arg(long = "pattern", value_name = "REGEX")]
    pub pattern: Option<String>,
}
