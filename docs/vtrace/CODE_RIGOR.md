# KILN Code Rigor

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Code Rigor |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/ARCHITECTURE.md`, `docs/vtrace/INTERFACES.md`, `docs/vtrace/DESIGN.md` |
| Risk level | high |
| Language/toolchain | Rust target; Cargo validation; dependency posture reviewed before implementation |
| Stage status | Role-reviewed fixed point for Code Rigor; implementation remains blocked until Implementation Plan and Work Packages authorize code. |

KILN is high rigor because it is a build/check/package trust boundary. A bug can
make a managed-agent capability appear ready, authorized, packaged, or safe when
it is not. These constraints must be satisfied by implementation work packages
before code is accepted.

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| KILN-CR-001 | Functions should stay under 60 logical lines, and critical parser/checker/diagnostic functions should target 40 lines. | Hand-authored Rust code | code review / size inspection | Larger units require written rationale in the work package. |
| KILN-CR-002 | Complex control flow must be decomposed into named rule functions or state-transition helpers. | Parser, checker, classifier, emitter | code review / tests | Complexity is allowed only with matching design rationale and fixtures. |
| KILN-CR-003 | Public CLI and library interfaces must handle invalid inputs explicitly. | CLI, parser, schema/record APIs | negative fixture tests | Impossible states require documented invariant and review. |
| KILN-CR-004 | No silent defaults may convert missing, ambiguous, unsupported, or unproven input into ready status. | Parser, checker, classifier | negative fixture tests | No waiver for readiness-affecting data. |
| KILN-CR-005 | Every readiness or boundary rule must map to a `KILN-REQ-*`, `KILN-SPEC-*`, or `KILN-IF-*` ID. | Check engine and diagnostics | trace/code review | Temporary discovery rules must be marked experimental and not readiness-bearing. |
| KILN-CR-006 | Diagnostics must use stable categories from `KILN-IF-004` or a reviewed addition. | Diagnostic model and CLI output | fixture tests / review | New categories require interface update or work-package waiver. |
| KILN-CR-007 | Exit code mapping must be explicit and tested. | CLI | CLI fixture tests | No broad catch may exit with success. |
| KILN-CR-008 | Build records must include required fields or fail visibly. | Build record emitter | fixture snapshot inspection | Partial records for `not_ready` remain disallowed until a work package resolves the open question. |
| KILN-CR-009 | The first implementation slice must be local-file-only and side-effect-free except an explicitly requested output file. | CLI, file IO | tests / code review | Any additional write, network, provider, process, registry, or runtime effect is prohibited. |
| KILN-CR-010 | KILN core must not depend on enterprise-only repos, provider SDKs, runtime hosts, registry services, policy engines, or private product surfaces. | Cargo dependencies and code imports | dependency inspection | No waiver in public core. Adapter exceptions require separate boundary review. |
| KILN-CR-011 | Non-std dependencies require explicit work-package rationale and security/supply-chain review. | Cargo dependencies | dependency review | Test-only dependencies may be approved if scoped and justified. |
| KILN-CR-012 | `unsafe` is prohibited in the foundation slice. | Rust code | code search / review | No waiver in first implementation slice. |
| KILN-CR-013 | `panic!`, `unwrap`, and `expect` are prohibited in user-input, parser, checker, CLI, and emitter paths. | Critical Rust code | code search / review | Tests may use `unwrap`/`expect`; production exceptions require rationale and non-user input proof. |
| KILN-CR-014 | Formatting, tests, and lints must be clean or explicitly waived. | Whole implementation scope | `cargo fmt --check`, `cargo test --workspace`, future lint command | Waivers need owner, rationale, and revisit trigger. |
| KILN-CR-015 | Fixture coverage must include valid, missing-field, degraded, policy-boundary, package-boundary, CAL-boundary, runtime-boundary, and enterprise-dependency scenarios before claiming readiness. | Tests and fixtures | fixture inventory / tests | Scope reductions require work-package decision and retained risk. |

## Tailoring

| Area | Rule | Rationale |
|---|---|---|
| Parser | Prefer constrained fixture parsing for the first slice unless a YAML dependency is approved by work package. | Avoids pulling supply-chain risk before the record shape stabilizes. |
| Check engine | Implement each readiness/boundary rule as a small named rule. | Makes traceability and review practical. |
| Diagnostics | Treat diagnostics as data, not formatted strings only. | JSON build records and downstream tools need stable categories. |
| Build record | Emit JSON as the stable machine output; text output is display-only during `v0`. | Avoids early compatibility burden for human prose. |
| File IO | Read only the named declaration file; write only when `--out` is explicitly requested. | Preserves side-effect-free claim. |
| CLI | Keep first command family to `kiln check`; defer broader commands. | Prevents scope creep before foundation proof. |
| Dependencies | Start std-only if feasible; otherwise justify exact parser/serialization dependencies. | Build tools are supply-chain sensitive. |
| Tests | Use retained fixtures rather than network or generated external data. | Keeps validation reproducible and local. |

## Exceptions / Waivers

| ID | Constraint | Exception | Rationale | Owner | Revisit Trigger |
|---|---|---|---|---|---|
| KILN-WAIVER-001 | KILN-CR-011 | Parser/serialization dependency may be proposed. | YAML/JSON correctness may be safer with mature public crates than hand parsing. | KILN | Before first implementation work package adds dependencies. |
| KILN-WAIVER-002 | KILN-CR-008 | resolved: `not_ready` emits diagnostics-only JSON, not a partial build record. | Missing identity/version cannot satisfy required build-record fields. | KILN | Reopen only if a later interface version adds an explicit partial-record schema. |

No waiver is currently accepted. The table records possible future waiver topics
that require explicit work-package approval.

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| KILN-EVID-CR-001 | KILN-CR-001; KILN-CR-002; KILN-CR-005 | Code review against work package | pending | future work package review |
| KILN-EVID-CR-002 | KILN-CR-003; KILN-CR-004; KILN-CR-006; KILN-CR-007; KILN-CR-015 | `cargo test --workspace` plus fixture inventory | pending | future verification evidence |
| KILN-EVID-CR-003 | KILN-CR-009; KILN-CR-010; KILN-CR-011; KILN-CR-012; KILN-CR-013 | code/dependency search and review | pending | future verification evidence |
| KILN-EVID-CR-004 | KILN-CR-014 | `cargo fmt --check`; future lint command if added by work package | pending | future validation output |
| KILN-EVID-CR-005 | KILN-CR-008 | build-record fixture snapshot review | pending | future fixture evidence |

## Required First-Slice Fixture Matrix

| Fixture ID | Purpose | Required Diagnostics / Status |
|---|---|---|
| KILN-FIX-001 | Minimal valid declaration. | `ready`; no blocking diagnostics. |
| KILN-FIX-002 | Missing `kiln` section. | `not_ready`; `missing_required`. |
| KILN-FIX-003 | Unsupported `kiln.version`. | `not_ready`; `unsupported_version`. |
| KILN-FIX-004 | Missing capability identity. | `not_ready`; `missing_required`. |
| KILN-FIX-005 | Unknown noncritical top-level section. | `degraded` or `not_ready` per work-package rule; explicit diagnostic. |
| KILN-FIX-006 | Policy section implies authorization. | `not_ready`; boundary/policy diagnostic; no authorization success. |
| KILN-FIX-007 | Package section claims publication or signing success. | `not_ready`; package boundary diagnostic; no registry mutation. |
| KILN-FIX-008 | CAL reference defines primitive semantics. | `not_ready`; boundary diagnostic. |
| KILN-FIX-009 | Runtime handoff hides unresolved gates. | `not_ready`; runtime readiness diagnostic. |
| KILN-FIX-010 | Enterprise-only field required in core declaration. | `not_ready`; boundary/dependency diagnostic. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Code rigor must protect KILN's trust-boundary purpose without blocking the first useful slice. | Addressed by small-rule, fixture-first constraints. |
| Requirements Traceability Auditor | Code constraints must trace to requirements, specs, and interfaces. | Addressed by rule-to-ID mapping requirements and fixture matrix. |
| Verification and Validation Lead | Code rigor needs objective commands/reviews, not general quality language. | Addressed by verification evidence table and fixture matrix. |
| Software Assurance Guardian | Parser, checker, diagnostics, emitter, CLI, dependencies, and file IO need explicit constraints. | Addressed by KILN-CR-001 through KILN-CR-015. |
| Security Privacy Guardian | Code rigor must prevent hidden effects, authority creep, registry trust, and supply-chain risk. | Addressed by side-effect, dependency, unsafe, and boundary constraints. |
| Source Custody Counsel | Dependency and adjacent-system references must avoid endorsement or private coupling. | Addressed by public-core dependency constraints. |
| Repo Maintainer | Constraints must be practical for a small first implementation. | Addressed by fixture-first scope and limited `kiln check` command. |
| Future Agent | Constraints must be searchable and reusable in work packages. | Addressed by stable `KILN-CR-*`, `KILN-EVID-*`, and `KILN-FIX-*` IDs. |

## Code Rigor Gate

Decision: pass_with_risk.

Rationale: Code rigor is specific enough to proceed to Implementation Planning.
The accepted risks are unresolved dependency choice and the `not_ready` build
record question. Both must be resolved or explicitly waived in work packages
before implementation touches parser or serializer code. The build-record emitter
question is resolved for the foundation slice by using diagnostics-only JSON for
`not_ready` cases.

## Source Links

- `docs/vtrace/DESIGN.md`
- `docs/vtrace/INTERFACES.md`
- `docs/vtrace/ARCHITECTURE.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/code-rigor.md`
- VTRACE `docs/framework/vtrace-process.md`
