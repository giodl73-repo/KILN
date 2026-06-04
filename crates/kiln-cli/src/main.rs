#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::path::Path;

use kiln_core::{
    can_emit_build_record, check_declaration, emit_json_report, parse_declaration_text,
    CheckReport, Diagnostic, Severity, Status,
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    std::process::exit(run(args));
}

fn run(args: Vec<String>) -> i32 {
    match parse_args(&args) {
        Ok(options) => run_check(&options),
        Err(message) => {
            eprintln!("{message}");
            usage();
            2
        }
    }
}

fn run_check(options: &Options) -> i32 {
    let text = match fs::read_to_string(&options.input_path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("failed to read {}: {error}", options.input_path);
            return 2;
        }
    };

    let declaration = parse_declaration_text(&text);
    let report = check_declaration(declaration);
    write_output(options, &report)
}

fn write_output(options: &Options, report: &CheckReport) -> i32 {
    match options.format {
        OutputFormat::Text => println!("{}", text_report(report)),
        OutputFormat::Json => println!("{}", emit_json_report(report)),
    }

    if let Some(path) = options.out_path.as_deref() {
        if let Err(message) = write_build_record(path, report) {
            eprintln!("{message}");
            return 2;
        }
    }

    exit_code(report.status)
}

fn write_build_record(path: &str, report: &CheckReport) -> Result<(), String> {
    if !can_emit_build_record(report) {
        return Ok(());
    }

    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }
    }

    fs::write(path, emit_json_report(report))
        .map_err(|error| format!("failed to write {path}: {error}"))
}

fn parse_args(args: &[String]) -> Result<Options, String> {
    if args.len() < 2 || args.first().map(String::as_str) != Some("check") {
        return Err("expected: check <path-to-kiln.yaml>".to_owned());
    }

    let input_path = args[1].clone();
    let mut format = OutputFormat::Text;
    let mut out_path = None;
    let mut index = 2;

    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("--format requires text or json".to_owned());
                };
                format = OutputFormat::parse(value)?;
                index += 2;
            }
            "--out" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("--out requires a path".to_owned());
                };
                out_path = Some(value.clone());
                index += 2;
            }
            other => return Err(format!("unsupported argument: {other}")),
        }
    }

    Ok(Options {
        input_path,
        format,
        out_path,
    })
}

fn text_report(report: &CheckReport) -> String {
    let mut lines = vec![format!("status: {}", report.status.as_str())];
    if report.diagnostics.is_empty() {
        lines.push("diagnostics: none".to_owned());
    } else {
        lines.push("diagnostics:".to_owned());
        for diagnostic in &report.diagnostics {
            lines.push(format_diagnostic(diagnostic));
        }
    }
    lines.join("\n")
}

fn format_diagnostic(diagnostic: &Diagnostic) -> String {
    format!(
        "- {} {} {}: {}",
        severity_str(diagnostic.severity),
        diagnostic.category.as_str(),
        diagnostic.field,
        diagnostic.message
    )
}

fn exit_code(status: Status) -> i32 {
    match status {
        Status::Ready => 0,
        Status::NotReady | Status::Degraded => 1,
    }
}

fn severity_str(severity: Severity) -> &'static str {
    severity.as_str()
}

fn usage() {
    eprintln!(
        "usage: kiln check <path-to-kiln.yaml> [--format text|json] [--out <build-record.json>]"
    );
}

struct Options {
    input_path: String,
    format: OutputFormat,
    out_path: Option<String>,
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            _ => Err(format!("unsupported format: {value}")),
        }
    }
}
