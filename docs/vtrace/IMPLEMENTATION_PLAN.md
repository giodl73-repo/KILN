# KILN Implementation Plan

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Implementation Plan |
| Implementation baseline | KILN foundation fixture-backed declaration checker |
| Stage status | Role-reviewed fixed point for Implementation Plan; implementation remains blocked until Work Packages, Verification, Validation, Trace, and Review authorize code. |

This plan sequences KILN implementation work. It does not itself authorize code.
Code starts only after `WORK_PACKAGES.md` defines approved work packages with
entry criteria, allowed files, exit criteria, validation commands, and evidence
requirements.

## Baseline Inputs

| Artifact | Status | Notes |
|---|---|---|
| `MISSION.md` | accepted | Public build/check/package mission; implementation blocked until VTRACE gates. |
| `CONOPS.md` | accepted | Six operating scenarios: declaration, runtime, policy, package, CAL, enterprise review. |
| `REQUIREMENTS.md` | accepted | `KILN-REQ-001` through `KILN-REQ-012`. |
| `SPECIFICATION_BASELINE.md` | accepted with risk | Target specs accepted; README/Product Plan/wave docs still need reconciliation. |
| `ARCHITECTURE.md` | accepted with risk | Layered architecture accepted; package/language choices intentionally unresolved. |
| `INTERFACES.md` | accepted with risk | Target `kiln.yaml`, `kiln check`, diagnostics, build record, handoff sections, and exit codes accepted. |
| `DESIGN.md` | accepted with risk | Fixture-first local checker design accepted; parser dependency and `not_ready` record behavior unresolved. |
| `CODE_RIGOR.md` | accepted with risk | Rust/Cargo rigor accepted; dependency choice and partial build-record question must be resolved in work packages. |
| `VERIFICATION.md` | pending | Must define tests/inspections before implementation work packages close. |
| `VALIDATION.md` | pending | Must define mission/CONOPS validation scenarios before implementation work packages close. |
| `TRACE.md` | pending | Must map mission to requirements/spec/design/work packages/evidence before readiness. |
| `REVIEW.md` | pending | Must record final pre-implementation readiness gate. |

## Implementation Strategy

Implement KILN in a strict foundation sequence:

1. Reconcile pre-VTRACE docs so README/Product Plan/wave records no longer imply
   unapproved implementation behavior.
2. Decide unresolved implementation questions in work-package entry gates:
   parser/serialization dependency, `not_ready` build-record behavior, and
   first fixture shape.
3. Build the smallest side-effect-free Rust foundation:
   - declaration model,
   - constrained `kiln.yaml` parsing or approved parser dependency,
   - readiness/boundary check rules,
   - diagnostic model,
   - JSON build-record emitter,
   - `kiln check` CLI,
   - retained fixture matrix.
4. Verify with local Cargo commands and fixture evidence.
5. Validate against mission/CONOPS scenarios before any integration claims.

No RUNE, CAL, WARDEN, DEPOT, GAUGE, ARCADE, WITNESS, Workbench, provider,
network, registry, policy, or enterprise integration is part of the foundation
implementation unless a later work package explicitly scopes an adapter.

## Sequencing

| Order | Work Package | Why This Order |
|---:|---|---|
| 1 | KILN-WP-001 Documentation baseline reconciliation | Removes conflicting pre-VTRACE implementation claims before code starts. |
| 2 | KILN-WP-002 Foundation workspace and dependency decision | Establishes Rust/Cargo shape and resolves parser/serialization dependency posture. |
| 3 | KILN-WP-003 Declaration model and fixtures | Defines the input records and retained examples that drive all checks. |
| 4 | KILN-WP-004 Parser and normalization | Converts local `kiln.yaml` files into the declaration model without side effects. |
| 5 | KILN-WP-005 Check rules and diagnostics | Implements readiness/boundary logic and explicit diagnostics. |
| 6 | KILN-WP-006 Build record emitter | Produces JSON records after model/check behavior is stable. |
| 7 | KILN-WP-007 CLI integration | Exposes `kiln check` after core behavior and records are tested. |
| 8 | KILN-WP-008 Verification/validation/trace closure | Runs proof package, closes gaps, and records readiness before broader adoption. |

## Source-To-Work-Package Mapping

