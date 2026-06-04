# KILN Work Packages

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Work Packages |
| Parent stage | `docs/vtrace/IMPLEMENTATION_PLAN.md` |
| Stage status | Role-reviewed fixed point |

These work packages define the controlled implementation slices for KILN's
foundation checker. They do not authorize code until Verification, Validation,
Trace, and Review fixed points exist and the user explicitly starts
implementation.

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|---|---|
| KILN-WP-001 | Reconcile pre-VTRACE docs with the accepted VTRACE baseline. | KILN-DES-010; KILN-UNK-005; KILN-RISK-001 | README, PRODUCT_PLAN, wave/pulse docs | VTRACE baseline through Implementation Plan accepted. | No doc claims crates/CLI/fixtures are accepted behavior before work packages. | L0: `git diff --check`; L1: role review; L2: n/a | complete |
| KILN-WP-002 | Establish Rust workspace shape and dependency decision. | KILN-CR-010; KILN-CR-011; KILN-WAIVER-001 | Cargo workspace, dependency notes | WP-001 complete; dependency decision recorded. | Workspace validates; dependency posture has review evidence. | L0: fmt/check; L1: tests/dependency inspection; L2: n/a | complete |
| KILN-WP-003 | Create declaration model and retained fixture matrix. | KILN-IF-001; KILN-FIX-001..KILN-FIX-010 | core model, fixtures | WP-002 complete; fixture schema target accepted. | All required fixtures exist and are documented. | L0: fixture inventory; L1: tests; L2: trace review | complete |
| KILN-WP-004 | Implement parser and normalization. | KILN-DES-002; KILN-DES-003; KILN-CR-001..KILN-CR-004 | parser/normalizer | WP-003 complete; parser dependency decided. | Valid/malformed fixtures parse or fail explicitly. | L0: parser tests; L1: workspace tests; L2: n/a | complete |
| KILN-WP-005 | Implement check rules, statuses, and diagnostics. | KILN-REQ-003..KILN-REQ-005; KILN-IF-004; KILN-CR-005..KILN-CR-007 | checker, classifier, diagnostics | WP-004 complete. | Required diagnostics/statuses match fixture matrix. | L0: rule tests; L1: workspace tests; L2: review | proposed |
| KILN-WP-006 | Implement JSON build-record emitter. | KILN-REQ-006; KILN-IF-003; KILN-WAIVER-002 | build record emitter | WP-005 complete; `not_ready` record behavior decided. | JSON build records include required fields or fail visibly. | L0: snapshot tests; L1: workspace tests; L2: record inspection | proposed |
| KILN-WP-007 | Integrate `kiln check` CLI. | KILN-IF-002; KILN-CR-007; KILN-CR-009 | CLI, exit codes, file IO | WP-006 complete. | CLI validates fixtures, maps exit codes, and remains side-effect-free. | L0: CLI fixtures; L1: workspace tests; L2: side-effect review | proposed |
| KILN-WP-008 | Close verification, validation, trace, and review evidence. | KILN-REQ-011; KILN-REQ-012; KILN-SPEC-010 | VTRACE proof package | WP-001..WP-007 complete. | Verification, Validation, Trace, and Review record evidence and decision. | L0: diff check; L1: full verification; L2: readiness review | proposed |

## Work Package Details

### KILN-WP-001: Documentation baseline reconciliation

Objective: Update pre-VTRACE README/Product Plan/wave/pulse docs so they point
to the VTRACE baseline and do not imply unapproved implementation behavior.

Parent requirement IDs: KILN-REQ-011, KILN-REQ-012.

Parent specification IDs: KILN-SPEC-001, KILN-SPEC-010.

Boundary/package IDs: KILN-ARCH-001.

Design/interface/code-rigor IDs: KILN-DES-010.

Validation scenario IDs: KILN-CONOPS-001, KILN-CONOPS-006.

Affected files/modules:

- `README.md`
- `PRODUCT_PLAN.md`
- `context/waves/PHASES.md`
- `context/waves/2026-06-03-foundation/WAVE.md`
- `context/waves/2026-06-03-foundation/pulses/pulse-01.md`

Entry criteria:

- Specification Baseline through Implementation Plan are accepted.
- This work package is selected for implementation.

