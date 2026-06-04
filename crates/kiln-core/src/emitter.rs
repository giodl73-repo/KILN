use crate::{CheckReport, Diagnostic, Severity, Status};

pub fn emit_json_report(report: &CheckReport) -> String {
    if can_emit_build_record(report) {
        emit_build_record(report)
    } else {
        emit_diagnostics_envelope(report)
    }
}

pub fn can_emit_build_record(report: &CheckReport) -> bool {
    report.status != Status::NotReady
        && report.declaration.kiln_version.is_some()
        && report.declaration.capability.is_some()
}

fn emit_build_record(report: &CheckReport) -> String {
    let Some(kiln_version) = report.declaration.kiln_version.as_deref() else {
        return emit_diagnostics_envelope(report);
    };
    let Some(capability) = report.declaration.capability.as_ref() else {
        return emit_diagnostics_envelope(report);
    };

    object(vec![
        pair("kind", &quote("build_record")),
        pair("kiln_version", &quote(kiln_version)),
        pair(
            "capability",
            &object(vec![
                pair("id", &quote(&capability.id)),
                pair("version", &quote(&capability.version)),
            ]),
        ),
        pair("status", &quote(report.status.as_str())),
        pair("checked_inputs", &string_array(&checked_inputs(report))),
        pair("diagnostics", &diagnostics_json(&report.diagnostics)),
        pair("unresolved_gates", &string_array(&report.unresolved_gates)),
        pair(
            "evidence_obligations",
            &string_array(&report.evidence_obligations),
        ),
        pair("handoffs", &string_array(&report.unresolved_gates)),
        pair("trace", &string_array(&trace_ids(report))),
    ])
}

fn emit_diagnostics_envelope(report: &CheckReport) -> String {
    object(vec![
        pair("kind", &quote("diagnostics")),
        pair("status", &quote(report.status.as_str())),
        pair("diagnostics", &diagnostics_json(&report.diagnostics)),
        pair("unresolved_gates", &string_array(&report.unresolved_gates)),
        pair(
            "evidence_obligations",
            &string_array(&report.evidence_obligations),
        ),
        pair("trace", &string_array(&trace_ids(report))),
    ])
}

fn diagnostics_json(diagnostics: &[Diagnostic]) -> String {
    let items = diagnostics
        .iter()
        .map(|item| {
            object(vec![
                pair("category", &quote(item.category.as_str())),
                pair("severity", &quote(severity_str(item.severity))),
                pair("field", &quote(&item.field)),
                pair("message", &quote(&item.message)),
                pair("trace", &string_array(&item.trace)),
            ])
        })
        .collect::<Vec<_>>();
    array(items)
}

fn checked_inputs(report: &CheckReport) -> Vec<String> {
    report
        .declaration
        .fields
        .iter()
        .filter(|field| field.path == "inputs[]")
        .map(|field| field.value.clone())
        .collect()
}

fn trace_ids(report: &CheckReport) -> Vec<String> {
    let mut ids = Vec::new();
    for diagnostic in &report.diagnostics {
        for trace in &diagnostic.trace {
            if !ids.iter().any(|item| item == trace) {
                ids.push(trace.clone());
            }
        }
    }
    ids
}

fn pair(key: &str, value: &str) -> String {
    format!("{}:{}", quote(key), value)
}

fn object(fields: Vec<String>) -> String {
    format!("{{{}}}", fields.join(","))
}

fn array(values: Vec<String>) -> String {
    format!("[{}]", values.join(","))
}

fn string_array(values: &[String]) -> String {
    array(values.iter().map(|value| quote(value)).collect())
}

fn quote(value: &str) -> String {
    let mut escaped = String::new();
    escaped.push('"');
    for c in value.chars() {
        push_json_char(&mut escaped, c);
    }
    escaped.push('"');
    escaped
}

fn push_json_char(output: &mut String, c: char) {
    match c {
        '"' => output.push_str("\\\""),
        '\\' => output.push_str("\\\\"),
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        c if c.is_control() => output.push_str(&format!("\\u{:04x}", c as u32)),
        c => output.push(c),
    }
}

fn severity_str(severity: Severity) -> &'static str {
    severity.as_str()
}
