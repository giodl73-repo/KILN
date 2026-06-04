# KILN Interfaces

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Interfaces |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/ARCHITECTURE.md` |
| Stage status | Role-reviewed fixed point for Interfaces; implementation remains blocked until Code Rigor, Implementation Plan, and Work Packages authorize code. |

This interface baseline controls the first implementation-facing surfaces for
KILN without authorizing code. Concrete files, command names, schemas, and
records named here are target interfaces for later Code Rigor, Implementation
Plan, and Work Packages.

## Interface Inventory

| ID | Interface | Type | Owner | Consumers | Compatibility Rule | Verification |
|---|---|---|---|---|---|---|
| KILN-IF-001 | KILN declaration file | file / schema | KILN | capability authors, checker | Versioned by `kiln.version`; required sections cannot be removed without change control. | schema/fixture inspection |
| KILN-IF-002 | `kiln check` command | CLI | KILN | repo maintainers, CI, future agents | Side-effect-free command; output shape changes require interface review. | CLI fixture tests |
| KILN-IF-003 | Build record | file / schema | KILN | runtimes, review clients, conformance tools | Must preserve status, checked inputs, diagnostics, unresolved gates, evidence obligations, and handoff summaries. | fixture/record inspection |
| KILN-IF-004 | Diagnostic record | schema / event-like record | KILN | authors, CI, review clients | Diagnostic codes are stable once accepted; new codes may be added. | negative fixture tests |
| KILN-IF-005 | Policy-needs handoff | schema section | KILN / WARDEN boundary | policy engines, reviewers | KILN may report needs, never authorization success. | policy negative fixture |
| KILN-IF-006 | Package-metadata handoff | schema section | KILN / DEPOT boundary | registries, package reviewers | KILN may report package intent, never publication/signing/registry mutation. | package negative fixture |
| KILN-IF-007 | CAL primitive reference | schema section | KILN / CAL boundary | CAL authors, capability authors | KILN records identifiers/version expectations only; CAL owns semantics. | boundary review |
| KILN-IF-008 | Runtime-readiness handoff | schema section | KILN / runtime boundary | ARCADE-style runtimes, future hosts | KILN reports checked/gated status, never executes. | mock-runtime fixture |
| KILN-IF-009 | Enterprise review evidence view | adapter record | KILN / enterprise adapter boundary | FLETCHER/Workbench-style clients | Adapter consumes public evidence; KILN core does not gain enterprise fields/deps. | dependency inspection |
| KILN-IF-010 | Conformance evidence manifest | file / schema | KILN / GAUGE-PROOF boundary | GAUGE/PROOF-style tools | Evidence is inspectable without KILN owning conformance/report tools. | evidence fixture review |

## Interface Details

### KILN-IF-001: KILN declaration file

Purpose: The declared managed-agent capability input that KILN checks.

Target file name: `kiln.yaml`.

Minimum target sections:

| Section | Purpose | Required in first implementation slice |
|---|---|---|
| `kiln` | KILN declaration identity, version, and schema/profile marker. | yes |
| `capability` | Managed-agent capability identity, version, summary, and owner. | yes |
| `inputs` | Declared source specs, descriptors, fixtures, or records used by the capability. | yes |
| `outputs` | Expected build record, artifacts, or generated outputs. | yes |
| `checks` | Required check gates and expected evidence obligations. | yes |
| `policy_needs` | Data/tool/network/write/provider/budget/approval needs for later policy systems. | no; required only when policy needs exist |
| `package` | Package identity, version, and publication intent for later registry systems. | no; required only when package handoff exists |
| `cal_refs` | Common Agent Library primitive references. | no; required only when CAL primitives are referenced |
| `runtime` | Intended runtime/readiness handoff targets. | no; required only when runtime handoff exists |
| `review` | Intended review/conformance evidence consumers. | no |

Inputs: Human- or tool-authored YAML.

Outputs: Parsed declaration model in future implementation; no output in this
stage.

Errors: Missing required section, unknown version, duplicate identity,
unsupported section, ambiguous reference, unsupported handoff target.

Versioning or compatibility: `kiln.version` is required. First target version is
`v0`. Version `v0` is allowed to change until the first implementation review
freezes it.

Evidence: Future valid, missing-field, and degraded declaration fixtures.

### KILN-IF-002: `kiln check` command

Purpose: Side-effect-free validation of a KILN declaration.

Target command:

```powershell
kiln check <path-to-kiln.yaml> --format text|json --out <build-record.json>
```

Inputs:

- KILN declaration file.
- Optional output path for build record.
- Optional output format.

Outputs:

- Process exit code.
- Human-readable or JSON diagnostics.
- Optional build record when the declaration is ready or degraded-but-recordable.

Errors:

- Usage error.
- Input file not found.
- Parse error.
- Schema/version error.
- Not-ready declaration.
- Internal error.

Versioning or compatibility: CLI flag compatibility is controlled after `v0`
freezes. `check` must remain side-effect-free.

Evidence: Future CLI fixture tests. No provider calls, network calls, product
writes, runtime execution, registry mutation, package publication, or policy
authorization may occur.

### KILN-IF-003: Build record

Purpose: Evidence-bearing output that states what KILN checked and what remains
gated.

Target file name: `kiln.build.json`.

Required target fields:

| Field | Purpose |
|---|---|
| `kiln_version` | KILN record version. |
| `capability` | Capability identity and version. |
| `status` | `ready`, `not_ready`, or `degraded`. |
| `checked_inputs` | Declared inputs KILN inspected. |
| `diagnostics` | Diagnostic records emitted by the check. |
| `unresolved_gates` | Policy, package, runtime, evidence, compatibility, or review gates not satisfied by KILN. |
| `evidence_obligations` | Evidence still required for verification or validation. |
| `handoffs` | Policy/package/CAL/runtime/review/conformance handoff summaries. |
| `trace` | Parent requirement/spec/interface IDs relevant to the record. |

Inputs: Checked declaration model and diagnostics.

Outputs: JSON build record for `ready` and `degraded` declarations with known
KILN version and capability identity. `not_ready` or identity-missing
declarations emit a diagnostics-only JSON envelope, not a partial build record.

Errors: Record cannot be emitted if declaration identity or version is unknown.

Versioning or compatibility: Fields may be added in `v0`; removing required
fields requires interface review.

Evidence: Build-record and diagnostics-envelope fixture inspection.

### KILN-IF-004: Diagnostic record

Purpose: Stable, machine-readable explanation of failed, degraded, or notable
conditions.

Target diagnostic categories:

| Category | Meaning |
|---|---|
| `missing_required` | Required declaration field or section is absent. |
| `ambiguous_reference` | A reference cannot be resolved uniquely. |
| `unsupported_version` | Declared version is unknown or unsupported. |
| `unsupported_handoff` | Handoff target is not recognized or not baselined. |
| `evidence_missing` | Evidence obligation is declared but not satisfied. |
| `policy_unresolved` | Policy need exists but authorization is outside KILN. |
| `package_not_ready` | Package metadata is incomplete or publication is not allowed. |
| `runtime_not_ready` | Runtime handoff lacks required readiness fields. |
| `boundary_violation` | Declaration asks KILN to own runtime, policy, registry, CAL, enterprise, or provider behavior. |

Inputs: Declaration model and checker findings.

Outputs: Diagnostic list in text or JSON output and build record.

Errors: None; diagnostics are the error reporting surface.

Versioning or compatibility: Diagnostic categories are append-only after `v0`
freezes.

Evidence: Future negative fixture tests.

### KILN-IF-005: Policy-needs handoff

Purpose: Express policy needs for WARDEN or another policy decision system.

Inputs: `policy_needs` section from declaration.

Outputs: Policy-needs summary in build record.

Errors: Missing policy declaration for a capability that requests data/tool/
network/write/provider/budget/approval authority.

Versioning or compatibility: KILN must not emit `authorized: true` or equivalent
authorization-success claims.

Evidence: Future negative fixture proving KILN reports unresolved policy rather
than granting authority.

### KILN-IF-006: Package-metadata handoff

Purpose: Express package metadata for DEPOT or another registry system.

Inputs: `package` section from declaration.

Outputs: Package handoff summary in build record.

Errors: Missing package identity/version, unsupported publication intent, or
trust/signing claims that KILN cannot own.

Versioning or compatibility: KILN must not publish, sign, distribute, or mutate
registry state.

Evidence: Future negative fixture proving no registry mutation.

### KILN-IF-007: CAL primitive reference

Purpose: Reference common agent primitives without defining their semantics in
KILN.

Inputs: `cal_refs` declaration section.

Outputs: CAL reference summary and unresolved compatibility notes.

Errors: Missing primitive identifier, unsupported version expectation, or
attempt to define primitive behavior in KILN declaration.

Versioning or compatibility: CAL owns primitive compatibility. KILN only records
identifier and version expectations.

Evidence: Future boundary review fixture.

### KILN-IF-008: Runtime-readiness handoff

Purpose: Let runtime systems know what KILN checked and what remains gated before
execution.

Inputs: `runtime` declaration section plus build status.

Outputs: Runtime-readiness summary in build record.

Errors: Missing runtime target, incompatible status, unresolved gates hidden from
runtime handoff.

Versioning or compatibility: KILN must not execute runtime work.

Evidence: Future mock-runtime fixture.

### KILN-IF-009: Enterprise review evidence view

Purpose: Let FLETCHER/Workbench-style clients display public KILN evidence
without changing KILN core.

Inputs: Build record and diagnostics.

Outputs: Adapter-ready evidence view. The adapter may live outside KILN if it
becomes enterprise-specific.

Errors: Enterprise-only field required in KILN core, private data requirement,
or dependency on enterprise repo from KILN core.

Versioning or compatibility: Public core output remains enterprise-neutral.

Evidence: Future dependency inspection and adapter fixture.

### KILN-IF-010: Conformance evidence manifest

Purpose: Provide GAUGE/PROOF-style tools with inspectable evidence without KILN
owning conformance or report generation.

Inputs: Build record, diagnostics, fixtures, and trace IDs.

Outputs: Evidence manifest or retained fixture inventory.

Errors: Missing evidence pointer, unknown trace ID, or conformance claim without
fixture evidence.

Versioning or compatibility: Evidence manifest shape must preserve trace IDs and
fixture pointers after accepted.

Evidence: Future evidence fixture review.

## Exit Codes

Target CLI exit-code contract:

| Code | Meaning |
|---:|---|
| 0 | Declaration is ready. |
| 1 | Declaration is degraded or not ready due to check diagnostics. |
| 2 | Usage, parse, schema, or input-file error. |
| 3 | Internal KILN error. |

Exit codes are a target interface and remain unimplemented until work packages
authorize code.

## Open Questions

- Should `kiln check` emit a build record for `not_ready` declarations, or only
  for `ready` and `degraded` declarations?
- Should the first declaration file be YAML only, JSON only, or YAML input with
  JSON output?
- Should RUNE descriptor collections be part of `inputs` in the first slice or a
  later adapter?
- Which fields are mandatory for package handoff before DEPOT exists?
- Should text output be considered stable, or should only JSON be compatibility
  controlled?

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Interfaces must define the first implementation-facing contract without absorbing adjacent systems. | Addressed by separate declaration, check, build record, diagnostics, and handoff interfaces. |
| Requirements Traceability Auditor | Interfaces must map to spec and requirement IDs. | Addressed by inventory and details derived from `KILN-SPEC-*` and `KILN-REQ-*`. |
| Verification and Validation Lead | Interfaces need future evidence hooks without claiming implementation exists. | Addressed by fixture/evidence entries on every interface. |
| Software Assurance Guardian | Interfaces must not authorize code before code rigor and work packages. | Addressed by scope and exit-code caveat. |
| Security Privacy Guardian | Interfaces must preserve side-effect-free checks and prevent policy/package authority creep. | Addressed by KILN-IF-005, KILN-IF-006, prohibited claims, and exit-code contract. |
| Source Custody Counsel | Enterprise and external system names must remain boundaries. | Addressed by adapter/handoff language. |
| Repo Maintainer | Interfaces should identify the smallest useful first slice. | Addressed by `kiln.yaml`, `kiln check`, diagnostics, and fixture evidence. |
| Future Agent | Interfaces need stable IDs for work packages and trace rows. | Addressed by `KILN-IF-*` IDs. |

## Interface Gate

Decision: pass_with_risk.

Rationale: The interfaces are specific enough to proceed to Code Rigor and
Implementation Planning. The accepted risks are unresolved `v0` field details,
RUNE intake timing, and whether text output is stable. Implementation remains
blocked until Code Rigor, Implementation Plan, and Work Packages settle.

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `docs/vtrace/ARCHITECTURE.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