Exit criteria:

- Docs describe implementation as planned, not already accepted.
- Planned crate/CLI/fixture commands point to VTRACE work packages.
- No code files are added or changed.

Verification commands:

```powershell
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | `git diff --check` | pass |
| L1 | yes | Role review against VTRACE baseline | pass |
| L2 | no | Not required for doc reconciliation | n/a |

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Need / CONOPS | KILN-NEED-001; KILN-CONOPS-001 | pass | Docs align with public build/check mission. |
| Requirements | KILN-REQ-011; KILN-REQ-012 | pass | Missing evidence remains explicit and traceable. |
| Specification | KILN-SPEC-001; KILN-SPEC-010 | pass | Public contract and trace continuity preserved. |
| Architecture / Interface | KILN-ARCH-001 | pass | VTRACE control package only. |
| Design / Code Rigor | KILN-DES-010 | pass | No code. |
| Implementation | docs only | pass | No code. |
| Verification | `git diff --check` | pass | Metadata-only verification. |
| Validation | maintainer review | pass | Docs no longer mislead. |
| Trace | TRACE rows | pass | Work package status and evidence updated. |
| Gate | REVIEW | pass | WP-001 authorized as first package. |

Assurance/security classification:

| Lane | Required | Reviewer / Role | Decision | Evidence / Rationale |
|---|---|---|---|---|
| Systems engineering | yes | Systems Engineering Steward | pass | Avoided source-of-truth drift. |
| Requirements traceability | yes | Requirements Traceability Auditor | pass | Preserved trace links. |
| V&V | yes | Verification and Validation Lead | pass | Removed unproven implementation claims. |
| Software assurance | no | Software Assurance Guardian | not_required | Docs only. |
| Security/privacy | no | Security Privacy Guardian | not_required | No implementation or data handling. |
| Safety/mission impact | no | Systems Engineering Steward | not_required | No high-consequence runtime impact. |
| Source custody | yes | Source Custody Counsel | pass | .NET/MSBuild analogy remains analogy. |
| Configuration/change control | yes | Future Agent | pass | Stabilized baseline docs. |

Review gate: pass required before code work packages.

Git execution:

- Branch/worktree: current KILN worktree.
- Commit plan: one docs-only child repo commit.
- Push/PR condition: diff check and role review complete.
- Agent stop condition: unexpected conflict with user-authored docs.

Wave/pulse execution:

- Active wave: `2026-06-03-foundation`.
- Pulse file: `context/waves/2026-06-03-foundation/pulses/pulse-01.md`.
- Pulse status: update during implementation.
- Pulse evidence: diff check and VTRACE doc pointers.

Boundary control:

| Boundary ID | Allowed Changes | Forbidden Changes | Integration Needed |
|---|---|---|---|
| KILN-ARCH-001 | Docs and wave text. | Code, fixtures, Cargo files. | no |

Status: complete.

Closure evidence:

| Item | Result |
|---|---|
| Input SHA | `954c05b` |
| Output SHA | this commit |
| Files reconciled | `README.md`, `PRODUCT_PLAN.md`, `context/waves/PHASES.md`, `context/waves/2026-06-03-foundation/WAVE.md`, `context/waves/2026-06-03-foundation/pulses/pulse-01.md` |
| Role review | No unresolved critical/major findings; docs now point to VTRACE and work packages instead of claiming old `status`/`fixtures\tiny` behavior. |

### KILN-WP-002: Foundation workspace and dependency decision

Objective: Establish the minimal Rust/Cargo workspace and decide whether the
foundation parser/serializer uses std-only code or reviewed public crates.

Parent requirement IDs: KILN-REQ-002, KILN-REQ-003.

Parent specification IDs: KILN-SPEC-002, KILN-SPEC-012.

Boundary/package IDs: KILN-ARCH-002, KILN-ARCH-003.

Design/interface/code-rigor IDs: KILN-DES-002, KILN-DES-008, KILN-CR-010,
KILN-CR-011, KILN-WAIVER-001.

Validation scenario IDs: KILN-CONOPS-001.

Affected files/modules:

- `Cargo.toml`
- future `crates/`
- dependency rationale note in VTRACE or work-package evidence

Entry criteria:

- KILN-WP-001 complete.
- Verification/Validation/Trace/Review plans exist.
- Dependency choice is proposed with rationale.

Exit criteria:

- Workspace exists and validates.
- No enterprise/provider/runtime/registry/policy dependencies.
- Any non-std dependency has rationale and review evidence.

Verification commands:

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | `cargo fmt --check`; `git diff --check` | pending |
| L1 | yes | `cargo test --workspace`; dependency inspection | pending |
| L2 | no | No downstream integration | n/a |

Status: complete.

Closure evidence:

| Item | Result |
|---|---|
| Input SHA | `28213a6` |
| Output SHA | this commit |
| Dependency decision | std-only foundation workspace; no non-std dependency accepted. |
| Evidence pointer | `docs/vtrace/DEPENDENCY_DECISION.md` |
| Validation | `cargo fmt --check`; `cargo test --workspace`; `git diff --check` pass. |

### KILN-WP-003: Declaration model and fixtures

Objective: Create the declaration model and retained fixture matrix for the
foundation checker.

Parent requirement IDs: KILN-REQ-001, KILN-REQ-003, KILN-REQ-004.

Parent specification IDs: KILN-SPEC-003, KILN-SPEC-004, KILN-SPEC-011.

Boundary/package IDs: KILN-ARCH-002, KILN-ARCH-011.

Design/interface/code-rigor IDs: KILN-IF-001, KILN-FIX-001..KILN-FIX-010,
KILN-CR-015.

Validation scenario IDs: KILN-CONOPS-001, KILN-CONOPS-003, KILN-CONOPS-004,
KILN-CONOPS-005, KILN-CONOPS-006.

Affected files/modules:

- future core model module
- `fixtures/`
- fixture inventory evidence

Entry criteria:

- KILN-WP-002 complete.
- Fixture paths and minimum `kiln.yaml` fields accepted.

Exit criteria:

- Required fixture matrix exists.
- Fixtures are local and retained.
- Model fields cover required sections and handoff placeholders.

Verification commands:

```powershell
cargo test --workspace
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | fixture inventory; `git diff --check` | pending |
| L1 | yes | `cargo test --workspace` | pending |
| L2 | yes | trace fixture matrix to CONOPS scenarios | pending |

