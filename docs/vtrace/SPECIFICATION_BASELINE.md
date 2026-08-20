# KILN Specification Baseline

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Specification Baseline |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md` |
| Baseline type | mixed: current docs/process scaffold, target controlled build/check contract |
| Baseline date | 2026-06-03 |
| Stage status | Role-reviewed fixed point for Specification Baseline; implementation remains blocked until Architecture, Interfaces, Code Rigor, Implementation Plan, and Work Packages authorize code. |

This baseline separates observed current scaffold content from the target KILN
contract. It does not authorize code. It controls the first implementation
shape only far enough for later Architecture, Interfaces, Code Rigor,
Implementation Plan, and Work Packages to decide concrete crates, commands,
schemas, fixtures, and validation commands.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `docs/vtrace/MISSION.md` | Mission fixed point | target | Controls public/product-neutral mission and implementation-blocked posture. |
| `docs/vtrace/CONOPS.md` | CONOPS fixed point | target | Controls actors, scenarios, side-effect-free operation, and handoff boundaries. |
| `docs/vtrace/REQUIREMENTS.md` | Requirements fixed point | target | Controls accepted KILN requirements and deferred implementation/interface detail. |
| `README.md` | Pre-VTRACE scaffold narrative | current / pre-baseline | Contains useful intent, but crate/CLI/validation language is not accepted behavior until reconciled. |
| `PRODUCT_PLAN.md` | Pre-VTRACE product plan | current / pre-baseline | Contains roadmap and layer decisions, but implementation specifics remain non-baselined. |
| `context/waves/` | Pre-VTRACE scaffold wave docs | current / pre-baseline | Mentions Rust workspace and CLI work that must be revised before implementation planning. |
| `.roles/` | Repo-local review panel | current | Accepted as the local role-review mechanism for VTRACE stages. |
| Tests / fixtures | None | unknown | No tests or fixtures are accepted yet. |
| CLI/API behavior | None | unknown | No CLI, API, crate, schema, or file format is accepted yet. |
| Released package / downstream use | None | unknown | No public release or downstream adoption is claimed. |
| TRACKER roadmap | `dependency-systems/managed-agent-platform-roadmap.md` | target context | Records portfolio placement: KILN public; CAL/WARDEN/DEPOT/GAUGE planned; enterprise surfaces separate. |

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| KILN-SPEC-001 | KILN-REQ-001; KILN-REQ-002 | product | target | KILN is a public Standards & Protocols repo for a product-neutral managed-agent build/check/package contract. | inspection / review | operator review | KILN | medium | proposed |
| KILN-SPEC-002 | KILN-REQ-003 | ops / safety | target | KILN foundation operations are side-effect-free: no provider calls, network calls, product writes, runtime execution, registry mutation, package publication, or policy authorization. | test / inspection | scenario review | KILN | high | proposed |
| KILN-SPEC-003 | KILN-REQ-003; KILN-REQ-004; KILN-REQ-005 | software | target | KILN accepts a declared capability record, checks it for completeness and readiness, and reports ready, not-ready, or degraded status with diagnostics. | test / analysis | scenario review | KILN | high | proposed |
| KILN-SPEC-004 | KILN-REQ-004; KILN-REQ-011 | software | target | KILN diagnostics identify missing, ambiguous, unsupported, incompatible, or unproven declaration/evidence conditions without silently treating them as ready. | test / inspection | operator review | KILN | high | proposed |
| KILN-SPEC-005 | KILN-REQ-006 | interface | target | KILN build records identify what was checked, which gates remain unresolved, and which downstream systems may consume the handoff. | test / inspection | runtime handoff scenario | KILN | high | proposed |
| KILN-SPEC-006 | KILN-REQ-007 | interface | target | KILN records policy needs as declaration data and handoff metadata, but authorization decisions remain out of scope. | inspection / test | policy handoff scenario | KILN / WARDEN boundary | high | proposed |
| KILN-SPEC-007 | KILN-REQ-008 | interface / package | target | KILN records package metadata and publication intent, but registry mutation, signing trust, publication, and distribution remain out of scope. | inspection / test | package handoff scenario | KILN / DEPOT boundary | high | proposed |
| KILN-SPEC-008 | KILN-REQ-009 | interface / package | target | KILN may reference common agent primitive identifiers, but CAL owns primitive semantics and compatibility. | inspection / review | CAL boundary scenario | KILN / CAL boundary | medium | proposed |
| KILN-SPEC-009 | KILN-REQ-010 | interface | target | Enterprise clients may consume KILN evidence through adapters without adding enterprise-only vocabulary or dependencies to KILN core. | inspection / demonstration | review-display scenario | KILN / enterprise adapter boundary | medium | proposed |
| KILN-SPEC-010 | KILN-REQ-012 | ops / trace | target | KILN VTRACE artifacts preserve traceability from needs and scenarios through requirements, specs, interfaces/design, verification, validation, and evidence. | trace review | future-agent review | KILN | medium | proposed |
| KILN-SPEC-011 | KILN-REQ-001; KILN-REQ-003; KILN-REQ-004 | test | target | The first implementation slice must be fixture-backed and must include valid, missing-field, and degraded declaration examples before broader integration. | test / inspection | repo maintainer review | KILN | medium | proposed |
| KILN-SPEC-012 | KILN-REQ-002; KILN-REQ-010 | package / dependency | target | KILN core must not depend on LATTICE, WITNESS, BAKER, enterprise connectors, tenant data, provider SDKs, or private product surfaces. | inspection / dependency review | public adoption scenario | KILN | high | proposed |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| KILN-PC-001 | KILN-SPEC-001; KILN-SPEC-010 | VTRACE docs | Mission, CONOPS, Requirements, Specification Baseline, and later trace docs must keep stable IDs once introduced. | Renaming, removing, or changing meaning of any `KILN-*` ID. | Role review and trace review. |
| KILN-PC-002 | KILN-SPEC-003; KILN-SPEC-004; KILN-SPEC-005 | build record shape | Exact fields are deferred, but future changes must preserve explicit status, diagnostics, checked inputs, unresolved gates, and evidence obligations. | Adding/removing readiness state, diagnostic category, checked input class, or evidence obligation. | Future fixture tests and schema/interface review. |
| KILN-PC-003 | KILN-SPEC-006 | policy handoff | KILN may describe policy needs but must not expose an authorization-success claim as KILN-owned behavior. | Any field or command implying KILN grants authority. | Security/privacy review and negative fixture. |
| KILN-PC-004 | KILN-SPEC-007 | package handoff | KILN may describe package metadata but must not publish, sign trust, mutate registry state, or claim distribution success. | Any package command, registry field, signing behavior, or publication workflow. | Source custody and security review. |
| KILN-PC-005 | KILN-SPEC-008 | CAL reference | KILN may reference CAL primitive identifiers only through a controlled interface; CAL owns semantics. | Any KILN behavior that validates or implements primitive semantics. | Boundary review. |
| KILN-PC-006 | KILN-SPEC-009; KILN-SPEC-012 | enterprise adapter boundary | Enterprise consumption must be through adapters or downstream clients; KILN core stays public and dependency-clean. | Adding enterprise fields, private data assumptions, or enterprise repo dependencies. | Dependency inspection and role review. |

## Package / Language Allocation

| Spec IDs | Package / Crate / Module / Language | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| KILN-SPEC-001; KILN-SPEC-010 | docs/vtrace | Controlled VTRACE intent, trace, and readiness records. | Executable behavior claims. | L0 |
| KILN-SPEC-002; KILN-SPEC-003; KILN-SPEC-004; KILN-SPEC-005 | future KILN core library | Product-neutral declaration, status, diagnostics, evidence obligation, and build-record model. | Runtime execution, provider calls, registry mutation, policy authorization, enterprise vocabulary. | L1 |
| KILN-SPEC-003; KILN-SPEC-004; KILN-SPEC-011 | future fixture/checker surface | Side-effect-free validation of controlled fixture examples. | Broad workflow parsing or product-specific language execution. | L1 |
| KILN-SPEC-006 | future policy handoff adapter | Emit/validate policy-needs metadata. | Decide authorization or grant authority. | L1 / L2 |
| KILN-SPEC-007 | future package handoff adapter | Emit/validate DEPOT-ready package metadata. | Publish, sign trust, distribute, or mutate registry state. | L1 / L2 |
| KILN-SPEC-008 | future CAL reference adapter | Reference CAL primitive IDs and compatibility expectations. | Define CAL primitive behavior. | L1 / L2 |
| KILN-SPEC-009 | future enterprise display adapter | Let enterprise clients display public KILN evidence. | Add enterprise dependency to KILN core. | L2 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| KILN-NF-001 | KILN-SPEC-001; KILN-SPEC-012 | Public dependency posture | KILN core has no enterprise-only dependency. | dependency inspection | proposed |
| KILN-NF-002 | KILN-SPEC-002 | Side-effect-free foundation | Foundation checks perform no provider calls, network calls, product writes, runtime execution, registry mutation, package publication, or policy authorization. | test / inspection | proposed |
| KILN-NF-003 | KILN-SPEC-004 | Fail-visible diagnostics | Missing evidence or invalid declarations must produce explicit diagnostic/degraded/not-ready output, not silent success. | test | proposed |
| KILN-NF-004 | KILN-SPEC-010 | Trace continuity | Every accepted requirement maps to at least one spec item or named deferral before implementation planning. | trace review | proposed |
| KILN-NF-005 | KILN-SPEC-006; KILN-SPEC-007; KILN-SPEC-008; KILN-SPEC-009 | Boundary preservation | Policy, package, CAL, enterprise display, runtime, and conformance behavior remains owned by the matching downstream layer unless explicitly baselined later. | role review / inspection | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| KILN-UNK-001 | Smallest declared record shape is not yet chosen. | Blocks schema, fixture, and CLI design. | discovery in Interfaces / Design | KILN |
| KILN-UNK-002 | First implementation language/package shape is not accepted. | Blocks crate/module/CLI work. | defer to Architecture, Interfaces, Code Rigor, Implementation Plan | KILN |
| KILN-UNK-003 | RUNE intake timing is undecided. | Affects first declared input classes and examples. | defer until after record shape is controlled | KILN / RUNE boundary |
| KILN-UNK-004 | DEPOT, WARDEN, CAL, and GAUGE repo existence/timing is not settled. | Handoff contracts may need placeholders before real upstream/downstream repos exist. | accept risk with explicit boundary specs | KILN |
| KILN-UNK-005 | Existing README/Product Plan/wave text contains pre-VTRACE implementation language. | Could mislead future agents into treating crates/CLI as accepted. | revise before implementation planning | KILN |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| KILN-REQ-001 | KILN-SPEC-001; KILN-SPEC-003; KILN-SPEC-011 | covered | Product role plus first checkable contract. |
| KILN-REQ-002 | KILN-SPEC-001; KILN-SPEC-012 | covered | Public/product-neutral and enterprise dependency posture. |
| KILN-REQ-003 | KILN-SPEC-002; KILN-SPEC-003; KILN-SPEC-011 | covered | Side-effect-free declaration checks. |
| KILN-REQ-004 | KILN-SPEC-004; KILN-SPEC-011 | covered | Diagnostics and fixture expectations. |
| KILN-REQ-005 | KILN-SPEC-003; KILN-SPEC-005 | covered | Readiness/degraded states and handoff record. |
| KILN-REQ-006 | KILN-SPEC-005 | covered | Runtime handoff record. |
| KILN-REQ-007 | KILN-SPEC-006 | covered | Policy needs, no authorization. |
| KILN-REQ-008 | KILN-SPEC-007 | covered | Package metadata, no registry mutation. |
| KILN-REQ-009 | KILN-SPEC-008 | covered | CAL reference boundary. |
| KILN-REQ-010 | KILN-SPEC-009; KILN-SPEC-012 | covered | Enterprise display adapter without core dependency. |
| KILN-REQ-011 | KILN-SPEC-004; KILN-SPEC-010; KILN-SPEC-011 | covered | Missing evidence visible in diagnostics and trace. |
| KILN-REQ-012 | KILN-SPEC-010 | covered | VTRACE trace continuity. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| KILN-SPEC-001 | VER-KILN-001 / inspection | KILN docs state public build/check/package contract and public dependency posture. | `docs/vtrace/REVIEW.md` planned | planned |
| KILN-SPEC-002 | VER-KILN-002 / fixture tests + inspection | Foundation operation has no external effects. | `docs/vtrace/VERIFICATION.md` planned | planned |
| KILN-SPEC-003 | VER-KILN-003 / fixture tests | Valid, not-ready, and degraded declarations produce deterministic status. | future fixture evidence | planned |
| KILN-SPEC-004 | VER-KILN-004 / negative fixture tests | Missing/ambiguous/evidence gaps produce explicit diagnostics. | future fixture evidence | planned |
| KILN-SPEC-005 | VER-KILN-005 / record inspection | Build record identifies checked inputs, unresolved gates, and handoff consumers. | future build-record fixture | planned |
| KILN-SPEC-006 | VER-KILN-006 / policy negative fixture | KILN does not emit authorization success. | future policy fixture | planned |
| KILN-SPEC-007 | VER-KILN-007 / package negative fixture | KILN does not publish, sign, or mutate registry state. | future package fixture | planned |
| KILN-SPEC-008 | VER-KILN-008 / boundary review | KILN references CAL primitives without defining semantics. | future interface review | planned |
| KILN-SPEC-009 | VER-KILN-009 / adapter demonstration | Enterprise display consumes public evidence without core dependency. | future adapter fixture | planned |
| KILN-SPEC-010 | VER-KILN-010 / trace review | Requirements map through specs and later evidence. | `docs/vtrace/TRACE.md` planned | planned |
| KILN-SPEC-011 | VER-KILN-011 / fixture inventory | First slice includes valid, missing-field, and degraded examples. | future fixture inventory | planned |
| KILN-SPEC-012 | VER-KILN-012 / dependency inspection | KILN core has no enterprise-only dependencies. | future dependency evidence | planned |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Baseline must separate current scaffold text from accepted target behavior. | Addressed by Specification Sources and assumptions. |
| Requirements Traceability Auditor | Every accepted requirement must map to a spec item or explicit deferral. | Addressed by Requirement-To-Spec Coverage. |
| Verification and Validation Lead | Specs need planned verification without claiming evidence already exists. | Addressed by Spec-To-Verification Coverage as planned evidence. |
| Software Assurance Guardian | Baseline must not authorize code before architecture, interfaces, code rigor, implementation plan, and work packages. | Addressed by Scope and gate decision. |
| Security Privacy Guardian | Side-effect-free operation, policy handoff, package handoff, and dependency posture must be controlled. | Addressed by KILN-SPEC-002, KILN-SPEC-006, KILN-SPEC-007, KILN-SPEC-012, and nonfunctional constraints. |
| Source Custody Counsel | Public/enterprise and analogy boundaries must not become endorsement or dependency claims. | Addressed by KILN-SPEC-001, KILN-SPEC-009, and KILN-SPEC-012. |
| Repo Maintainer | Baseline should identify what can be implemented first without absorbing adjacent systems. | Addressed by KILN-SPEC-011 and package/language allocation. |
| Future Agent | Baseline needs stable IDs and unknowns for resumption. | Addressed by `KILN-SPEC-*`, `KILN-PC-*`, `KILN-NF-*`, and `KILN-UNK-*` IDs. |

## Specification Gate

Decision: pass_with_risk.

Required before implementation planning:

- [x] Every accepted `KILN-REQ-*` maps to one or more `KILN-SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `KILN-SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [x] Unknowns are resolved, blocked, deferred, or converted to discovery work.
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: The target specification baseline is coherent enough to proceed to
Architecture and Interfaces, but implementation remains blocked. The accepted
risk is that existing README/Product Plan/wave docs still contain pre-VTRACE
implementation language. That language must be reconciled before implementation
planning or work packages treat any crate, CLI, fixture, or validation command
as accepted behavior.

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `.roles/ROLE.md`
- `README.md`
- `PRODUCT_PLAN.md`
- `context/waves/PHASES.md`
- TRACKER `dependency-systems/managed-agent-platform-roadmap.md`
- VTRACE `docs/framework/vtrace-process.md`
