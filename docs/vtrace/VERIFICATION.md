# KILN Verification Plan

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Verification |
| Parent stages | `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/INTERFACES.md`, `docs/vtrace/DESIGN.md`, `docs/vtrace/CODE_RIGOR.md`, `docs/vtrace/WORK_PACKAGES.md` |
| Stage status | Role-reviewed fixed point |

This plan defines how KILN verifies the first fixture-backed foundation checker.
KILN-WP-002 through KILN-WP-007 have now produced a std-only workspace,
retained fixtures, parser, checker, emitter, and CLI. KILN-WP-008 records the
executed evidence below.

## Executed Evidence Summary

| Evidence | Result |
|---|---|
| Input SHA | `d8e91ed` |
| Output SHA | this commit |
| `cargo fmt --check` | pass |
| `cargo test --workspace` | pass; 11 unit tests plus doc tests |
| CLI fixture exit codes | pass; `valid=0`, all nine non-ready/degraded fixtures exit `1` |
| Build record output | pass; `target\kiln\valid.build.json` produced from explicit `--out` |
| Code rigor search | pass; `unsafe` forbidden; `expect` appears only in tests |
| `git diff --check` | pass |

## Verification Method Legend

| Method | Meaning for KILN |
|---|---|
| inspection | Review controlled files, crate manifests, dependency lists, records, or docs. |
| review | Apply repo-local `.roles` review and record findings. |
| analysis | Compare states, transitions, side effects, or boundary ownership without executing downstream systems. |
| test | Execute retained fixtures or unit/integration tests after implementation exists. |
| demonstration | Show a consumer-facing scenario with fixture data and no downstream mutation. |
| trace review | Inspect `TRACE.md` rows for complete V coverage. |

## Requirement Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| KILN-REQ-001 | inspection / review | Review VTRACE docs, interfaces, build-record target | Public build/check/package contract is described and bounded. | pending | KILN-EVID-VER-001 |
| KILN-REQ-002 | inspection / dependency review | Inspect Cargo manifests and imports once workspace exists | KILN core has no enterprise-only dependency. | pending | KILN-EVID-VER-002 |
| KILN-REQ-003 | test / inspection | Run side-effect-free fixture tests and review file IO | Checker performs no provider, network, product write, runtime, registry, package publication, or policy authorization effects. | pending | KILN-EVID-VER-003 |
| KILN-REQ-004 | test | Run negative declaration fixtures | Missing or ambiguous data emits explicit diagnostics. | pending | KILN-EVID-VER-004 |
| KILN-REQ-005 | test / analysis | Run status fixture matrix | `ready`, `not_ready`, and `degraded` are deterministic and documented. | pending | KILN-EVID-VER-005 |
| KILN-REQ-006 | test / inspection | Inspect emitted `kiln.build.json` fixtures | Build records state checked inputs, unresolved gates, evidence obligations, and handoffs. | pending | KILN-EVID-VER-006 |
| KILN-REQ-007 | inspection / test | Run policy-boundary negative fixture | KILN records policy needs but does not emit authorization success. | pending | KILN-EVID-VER-007 |
| KILN-REQ-008 | inspection / test | Run package-boundary negative fixture | KILN records package intent but does not publish, sign, distribute, or mutate registry state. | pending | KILN-EVID-VER-008 |
| KILN-REQ-009 | inspection / review | Run CAL-boundary fixture and review semantics | KILN references CAL identifiers without defining primitive semantics. | pending | KILN-EVID-VER-009 |
| KILN-REQ-010 | inspection / demonstration | Inspect dependency graph and adapter fixture | Enterprise review consumes public evidence without adding enterprise-only core fields or dependencies. | pending | KILN-EVID-VER-010 |
| KILN-REQ-011 | inspection / review | Review diagnostics, build records, and VTRACE gaps | Missing evidence is explicit as diagnostic, unresolved gate, gap, deferred item, degraded status, or not-ready status. | pending | KILN-EVID-VER-011 |
| KILN-REQ-012 | trace review | Review `TRACE.md` | Needs, CONOPS, requirements, specs, interfaces, work packages, verification, validation, and evidence are connected. | pending | KILN-EVID-VER-012 |

## Specification Verification Matrix