Status: complete.

Closure evidence:

| Item | Result |
|---|---|
| Input SHA | `85c952c` |
| Output SHA | this commit |
| Fixture inventory | `docs/vtrace/FIXTURES.md`; `fixtures\valid` through `fixtures\enterprise-required`. |
| Model surface | `crates\kiln-core\src\lib.rs` declaration, status, diagnostics, and report types. |
| Validation | `cargo fmt --check`; `cargo test --workspace`; `git diff --check` pass. |

### KILN-WP-004: Parser and normalization

Objective: Implement local-file parser and normalization from `kiln.yaml` into
the declaration model.

Parent requirement IDs: KILN-REQ-003, KILN-REQ-004.

Parent specification IDs: KILN-SPEC-002, KILN-SPEC-003, KILN-SPEC-004.

Boundary/package IDs: KILN-ARCH-002, KILN-ARCH-003.

Design/interface/code-rigor IDs: KILN-DES-002, KILN-DES-003, KILN-CR-001,
KILN-CR-002, KILN-CR-003, KILN-CR-004, KILN-IF-001.

Validation scenario IDs: KILN-CONOPS-001.

Affected files/modules:

- future parser module
- parser tests
- parser fixtures

Entry criteria:

- KILN-WP-003 complete.
- Parser dependency decision complete.

Exit criteria:

- Valid fixture normalizes.
- Malformed/missing/unknown sections fail visibly.
- No network/provider/process/workspace mutation.

Verification commands:

