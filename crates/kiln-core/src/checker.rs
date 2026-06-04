use crate::{Declaration, Diagnostic, DiagnosticCategory, Severity, Status, KILN_VERSION};

const REQUIRED_SECTIONS: &[&str] = &["kiln", "capability", "inputs", "outputs", "checks"];
const KNOWN_SECTIONS: &[&str] = &[
    "kiln",
    "capability",
    "inputs",
    "outputs",
    "checks",
    "policy_needs",
    "package",
    "cal_refs",
    "runtime",
    "review",
];

pub fn check_declaration(declaration: Declaration) -> crate::CheckReport {
    let mut diagnostics = Vec::new();
    let mut unresolved_gates = Vec::new();

    check_required_sections(&declaration, &mut diagnostics);
    check_identity(&declaration, &mut diagnostics);
    check_unknown_sections(&declaration, &mut diagnostics);
    check_policy_boundary(&declaration, &mut diagnostics, &mut unresolved_gates);
    check_package_boundary(&declaration, &mut diagnostics, &mut unresolved_gates);
    check_cal_boundary(&declaration, &mut diagnostics, &mut unresolved_gates);
    check_runtime_boundary(&declaration, &mut diagnostics, &mut unresolved_gates);
    check_enterprise_boundary(&declaration, &mut diagnostics, &mut unresolved_gates);

    let status = classify(&diagnostics);
    let evidence_obligations = evidence_obligations(&declaration, status);

    crate::CheckReport {
        status,
        declaration,
        diagnostics,
        unresolved_gates,
        evidence_obligations,
    }
}

fn check_required_sections(declaration: &Declaration, diagnostics: &mut Vec<Diagnostic>) {
    for section in REQUIRED_SECTIONS {
        if !has_section(declaration, section) {
            diagnostics.push(diagnostic(
                DiagnosticCategory::MissingRequired,
                Severity::Error,
                section,
                "required declaration section is missing",
                "KILN-REQ-004",
            ));
        }
    }
}

fn check_identity(declaration: &Declaration, diagnostics: &mut Vec<Diagnostic>) {
    match declaration.kiln_version.as_deref() {
        Some(KILN_VERSION) => {}
        Some(_) => diagnostics.push(diagnostic(
            DiagnosticCategory::UnsupportedVersion,
            Severity::Error,
            "kiln.version",
            "kiln.version is not supported by this foundation checker",
            "KILN-REQ-003",
        )),
        None => {
            if has_section(declaration, "kiln") {
                diagnostics.push(diagnostic(
                    DiagnosticCategory::MissingRequired,
                    Severity::Error,
                    "kiln.version",
                    "kiln.version is required",
                    "KILN-REQ-004",
                ));
            }
        }
    }

    if has_section(declaration, "capability") && declaration.capability.is_none() {
        diagnostics.push(diagnostic(
            DiagnosticCategory::MissingRequired,
            Severity::Error,
            "capability.id",
            "capability.id is required",
            "KILN-REQ-004",
        ));
    }
}

fn check_unknown_sections(declaration: &Declaration, diagnostics: &mut Vec<Diagnostic>) {
    for section in &declaration.sections {
        if !KNOWN_SECTIONS.iter().any(|known| known == section) {
            diagnostics.push(diagnostic(
                DiagnosticCategory::UnsupportedHandoff,
                Severity::Warning,
                section,
                "unknown top-level section is not part of the v0 interface",
                "KILN-IF-001",
            ));
        }
    }
}

fn check_policy_boundary(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
    unresolved_gates: &mut Vec<String>,
) {
    if has_section(declaration, "policy_needs") {
        unresolved_gates.push("policy".to_owned());
    }

    if has_true_field(declaration, "policy_needs.authorized") {
        diagnostics.push(diagnostic(
            DiagnosticCategory::PolicyUnresolved,
            Severity::Error,
            "policy_needs.authorized",
            "KILN records policy needs but does not authorize them",
            "KILN-REQ-007",
        ));
        diagnostics.push(diagnostic(
            DiagnosticCategory::BoundaryViolation,
            Severity::Error,
            "policy_needs.authorized",
            "authorization success is outside KILN",
            "KILN-IF-005",
        ));
    }
}