| Spec ID | Verification ID | Method | Command / Inspection | Expected Evidence | Result |
|---|---|---|---|---|---|
| KILN-SPEC-001 | VER-KILN-001 | inspection / review | Inspect VTRACE source-of-truth docs | Public Standards & Protocols position and product-neutral contract remain clear. | pending |
| KILN-SPEC-002 | VER-KILN-002 | test / inspection | Side-effect fixture tests plus source review | Foundation operation remains side-effect-free. | pending |
| KILN-SPEC-003 | VER-KILN-003 | test / analysis | Status fixture tests | Declared capability record checks produce deterministic readiness. | pending |
| KILN-SPEC-004 | VER-KILN-004 | test / inspection | Negative diagnostics fixtures | Missing, ambiguous, unsupported, incompatible, or unproven conditions are visible. | pending |
| KILN-SPEC-005 | VER-KILN-005 | test / inspection | Build-record fixture snapshot | Checked inputs, unresolved gates, handoffs, and obligations are present. | pending |
| KILN-SPEC-006 | VER-KILN-006 | test / inspection | Policy negative fixture | No KILN-owned authorization decision exists. | pending |
| KILN-SPEC-007 | VER-KILN-007 | test / inspection | Package negative fixture | No publication, signing, distribution, or registry mutation exists. | pending |
| KILN-SPEC-008 | VER-KILN-008 | inspection / review | CAL boundary fixture and role review | CAL semantics are not defined by KILN. | pending |
| KILN-SPEC-009 | VER-KILN-009 | demonstration / inspection | Enterprise display adapter fixture or dependency review | Enterprise consumption stays outside public KILN core. | pending |
| KILN-SPEC-010 | VER-KILN-010 | trace review | Inspect `TRACE.md` | Stable VTRACE IDs connect through evidence. | pending |
| KILN-SPEC-011 | VER-KILN-011 | test / inspection | Fixture inventory and tests | First slice includes valid, missing-field, degraded, and boundary fixtures. | pending |
| KILN-SPEC-012 | VER-KILN-012 | dependency review | Inspect Cargo manifests, lockfile, and imports | No enterprise-only, provider SDK, runtime host, registry service, or private product dependency in core. | pending |

## Interface Verification Matrix

| Interface ID | Method | Command / Inspection | Expected Evidence | Result |
|---|---|---|---|---|
| KILN-IF-001 | schema / fixture inspection | Inspect `kiln.yaml` fixtures | Required sections, versioning, and unsupported-section diagnostics are covered. | pending |
| KILN-IF-002 | CLI fixture tests | Run `kiln check` fixture commands | Exit codes, `--format`, and explicit `--out` behavior match interface. | pending |
| KILN-IF-003 | record inspection | Inspect emitted `kiln.build.json` snapshots | Required fields are present or record emission fails visibly. | pending |
| KILN-IF-004 | negative fixture tests | Run diagnostics fixtures | Stable diagnostic categories are emitted. | pending |
| KILN-IF-005 | policy negative fixture | Inspect policy handoff output | Policy needs are unresolved; no authorization success. | pending |
| KILN-IF-006 | package negative fixture | Inspect package handoff output | Package intent is recorded; no publish/sign/registry mutation. | pending |
| KILN-IF-007 | boundary review | Inspect CAL reference fixture | KILN records IDs/version expectations only. | pending |
| KILN-IF-008 | mock-runtime fixture | Inspect runtime handoff output | Runtime receives checked/gated status; no execution. | pending |
| KILN-IF-009 | dependency inspection | Inspect core dependencies and adapter evidence | Enterprise-only fields/deps are absent from core. | pending |
| KILN-IF-010 | evidence fixture review | Inspect evidence manifest or retained inventory | Evidence is inspectable without KILN owning conformance tooling. | pending |

## Fixture Verification Matrix

| Fixture ID | Primary Coverage | Expected Status / Diagnostic | Result | Evidence Pointer |
|---|---|---|---|---|
| KILN-FIX-001 | valid declaration | `ready`; no blocking diagnostics | pending | future fixture |
| KILN-FIX-002 | missing `kiln` section | `not_ready`; `missing_required` | pending | future fixture |
| KILN-FIX-003 | unsupported `kiln.version` | `not_ready`; `unsupported_version` | pending | future fixture |
| KILN-FIX-004 | missing capability identity | `not_ready`; `missing_required` | pending | future fixture |
| KILN-FIX-005 | unknown top-level section | `degraded` or `not_ready`; explicit diagnostic | pending | future fixture |
| KILN-FIX-006 | policy implies authorization | `not_ready`; policy/boundary diagnostic; no authorization success | pending | future fixture |
| KILN-FIX-007 | package claims publication/signing | `not_ready`; package boundary diagnostic; no registry mutation | pending | future fixture |
| KILN-FIX-008 | CAL primitive semantics in KILN | `not_ready`; boundary diagnostic | pending | future fixture |
| KILN-FIX-009 | runtime hides unresolved gates | `not_ready`; runtime readiness diagnostic | pending | future fixture |
| KILN-FIX-010 | enterprise-only field required | `not_ready`; boundary/dependency diagnostic | pending | future fixture |