```powershell
cargo test --workspace
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | targeted parser tests | pending |
| L1 | yes | `cargo test --workspace` | pending |
| L2 | no | No downstream integration | n/a |

Status: complete.

Closure evidence:

| Item | Result |
|---|---|
| Input SHA | `d9601f2` |
| Output SHA | this commit |
| Parser | `crates\kiln-core\src\parser.rs` constrained section/field parser. |
| Normalization | `parse_declaration_text` fills KILN version, capability identity, sections, and retained fields. |
| Validation | `cargo fmt --check`; `cargo test --workspace`; `git diff --check` pass. |

### KILN-WP-005: Check rules and diagnostics

Objective: Implement readiness classification and explicit diagnostics for
required fields, boundary violations, and unresolved gates.

Parent requirement IDs: KILN-REQ-003, KILN-REQ-004, KILN-REQ-005,
KILN-REQ-007, KILN-REQ-008, KILN-REQ-009.

Parent specification IDs: KILN-SPEC-003, KILN-SPEC-004, KILN-SPEC-006,
KILN-SPEC-007, KILN-SPEC-008.

Boundary/package IDs: KILN-ARCH-003, KILN-ARCH-004, KILN-ARCH-006,
KILN-ARCH-007, KILN-ARCH-008, KILN-ARCH-009.

Design/interface/code-rigor IDs: KILN-IF-004, KILN-DES-004, KILN-DES-005,
KILN-DES-007, KILN-CR-005, KILN-CR-006, KILN-CR-007.

Validation scenario IDs: KILN-CONOPS-001, KILN-CONOPS-003, KILN-CONOPS-004,
KILN-CONOPS-005.

Affected files/modules:

- future checker module
- future diagnostics module
- rule tests

Entry criteria:

- KILN-WP-004 complete.
- Diagnostic categories from KILN-IF-004 accepted.

Exit criteria:

- Fixture statuses and diagnostics match Code Rigor matrix.
- No boundary rule implies downstream integration.
- Rule functions trace to IDs.

Verification commands:

```powershell
cargo test --workspace
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | targeted rule tests | pending |
| L1 | yes | `cargo test --workspace`; rule-to-ID review | pending |
| L2 | yes | boundary scenario review | pending |

Status: proposed.

### KILN-WP-006: Build record emitter

Objective: Emit JSON build records with checked inputs, diagnostics, unresolved
gates, evidence obligations, handoffs, and trace IDs.

Parent requirement IDs: KILN-REQ-006, KILN-REQ-011, KILN-REQ-012.

Parent specification IDs: KILN-SPEC-005, KILN-SPEC-010.

Boundary/package IDs: KILN-ARCH-005, KILN-ARCH-010, KILN-ARCH-011.

Design/interface/code-rigor IDs: KILN-IF-003, KILN-DES-006, KILN-WAIVER-002,
KILN-CR-008.

Validation scenario IDs: KILN-CONOPS-002, KILN-CONOPS-006.

Affected files/modules:

- future build record module
- snapshot fixtures

Entry criteria:

- KILN-WP-005 complete.
- `not_ready` partial build-record behavior decided.

Exit criteria:

- `ready` and `degraded` build records include required fields.
- Emitter fails visibly on missing identity/version.
- Snapshot fixtures are retained.

Verification commands:

```powershell
cargo test --workspace
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | snapshot tests | pending |
| L1 | yes | `cargo test --workspace`; record inspection | pending |
| L2 | yes | mock runtime/review record inspection | pending |

Status: proposed.

### KILN-WP-007: CLI integration

Objective: Expose the foundation checker through the target `kiln check` CLI.

Parent requirement IDs: KILN-REQ-003, KILN-REQ-004, KILN-REQ-005,
KILN-REQ-006.

Parent specification IDs: KILN-SPEC-002, KILN-SPEC-003, KILN-SPEC-004,
KILN-SPEC-005.

Boundary/package IDs: KILN-ARCH-003, KILN-ARCH-005.

Design/interface/code-rigor IDs: KILN-IF-002, KILN-DES-003, KILN-CR-007,
KILN-CR-009, KILN-CR-013.

Validation scenario IDs: KILN-CONOPS-001.

Affected files/modules:

- future CLI package/module
- CLI fixture tests

Entry criteria:

- KILN-WP-006 complete.
- Exit-code contract accepted.

Exit criteria:

- CLI reads only named input file.
- CLI writes only when `--out` is explicitly requested.
- Exit codes match KILN-IF-002.
- CLI fixture commands pass.

Verification commands:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
cargo run -q -p kiln-cli -- check fixtures\missing-kiln\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\policy-authorized\kiln.yaml --format json
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | targeted CLI fixtures; `git diff --check` | pending |
| L1 | yes | full command set and workspace tests | pending |
| L2 | yes | side-effect and boundary review | pending |

Status: proposed.

### KILN-WP-008: Proof package closure

Objective: Close Verification, Validation, Trace, and Review for the foundation
slice before broader adoption.

Parent requirement IDs: KILN-REQ-011, KILN-REQ-012.

Parent specification IDs: KILN-SPEC-010.

Boundary/package IDs: KILN-ARCH-001, KILN-ARCH-011.

Design/interface/code-rigor IDs: KILN-IF-010, KILN-CR-014, KILN-CR-015.

Validation scenario IDs: all KILN-CONOPS scenarios.

Affected files/modules:

- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/VALIDATION.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/REVIEW.md`
- final evidence updates