fn check_package_boundary(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
    unresolved_gates: &mut Vec<String>,
) {
    if has_section(declaration, "package") {
        unresolved_gates.push("package".to_owned());
    }

    for path in ["package.published", "package.signed"] {
        if has_true_field(declaration, path) {
            diagnostics.push(diagnostic(
                DiagnosticCategory::PackageNotReady,
                Severity::Error,
                path,
                "KILN records package intent but does not publish or sign packages",
                "KILN-REQ-008",
            ));
            diagnostics.push(diagnostic(
                DiagnosticCategory::BoundaryViolation,
                Severity::Error,
                path,
                "registry trust and publication are outside KILN",
                "KILN-IF-006",
            ));
        }
    }
}

fn check_cal_boundary(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
    unresolved_gates: &mut Vec<String>,
) {
    if has_section(declaration, "cal_refs") {
        unresolved_gates.push("cal".to_owned());
    }

    if has_field(declaration, "cal_refs.behavior") {
        diagnostics.push(diagnostic(
            DiagnosticCategory::BoundaryViolation,
            Severity::Error,
            "cal_refs.behavior",
            "CAL owns primitive semantics; KILN only records identifiers",
            "KILN-REQ-009",
        ));
    }
}

fn check_runtime_boundary(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
    unresolved_gates: &mut Vec<String>,
) {
    if has_section(declaration, "runtime") {
        unresolved_gates.push("runtime".to_owned());
    }

    if has_true_field(declaration, "runtime.hides_unresolved_gates") {
        diagnostics.push(diagnostic(
            DiagnosticCategory::RuntimeNotReady,
            Severity::Error,
            "runtime.hides_unresolved_gates",
            "runtime handoffs must not hide unresolved gates",
            "KILN-REQ-006",
        ));
    }
}

fn check_enterprise_boundary(
    declaration: &Declaration,
    diagnostics: &mut Vec<Diagnostic>,
    unresolved_gates: &mut Vec<String>,
) {
    if has_section(declaration, "enterprise") {
        unresolved_gates.push("enterprise".to_owned());
        diagnostics.push(diagnostic(
            DiagnosticCategory::BoundaryViolation,
            Severity::Error,
            "enterprise",
            "enterprise-only fields are not allowed in KILN core",
            "KILN-REQ-010",
        ));
    }
}

fn classify(diagnostics: &[Diagnostic]) -> Status {
    if diagnostics
        .iter()
        .any(|item| item.severity == Severity::Error)
    {
        return Status::NotReady;
    }
    if diagnostics
        .iter()
        .any(|item| item.severity == Severity::Warning)
    {
        return Status::Degraded;
    }
    Status::Ready
}

fn evidence_obligations(declaration: &Declaration, status: Status) -> Vec<String> {
    let mut obligations = vec!["KILN-EVID-VER-004".to_owned()];
    if status != Status::Ready || !has_section(declaration, "review") {
        obligations.push("KILN-EVID-VAL-010".to_owned());
    }
    obligations
}

fn has_section(declaration: &Declaration, section: &str) -> bool {
    declaration.sections.iter().any(|item| item == section)
}

fn has_field(declaration: &Declaration, path: &str) -> bool {
    declaration.fields.iter().any(|field| field.path == path)
}

fn has_true_field(declaration: &Declaration, path: &str) -> bool {
    declaration
        .fields
        .iter()
        .any(|field| field.path == path && field.value.eq_ignore_ascii_case("true"))
}

fn diagnostic(
    category: DiagnosticCategory,
    severity: Severity,
    field: &str,
    message: &str,
    trace: &str,
) -> Diagnostic {
    Diagnostic {
        category,
        severity,
        field: field.to_owned(),
        message: message.to_owned(),
        trace: vec![trace.to_owned()],
    }
}