## Commands

### Current pre-implementation commands

KILN is not yet initialized as a Git repository, so Git-based verification is a
pending process gap. Until repo initialization, use file hygiene inspection for
new VTRACE docs and record that SHA-based closure is unavailable.

```powershell
$p='C:\src\kiln\docs\vtrace\VERIFICATION.md'
$lines=Get-Content -LiteralPath $p
$trailing=@()
for($i=0;$i -lt $lines.Count;$i++){ if($lines[$i] -match '\s+$'){ $trailing += ($i+1) } }
$conflicts=Select-String -LiteralPath $p -Pattern '^(<<<<<<<|=======|>>>>>>>)'
if($trailing.Count -eq 0 -and -not $conflicts){ 'OK' }
```

### Required implementation commands

These commands become mandatory after the matching work packages create the
workspace, fixtures, parser, checker, emitter, and CLI.

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
cargo run -q -p kiln-cli -- check fixtures\missing-kiln\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\policy-authorized\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\package-published\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\cal-semantics\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\runtime-hidden-gates\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\enterprise-required\kiln.yaml --format json
git diff --check
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast local sanity for the active work package. | Targeted fixture tests or doc hygiene check; `cargo fmt --check` after workspace exists. | pending |
| L1 | Full repo confidence before child repo push or PR. | `cargo test --workspace`; full fixture matrix; dependency inspection; `git diff --check`. | pending |
| L2 | Integration/readiness proof before release, public claim, or downstream adoption. | Record inspection, boundary role review, trace closure, and mock downstream evidence where applicable. | pending |

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| KILN-EVID-VER-001 | review | `docs/vtrace/*`; role ledger | public contract and source-of-truth posture | pending |
| KILN-EVID-VER-002 | dependency review | future Cargo manifests, lockfile, imports | public dependency posture | pending |
| KILN-EVID-VER-003 | test / inspection | side-effect fixture tests and source review | side-effect-free operation | pending |
| KILN-EVID-VER-004 | test | negative diagnostics fixtures | explicit diagnostics | pending |
| KILN-EVID-VER-005 | test / analysis | status fixture matrix | readiness state classification | pending |
| KILN-EVID-VER-006 | test / inspection | build-record snapshots | checked inputs, unresolved gates, obligations, handoffs | pending |
| KILN-EVID-VER-007 | test / inspection | policy negative fixture | no authorization ownership | pending |
| KILN-EVID-VER-008 | test / inspection | package negative fixture | no registry/package trust mutation | pending |
| KILN-EVID-VER-009 | review / fixture | CAL boundary fixture | no CAL semantic ownership | pending |
| KILN-EVID-VER-010 | demonstration / inspection | enterprise adapter fixture or dependency review | public core remains enterprise-neutral | pending |
| KILN-EVID-VER-011 | inspection / review | diagnostics, records, VTRACE gaps | missing evidence remains visible | pending |
| KILN-EVID-VER-012 | trace review | `docs/vtrace/TRACE.md` | full V continuity | pending |
| KILN-EVID-VER-013 | doc hygiene | pre-implementation file hygiene command | no whitespace/conflict-marker defect in this plan | pending |

## Code Rigor Verification

| Constraint ID | Method | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| KILN-CR-001 | size/complexity inspection | Review critical functions against 40/60-line targets. | pending | KILN-EVID-CR-001 |
| KILN-CR-002 | design/code review | Confirm complex flow is split into named rules/helpers. | pending | KILN-EVID-CR-001 |
| KILN-CR-003 | negative tests | Invalid input fixtures and CLI tests. | pending | KILN-EVID-CR-002 |
| KILN-CR-004 | negative tests | Missing/ambiguous/unsupported data cannot become ready. | pending | KILN-EVID-CR-002 |
| KILN-CR-005 | trace/code review | Readiness/boundary rules map to `KILN-*` IDs. | pending | KILN-EVID-CR-001 |
| KILN-CR-006 | fixture tests / review | Diagnostics use `KILN-IF-004` categories or reviewed additions. | pending | KILN-EVID-CR-002 |
| KILN-CR-007 | CLI tests | Exit code fixtures for ready, degraded/not-ready, usage/input, internal error classes. | pending | KILN-EVID-CR-002 |
| KILN-CR-008 | snapshot inspection | Build-record snapshots include required fields or fail visibly. | pending | KILN-EVID-CR-005 |
| KILN-CR-009 | tests / review | Side-effect review and explicit `--out` write checks. | pending | KILN-EVID-CR-003 |
| KILN-CR-010 | dependency inspection | Inspect manifests/imports for enterprise/provider/runtime/registry/policy deps. | pending | KILN-EVID-CR-003 |
| KILN-CR-011 | supply-chain review | Review any non-std dependency rationale. | pending | KILN-EVID-CR-003 |
| KILN-CR-012 | code search / review | Search for `unsafe`. | pending | KILN-EVID-CR-003 |
| KILN-CR-013 | code search / review | Search production paths for `panic!`, `unwrap`, and `expect`. | pending | KILN-EVID-CR-003 |
| KILN-CR-014 | tool commands | `cargo fmt --check`; `cargo test --workspace`; future lint if added. | pending | KILN-EVID-CR-004 |
| KILN-CR-015 | fixture inventory / tests | Verify required fixture matrix exists and passes. | pending | KILN-EVID-CR-002 |