| Source IDs | Work Package | Disposition | Notes |
|---|---|---|---|
| KILN-DES-010; KILN-UNK-005 | KILN-WP-001 | implement | Reconcile README/Product Plan/wave docs before code. |
| KILN-CR-010; KILN-CR-011; KILN-WAIVER-001 | KILN-WP-002 | implement / decide | Resolve dependency posture. |
| KILN-IF-001; KILN-FIX-001..KILN-FIX-010 | KILN-WP-003 | implement | Fixtures drive parser/checker behavior. |
| KILN-DES-002; KILN-DES-003; KILN-CR-001..KILN-CR-004 | KILN-WP-004 | implement | Parser/normalizer must be small and explicit. |
| KILN-REQ-003..KILN-REQ-005; KILN-IF-004; KILN-CR-005..KILN-CR-007 | KILN-WP-005 | implement | Rule/diagnostic/status core. |
| KILN-REQ-006; KILN-IF-003; KILN-WAIVER-002 | KILN-WP-006 | implement / decide | Resolve `not_ready` record behavior before emitter code. |
| KILN-IF-002; KILN-CR-007; KILN-CR-009 | KILN-WP-007 | implement | CLI comes after tested core. |
| KILN-REQ-011; KILN-REQ-012; KILN-SPEC-010 | KILN-WP-008 | implement | Close VTRACE proof, verification, validation, and trace. |
| RUNE intake | deferred | defer | Future adapter after foundation proof. |
| CAL/WARDEN/DEPOT/GAUGE/ARCADE/WITNESS integrations | deferred | defer | Future interfaces/adapters after foundation proof. |

## Boundary-To-Work-Package Mapping

| Boundary IDs | Work Package | Allowed Touches | Integration Needed |
|---|---|---|---|
| KILN-ARCH-001 | KILN-WP-001; KILN-WP-008 | VTRACE docs, README/Product Plan/wave reconciliation. | no |
| KILN-ARCH-002 | KILN-WP-003; KILN-WP-004 | Declaration model and fixtures. | no |
| KILN-ARCH-003 | KILN-WP-004; KILN-WP-005 | Parser/normalizer/check engine. | no |
| KILN-ARCH-004 | KILN-WP-005 | Diagnostic categories and outputs. | no |
| KILN-ARCH-005 | KILN-WP-006 | JSON build-record emitter. | no |
| KILN-ARCH-006 | KILN-WP-005; KILN-WP-006 | Policy-needs metadata and diagnostics only. | no |
| KILN-ARCH-007 | KILN-WP-005; KILN-WP-006 | Package metadata and diagnostics only. | no |
| KILN-ARCH-008 | KILN-WP-005; KILN-WP-006 | CAL reference metadata and diagnostics only. | no |
| KILN-ARCH-009 | KILN-WP-005; KILN-WP-006 | Runtime readiness metadata only. | no |
| KILN-ARCH-010 | KILN-WP-006; KILN-WP-008 | Enterprise-neutral evidence only. | no |
| KILN-ARCH-011 | KILN-WP-003; KILN-WP-008 | Fixtures and evidence manifest candidates. | no |

## Branch / Change Control

Branch strategy: use the current repo branch until a remote/submodule workflow is
established; after remote creation, use short feature branches per work package.

Worktree strategy: one KILN worktree for foundation implementation; do not mix
TRACKER submodule pointer work with child repo implementation commits.

Change-control trigger:

- any new external dependency;
- any public interface field or diagnostic category change;
- any change to readiness status semantics;
- any side-effect beyond explicitly requested output file;
- any enterprise-only dependency or private data assumption;
- any scope expansion into RUNE/CAL/WARDEN/DEPOT/GAUGE/ARCADE/WITNESS.

Rollback or revert strategy: each work package should commit independently after
validation so it can be reverted without removing unrelated VTRACE history.

## Commit / Push Policy

Commit scope: one work package per child repo commit.

Push condition: work package validation passes and review ledger has no critical
or major unresolved actionable finding.

Merge/readiness condition: Verification, Validation, Trace, and Review records
show the work package evidence and remaining risks.

## Wave / Pulse Policy

Active wave: `2026-06-03-foundation`.

Pulse mapping rule: each implementation work package should map to one pulse or
a named pulse section. If the current pre-VTRACE pulse language conflicts with
this plan, reconcile it in KILN-WP-001.

