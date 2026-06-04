# KILN Fixture Inventory

## Scope

| Field | Value |
|---|---|
| Work package | KILN-WP-003 |
| Fixture root | `fixtures\` |
| Status | accepted for foundation slice |

The foundation fixture matrix is retained in source control. Each fixture uses a
local `kiln.yaml` declaration and must remain side-effect-free.

## Matrix

| Fixture ID | Directory | Purpose | Expected Status / Diagnostic |
|---|---|---|---|
| KILN-FIX-001 | `fixtures\valid\kiln.yaml` | Minimal valid declaration. | `ready`; no blocking diagnostics. |
| KILN-FIX-002 | `fixtures\missing-kiln\kiln.yaml` | Missing `kiln` section. | `not_ready`; `missing_required`. |
| KILN-FIX-003 | `fixtures\unsupported-version\kiln.yaml` | Unsupported `kiln.version`. | `not_ready`; `unsupported_version`. |
| KILN-FIX-004 | `fixtures\missing-capability\kiln.yaml` | Missing capability identity. | `not_ready`; `missing_required`. |
| KILN-FIX-005 | `fixtures\unknown-section\kiln.yaml` | Unknown noncritical top-level section. | `degraded`; `unsupported_handoff`. |
| KILN-FIX-006 | `fixtures\policy-authorized\kiln.yaml` | Policy section implies authorization. | `not_ready`; `policy_unresolved` and `boundary_violation`. |
| KILN-FIX-007 | `fixtures\package-published\kiln.yaml` | Package section claims publication/signing success. | `not_ready`; `package_not_ready` and `boundary_violation`. |
| KILN-FIX-008 | `fixtures\cal-semantics\kiln.yaml` | CAL reference defines primitive semantics. | `not_ready`; `boundary_violation`. |
| KILN-FIX-009 | `fixtures\runtime-hidden-gates\kiln.yaml` | Runtime hides unresolved gates. | `not_ready`; `runtime_not_ready`. |
| KILN-FIX-010 | `fixtures\enterprise-required\kiln.yaml` | Enterprise-only field required in core declaration. | `not_ready`; `boundary_violation`. |

## Notes

- `not_ready` fixtures may emit diagnostics-only JSON if required identity or
  version data is missing.
- Full build records require known `kiln.version` and capability identity.
- The fixture names intentionally match the CLI commands in
  `docs\vtrace\REVIEW.md`.