## Work Package Verification Responsibilities

| Work Package | Required Verification Closure |
|---|---|
| KILN-WP-001 | Docs no longer claim unapproved implementation behavior; doc hygiene evidence retained. |
| KILN-WP-002 | Workspace validates; dependency decision and supply-chain review recorded. |
| KILN-WP-003 | Fixture matrix exists and maps to requirements/specs/interfaces. |
| KILN-WP-004 | Parser and normalization pass valid and malformed fixture tests. |
| KILN-WP-005 | Rule/status/diagnostic tests pass and map to stable IDs. |
| KILN-WP-006 | Build-record snapshots include required fields and unresolved gates. |
| KILN-WP-007 | CLI fixture commands and exit-code behavior pass; side effects remain bounded. |
| KILN-WP-008 | This verification plan is updated with actual evidence; Validation, Trace, and Review close. |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| KILN Git repository was initialized during implementation. | SHA-based closure is now available locally; remote/submodule snapshot remains future portfolio work. | Resolved locally; push and TRACKER submodule update still required for portfolio snapshot. |
| Rust workspace exists and validates. | Cargo verification commands run successfully. | Resolved in KILN-WP-002. |
| Fixture matrix exists and validates. | Fixture-based requirement/spec/interface verification executes. | Resolved in KILN-WP-003. |
| Parser dependency decision is resolved std-only. | Dependency/supply-chain verification can close for foundation slice. | Resolved in KILN-WP-002 and `docs/vtrace/DEPENDENCY_DECISION.md`. |
| `not_ready` partial build-record behavior is resolved. | `not_ready` emits diagnostics-only JSON, not a partial build record. | Resolved in KILN-WP-006. |
| Downstream repos CAL/WARDEN/DEPOT/GAUGE may not exist yet. | L2 integration evidence must remain fixture/mock/boundary-based. | Accept for first slice; record downstream integrations as later work. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Verification must prove KILN's build/check contract without turning verification into implementation. | Addressed by planned evidence and pending results. |
| Requirements Traceability Auditor | Every accepted requirement/spec/interface/code-rigor item needs a verification path. | Addressed by matrices and work-package responsibilities. |
| Verification and Validation Lead | Commands must distinguish current doc-stage checks from future implementation checks. | Addressed by current and required command sections. |
| Software Assurance Guardian | Code rigor must be objectively checkable before code is accepted. | Addressed by Code Rigor Verification table. |
| Security Privacy Guardian | Side-effect freedom, policy, package, and dependency posture need negative evidence. | Addressed by negative fixtures, side-effect review, and dependency review. |
| Source Custody Counsel | Verification must not require private or enterprise systems for public core proof. | Addressed by boundary/mock L2 evidence and dependency checks. |
| Repo Maintainer | Verification must be executable package-by-package. | Addressed by work-package verification responsibilities. |
| Future Agent | Verification needs stable evidence IDs and resumption notes. | Addressed by `KILN-EVID-VER-*` IDs and gaps table. |

## Verification Gate

Decision: pass_with_risk.

Rationale: The verification plan is specific enough to proceed to Validation,
Trace, and Review planning. No implementation is authorized by this document.
Accepted risks are limited to absent repository initialization, absent code,
absent fixtures, unresolved dependency choice, and unresolved not-ready build
record behavior. Each risk has a named work-package or later proof-stage
disposition.

Stage ledger:

| Repo | Stage | File | Status | Input SHA | Output SHA | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|---|---|---|
| KILN | VERIFICATION | `docs/vtrace/VERIFICATION.md` | settled | pending repo init | pending repo init | KILN `.roles` simulated fixed-point review | No unresolved critical/major findings; current evidence gaps recorded | pass_with_risk | VALIDATION |

## Source Links

- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `docs/vtrace/INTERFACES.md`
- `docs/vtrace/DESIGN.md`
- `docs/vtrace/CODE_RIGOR.md`
- `docs/vtrace/WORK_PACKAGES.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
