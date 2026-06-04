#![forbid(unsafe_code)]

mod parser;

use std::fmt;

pub use parser::parse_declaration_text;

pub const KILN_VERSION: &str = "v0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Ready,
    NotReady,
    Degraded,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::NotReady => "not_ready",
            Self::Degraded => "degraded",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticCategory {
    MissingRequired,
    AmbiguousReference,
    UnsupportedVersion,
    UnsupportedHandoff,
    EvidenceMissing,
    PolicyUnresolved,
    PackageNotReady,
    RuntimeNotReady,
    BoundaryViolation,
}

impl DiagnosticCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequired => "missing_required",
            Self::AmbiguousReference => "ambiguous_reference",
            Self::UnsupportedVersion => "unsupported_version",
            Self::UnsupportedHandoff => "unsupported_handoff",
            Self::EvidenceMissing => "evidence_missing",
            Self::PolicyUnresolved => "policy_unresolved",
            Self::PackageNotReady => "package_not_ready",
            Self::RuntimeNotReady => "runtime_not_ready",
            Self::BoundaryViolation => "boundary_violation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub category: DiagnosticCategory,
    pub severity: Severity,
    pub field: String,
    pub message: String,
    pub trace: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    pub id: String,
    pub version: String,
    pub summary: Option<String>,
    pub owner: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Declaration {
    pub kiln_version: Option<String>,
    pub capability: Option<Capability>,
    pub sections: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub path: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckReport {
    pub status: Status,
    pub declaration: Declaration,
    pub diagnostics: Vec<Diagnostic>,
    pub unresolved_gates: Vec<String>,
    pub evidence_obligations: Vec<String>,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_declaration_text, DiagnosticCategory, Status, KILN_VERSION};

    #[test]
    fn exposes_v0_foundation_version() {
        assert_eq!(KILN_VERSION, "v0");
    }

    #[test]
    fn status_strings_are_stable() {
        assert_eq!(Status::Ready.as_str(), "ready");
        assert_eq!(Status::NotReady.as_str(), "not_ready");
        assert_eq!(Status::Degraded.as_str(), "degraded");
    }

    #[test]
    fn diagnostic_categories_match_interface() {
        let categories = [
            DiagnosticCategory::MissingRequired,
            DiagnosticCategory::AmbiguousReference,
            DiagnosticCategory::UnsupportedVersion,
            DiagnosticCategory::UnsupportedHandoff,
            DiagnosticCategory::EvidenceMissing,
            DiagnosticCategory::PolicyUnresolved,
            DiagnosticCategory::PackageNotReady,
            DiagnosticCategory::RuntimeNotReady,
            DiagnosticCategory::BoundaryViolation,
        ];

        assert_eq!(categories.len(), 9);
        assert_eq!(categories[0].as_str(), "missing_required");
        assert_eq!(categories[8].as_str(), "boundary_violation");
    }

    #[test]
    fn required_fixture_matrix_is_retained() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("fixtures");
        let expected = [
            "valid",
            "missing-kiln",
            "unsupported-version",
            "missing-capability",
            "unknown-section",
            "policy-authorized",
            "package-published",
            "cal-semantics",
            "runtime-hidden-gates",
            "enterprise-required",
        ];

        for fixture in expected {
            let path = root.join(fixture).join("kiln.yaml");
            assert!(
                path.is_file(),
                "missing retained fixture {}",
                path.display()
            );
        }
    }

    #[test]
    fn parses_valid_fixture_identity() {
        let text = fixture_text("valid");
        let declaration = parse_declaration_text(&text);
        let capability = declaration.capability.as_ref().expect("capability");

        assert_eq!(declaration.kiln_version.as_deref(), Some("v0"));
        assert_eq!(capability.id, "example.echo");
        assert!(declaration
            .sections
            .iter()
            .any(|section| section == "checks"));
    }

    #[test]
    fn missing_kiln_does_not_default_version() {
        let text = fixture_text("missing-kiln");
        let declaration = parse_declaration_text(&text);

        assert_eq!(declaration.kiln_version, None);
        assert!(declaration.capability.is_some());
    }

    #[test]
    fn retains_boundary_fields_for_checker() {
        let text = fixture_text("policy-authorized");
        let declaration = parse_declaration_text(&text);

        assert!(declaration
            .fields
            .iter()
            .any(|field| field.path == "policy_needs.authorized" && field.value == "true"));
    }

    fn fixture_text(name: &str) -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("fixtures")
            .join(name)
            .join("kiln.yaml");
        std::fs::read_to_string(path).expect("fixture text")
    }
}