Pulse close condition:

- allowed files were respected;
- work-package exit criteria passed;
- validation commands were run;
- evidence pointers were added to VTRACE docs;
- review findings were closed or deferred.

## Integration Strategy

Foundation integration is internal only:

```text
fixtures
  -> declaration model
  -> parser/normalizer
  -> check rules
  -> diagnostics
  -> build record
  -> CLI
  -> verification/validation/trace/review evidence
```

External integrations are intentionally deferred. RUNE inputs, CAL references,
WARDEN policy, DEPOT package metadata, GAUGE/PROOF evidence, ARCADE runtime
handoff, and WITNESS/Workbench review display remain boundary sections in the
build record until later work packages define adapters.

## Verification Strategy

Verification must be fully specified in `docs/vtrace/VERIFICATION.md` before any
implementation work package closes.

Minimum foundation commands:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
cargo run -q -p kiln-cli -- check fixtures\missing-kiln\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\policy-authorized\kiln.yaml --format json
git diff --check
```

These command names are planned implementation targets and remain unavailable
until work packages create them.

## Validation Levels

| Level | Scope | Required Commands / Evidence | Required Before |
|---|---|---|---|
| L0 | Fast local sanity | `cargo fmt --check`; targeted unit/fixture tests; `git diff --check`. | commit |
| L1 | Full repo confidence | `cargo test --workspace`; all first-slice fixtures; dependency/code-rigor inspection. | push / PR |
| L2 | Integration or release readiness | Mock runtime/review/conformance evidence, trace closure, and role review. | downstream adoption / release |

## Risks

| Risk ID | Risk | Mitigation | Owner |
|---|---|---|---|
| KILN-RISK-001 | Pre-VTRACE docs imply code scope that VTRACE has not authorized. | KILN-WP-001 reconciles docs before code. | KILN |
| KILN-RISK-002 | Hand-rolled parser accepts invalid YAML or creates false readiness. | Code Rigor requires constrained fixtures or approved parser dependency. | KILN |
| KILN-RISK-003 | Parser/serialization dependency adds supply-chain risk. | Dependency review before addition. | KILN |
| KILN-RISK-004 | `not_ready` build-record behavior remains unresolved. | Decide in KILN-WP-006 before emitter code. | KILN |
| KILN-RISK-005 | Boundary sections imply real WARDEN/DEPOT/CAL/ARCADE/WITNESS integration. | Keep outputs as metadata and diagnostics only in foundation. | KILN |
| KILN-RISK-006 | Side-effect-free claim is violated by file writes or hidden process/network behavior. | Code Rigor and verification test/inspect no hidden effects. | KILN |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Implementation plan must sequence real work without bypassing proof stages. | Addressed by pre-code proof requirements and ordered work packages. |
| Requirements Traceability Auditor | Plan must map source IDs to work packages or deferrals. | Addressed by source-to-work-package mapping. |
| Verification and Validation Lead | Verification/Validation/Trace/Review must be designed before work packages close. | Addressed by baseline inputs and KILN-WP-008. |
| Software Assurance Guardian | Code Rigor risks must become work-package entry/exit gates. | Addressed by sequencing and verification strategy. |
| Security Privacy Guardian | Side effects, dependencies, policy, and package trust must remain controlled. | Addressed by change-control triggers and risks. |
| Source Custody Counsel | Public/enterprise boundary must remain clean. | Addressed by external integration deferrals. |
| Repo Maintainer | First implementation must be small and reviewable. | Addressed by eight narrow work packages. |
| Future Agent | Plan must be resumable without chat history. | Addressed by stable work-package IDs and mappings. |

## Implementation Readiness Decision

Decision: deferred.

Rationale: The implementation plan is ready to proceed to Work Packages and the
right-side proof package, but implementation is not yet authorized. Before code
starts, KILN still needs `WORK_PACKAGES.md`, `VERIFICATION.md`, `VALIDATION.md`,
`TRACE.md`, and `REVIEW.md` fixed points.

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `docs/vtrace/ARCHITECTURE.md`
- `docs/vtrace/INTERFACES.md`
- `docs/vtrace/DESIGN.md`
- `docs/vtrace/CODE_RIGOR.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
