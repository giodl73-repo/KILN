# KILN Detailed Design

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Detailed Design |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/ARCHITECTURE.md`, `docs/vtrace/INTERFACES.md` |
| Stage status | Role-reviewed fixed point for Detailed Design; implementation remains blocked until Code Rigor, Implementation Plan, and Work Packages authorize code. |

This design defines the first implementation strategy for KILN's foundation
slice. It does not authorize code. Code Rigor, Implementation Plan, and Work
Packages must still settle before implementation begins.

## Design Decision Summary

| ID | Decision | Requirement IDs | Rationale | Alternatives | Evidence |
|---|---|---|---|---|---|
| KILN-DES-001 | Implement the first slice as a fixture-backed declaration checker over `kiln.yaml`. | KILN-REQ-001; KILN-REQ-003; KILN-REQ-004 | Proves KILN's build/check contract without broad integration or runtime execution. | Start with RUNE intake, package publishing, or runtime handoff first. | Future valid/missing/degraded fixtures. |
| KILN-DES-002 | Treat `v0` declaration parsing as constrained section/field extraction, not a general workflow language. | KILN-REQ-003; KILN-REQ-009 | Prevents product-specific workflow creep and keeps the first proof small. | Full YAML object model, custom DSL, JSON-only declaration. | Fixture inspection and parser tests. |
| KILN-DES-003 | Use a simple check pipeline: load -> parse -> normalize -> validate -> classify -> emit. | KILN-REQ-003; KILN-REQ-004; KILN-REQ-005 | Keeps failure modes explicit and testable. | Monolithic command logic or implicit best-effort parsing. | Unit tests for each stage. |
| KILN-DES-004 | Model status as exactly `ready`, `not_ready`, or `degraded` for the first slice. | KILN-REQ-005; KILN-REQ-006 | Gives runtimes/reviewers deterministic readiness without overfitting early states. | Boolean pass/fail, many detailed statuses. | Status fixture tests. |
| KILN-DES-005 | Emit explicit diagnostics for every not-ready/degraded condition. | KILN-REQ-004; KILN-REQ-011 | Missing evidence must be visible and machine-readable. | Human-only errors, silent defaults, broad catch-all failures. | Negative fixture tests. |
| KILN-DES-006 | Emit JSON build records; treat human text as non-stable display output during `v0`. | KILN-REQ-006; KILN-REQ-012 | JSON gives downstream tools a stable target while avoiding early text compatibility burden. | Stable text output, YAML output, binary record. | Build-record fixture inspection. |
| KILN-DES-007 | Keep policy, package, CAL, runtime, enterprise review, and conformance outputs as sections in the build record, not active integrations. | KILN-REQ-007; KILN-REQ-008; KILN-REQ-009; KILN-REQ-010 | Preserves boundaries while making handoff intent visible. | Call WARDEN/DEPOT/ARCADE, implement CAL primitives, embed enterprise fields. | Boundary fixtures and dependency inspection. |
| KILN-DES-008 | Make side-effect freedom a design invariant, not only a test expectation. | KILN-REQ-003; KILN-REQ-002 | KILN's first credibility claim depends on no hidden execution, mutation, or network/provider behavior. | Allow optional network/registry/provider flags in first slice. | Code review, dependency inspection, fixture tests. |
| KILN-DES-009 | Keep RUNE descriptor collections out of the first implementation slice unless represented as opaque declared inputs. | KILN-REQ-002; KILN-REQ-009 | Avoids coupling KILN's core proof to RUNE integration before declared input semantics settle. | Make RUNE mandatory upstream in the first slice. | Interface review and future RUNE work package. |
| KILN-DES-010 | Reconcile pre-VTRACE README/Product Plan/wave implementation claims before code work packages start. | KILN-REQ-011; KILN-REQ-012 | Prevents conflicting source-of-truth claims for crates, CLI, fixtures, and validation commands. | Leave conflicting docs until after code. | Documentation diff before implementation planning. |

## Algorithms / Logic

### First-slice check pipeline

```text
load declaration file
  -> parse constrained sections
  -> normalize declaration fields
  -> validate required sections and identities
  -> validate optional handoff sections
  -> classify status
  -> emit diagnostics
  -> emit build record when allowed