Entry criteria:

- KILN-WP-001 through KILN-WP-007 complete.
- All validation commands have retained results.

Exit criteria:

- Verification evidence covers accepted requirements/specs.
- Validation evidence covers mission and CONOPS scenarios or records accepted risk.
- Trace rows connect the full V.
- Review decision is pass, pass_with_risk, blocked, or deferred.

Verification commands:

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | diff and docs checks | pending |
| L1 | yes | full verification command set | pending |
| L2 | yes | role review and trace closure | pending |

Status: proposed.

## Orphan Check

Before implementation starts, confirm:

- [x] Every accepted `KILN-REQ-*` is assigned to a work package or dispositioned.
- [x] Every accepted `KILN-SPEC-*` is assigned to a work package, verification item, or dispositioned.
- [x] Every interface-changing work package names `KILN-IF-*` IDs.
- [x] Every package/crate/module-changing work package names `KILN-ARCH-*` boundary IDs.
- [x] Every critical-code work package names `KILN-CR-*` IDs.
- [x] Every work package has exit criteria and verification commands.
- [x] Every work package lists L0/L1/L2 requirements or explicit non-requirement.
- [x] Every work package has V closure rows completed or scoped by package detail.
- [x] Every required assurance/security review lane is complete or accepted with risk before the package closes.
- [x] No work package is only "cleanup" without parent IDs or discovery status.

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Work packages must be ordered and scoped so implementation cannot bypass VTRACE. | Addressed by entry criteria and WP-008 proof closure. |
| Requirements Traceability Auditor | Every accepted requirement/spec/interface/code-rigor item needs package coverage. | Addressed by package mappings and orphan check. |
| Verification and Validation Lead | Work packages need validation levels and proof obligations. | Addressed by L0/L1/L2 rows and WP-008. |
| Software Assurance Guardian | Critical code packages must carry Code Rigor constraints. | Addressed by KILN-CR mappings. |
| Security Privacy Guardian | Side effects, dependencies, policy, package, and enterprise boundaries need explicit gates. | Addressed by package entry/exit criteria and boundary controls. |
| Source Custody Counsel | Public/enterprise and external-system references must remain boundary-only. | Addressed by integration deferrals. |
| Repo Maintainer | Packages should be small enough to implement and review independently. | Addressed by eight ordered packages. |
| Future Agent | Packages need stable IDs and resumption context. | Addressed by `KILN-WP-*` IDs and detailed package sections. |

## Work Package Gate

Decision: pass_with_risk.

Rationale: Work packages are scoped enough to proceed to Verification,
Validation, Trace, and Review planning. Implementation remains blocked until
those proof documents reach fixed point and the user explicitly starts the first
work package.

Validation note: KILN is not yet initialized as a Git repository, so
repo-local `git diff --check` and SHA-based stage closure are pending repository
initialization. This is recorded as process evidence gap, not implementation
approval.

Stage ledger:

| Repo | Stage | File | Status | Input SHA | Output SHA | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|---|---|---|
| KILN | WORK_PACKAGES | `docs/vtrace/WORK_PACKAGES.md` | settled | pending repo init | pending repo init | KILN `.roles` simulated fixed-point review | No unresolved critical/major findings; validation gap recorded | pass_with_risk | VERIFICATION |

## Source Links

- `docs/vtrace/IMPLEMENTATION_PLAN.md`
- `docs/vtrace/CODE_RIGOR.md`
- `docs/vtrace/DESIGN.md`
- `docs/vtrace/INTERFACES.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