```

### Load

Input is a local file path. The first implementation slice reads only the named
file. It must not discover files recursively, read environment credentials,
access the network, call providers, invoke tools, or mutate the workspace.

### Parse

The first parser recognizes the target `kiln.yaml` sections from
`KILN-IF-001`. It should be intentionally conservative:

- required top-level sections are recognized by name;
- unknown top-level sections produce diagnostics unless explicitly allowed;
- duplicate identity/version fields produce diagnostics;
- deeply nested semantics are not interpreted in the first slice.

If Code Rigor later approves a YAML dependency, the parser may use it. If not,
the first slice may use constrained fixture parsing sufficient for retained
fixtures, but must document that limitation in diagnostics and verification.

### Normalize

Normalization converts parsed fields into a declaration model:

| Model Area | Minimum Content |
|---|---|
| KILN identity | `kiln.version` |
| Capability identity | capability id and version |
| Inputs | declared input list or explicit empty diagnostic |
| Outputs | expected build-record path or artifact declaration |
| Checks | required gates/evidence obligations |
| Handoffs | optional policy/package/CAL/runtime/review/conformance sections |

### Validate

Validation applies deterministic rules:

1. Required sections must exist.
2. Required identities must be non-empty.
3. `kiln.version` must be supported by the implementation.
4. Handoff sections must not claim behavior KILN does not own.
5. Evidence obligations must not be treated as satisfied unless represented by
   accepted fixture evidence.
6. Unknown or unsupported sections become diagnostics.

### Classify

Target classification:

| Status | Rule |
|---|---|
| `ready` | Required declaration data is present, no blocking diagnostics exist, and unresolved handoffs are explicit. |
| `degraded` | Declaration can produce a build record, but non-blocking diagnostics or unresolved downstream gates remain. |
| `not_ready` | Required declaration data, identity, version, or safety boundary is missing or invalid. |

### Emit

Diagnostic output is always allowed. Build-record output is allowed for `ready`
and `degraded`. Whether `not_ready` also emits a partial build record remains an
open design question for implementation planning.

## State Transitions

```text
unread
  -> load_failed
  -> parsed
  -> invalid
  -> not_ready

unread
  -> parsed
  -> normalized
  -> checked
  -> degraded
  -> emitted

unread
  -> parsed
  -> normalized
  -> checked
  -> ready
  -> emitted
```

Invalid input never transitions to runtime execution, policy authorization,
registry mutation, package publication, provider call, or enterprise adapter
behavior inside KILN.

## Invariants

- KILN core remains public and product-neutral.
- The first implementation slice is local-file-only and side-effect-free.
- KILN never executes a managed-agent workflow.
- KILN never grants policy authorization.
- KILN never publishes, signs, distributes, or mutates registry state.
- KILN never defines CAL primitive semantics.
- KILN core never depends on enterprise-only repos or private product surfaces.
- Missing evidence is represented as a diagnostic, unresolved gate, degraded
  status, not-ready status, or deferred VTRACE item.
- Accepted IDs from VTRACE docs remain stable unless changed through review.

## Edge Cases

| Edge Case | Expected Behavior | Verification |
|---|---|---|
| Missing `kiln` section | `not_ready`; `missing_required` diagnostic. | Negative fixture. |
| Unsupported `kiln.version` | `not_ready`; `unsupported_version` diagnostic. | Negative fixture. |
| Missing capability identity | `not_ready`; `missing_required` diagnostic. | Negative fixture. |
| Duplicate capability identity | `not_ready` or `degraded` based on parser certainty; explicit diagnostic required. | Negative fixture. |
| Unknown top-level section | `degraded` or `not_ready` depending on section risk; explicit diagnostic required. | Negative fixture. |
| Policy need with implied authorization | `not_ready`; boundary diagnostic; no `authorized: true` output. | Policy negative fixture. |
| Package section claims publication success | `not_ready`; boundary diagnostic; no registry mutation. | Package negative fixture. |
| CAL reference attempts to define primitive behavior | `not_ready`; boundary diagnostic. | CAL boundary fixture. |
| Runtime target hides unresolved gates | `not_ready`; runtime readiness diagnostic. | Runtime negative fixture. |
| Enterprise-only field required in KILN core | `not_ready`; boundary diagnostic or rejected interface. | Dependency/boundary review. |
| Missing evidence pointer | `degraded` or `not_ready` depending on required gate; explicit `evidence_missing`. | Evidence fixture. |
| Output path omitted | Text/JSON diagnostics still emitted; build record path uses default only if specified by work package. | CLI fixture. |
| Input file not found | Exit code 2; usage/input error; no build record. | CLI fixture. |
| Internal bug | Exit code 3; no success-shaped fallback. | Error-handling test/review. |

## Migration / Rollout

KILN has no accepted implementation yet, so rollout is staged:

1. Reconcile README/Product Plan/wave docs so they no longer claim pre-VTRACE
   implementation behavior.
2. Freeze Code Rigor constraints for parser/checker/diagnostic work.
3. Create Implementation Plan and Work Packages for the fixture-backed first
   slice.
4. Implement local-only `kiln check` and retained fixtures.
5. Verify valid, missing-field, degraded, policy-boundary, package-boundary, and
   dependency-boundary scenarios.
6. Only then consider RUNE intake, CAL references, WARDEN/DEPOT/GAUGE handoff
   adapters, or enterprise display adapters.

## Code Rigor Hooks

| Area | Risk | Required Code Rigor Constraint |
|---|---|---|
| Declaration parser | Parser ambiguity can create false readiness. | Small functions, explicit parse errors, no silent defaults, fixture coverage for malformed input. |
| Check engine | Boundary mistakes can imply runtime, policy, registry, or CAL ownership. | Rule functions must map to requirement/spec/interface IDs and have negative fixtures. |
| Diagnostics | Broad catch-all errors can hide actionable evidence gaps. | Stable diagnostic categories, no success-shaped fallback, explicit unknown/unsupported cases. |
| Build record emitter | Missing fields can mislead downstream consumers. | Required-field construction, deterministic ordering where practical, fixture snapshot review. |
| CLI error handling | Exit codes can misrepresent readiness. | Explicit exit-code mapping, no broad catch that exits success, tests for input/usage/internal classes. |
| Dependencies | YAML/parser or CLI dependencies can expand supply-chain risk. | Dependency review before adding non-std crates; no enterprise/provider/runtime SDK dependencies. |
| File operations | Accidental workspace mutation would violate side-effect-free claim. | Read-only input handling unless output path is explicitly requested; tests/inspection for no hidden writes. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Design must show implementable logic without absorbing runtime/policy/registry/library responsibilities. | Addressed by pipeline, invariants, and boundary edge cases. |
| Requirements Traceability Auditor | Design decisions need links to requirements and stable IDs. | Addressed by `KILN-DES-*` decisions and requirement links. |
| Verification and Validation Lead | Design needs fixture/evidence hooks for every meaningful behavior. | Addressed by edge-case verification and rollout plan. |
| Software Assurance Guardian | Design must identify Code Rigor constraints before code. | Addressed by Code Rigor Hooks. |
| Security Privacy Guardian | Design must prevent side effects, authority creep, registry trust creep, and enterprise dependency leak. | Addressed by invariants, prohibited transitions, and edge cases. |
| Source Custody Counsel | Design must not turn adjacent product names into dependencies or endorsement claims. | Addressed by boundary wording and rollout sequencing. |
| Repo Maintainer | Design should enable a small first work package. | Addressed by fixture-first local checker. |
| Future Agent | Design must be resumable into code rigor and work packages. | Addressed by stable IDs, pipeline, edge cases, and rollout steps. |

## Design Gate

Decision: pass_with_risk.

Rationale: The design is specific enough to proceed to Code Rigor. The accepted
risks are the unresolved parser dependency choice, whether `not_ready` emits a
partial build record, and the need to reconcile pre-VTRACE docs before
implementation planning.

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `docs/vtrace/ARCHITECTURE.md`
- `docs/vtrace/INTERFACES.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
