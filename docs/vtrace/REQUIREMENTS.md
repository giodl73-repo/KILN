# KILN Requirements

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Requirements |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md` |
| Stage status | Role-reviewed fixed point for Requirements; implementation remains blocked until specification, interfaces, code rigor, implementation plan, and work packages authorize code. |

These requirements convert KILN's mission and CONOPS into testable statements.
They do not authorize implementation. Exact record fields, schemas, interfaces,
crate surfaces, CLI commands, diagnostics, fixtures, and validation commands are
controlled by later VTRACE stages.

## Parent Need and Scenario Anchors

| ID | Source | Summary |
|---|---|---|
| KILN-NEED-001 | Mission | Managed-agent capabilities need a public, repeatable build/check/package layer before runtime, registry, policy, review, or enterprise consumption. |
| KILN-NEED-002 | Mission | KILN must remain public, product-neutral infrastructure and must not depend on enterprise-only systems. |
| KILN-CONOPS-001 | CONOPS Scenario 1 | Public build-record declaration. |
| KILN-CONOPS-002 | CONOPS Scenario 2 | Runtime handoff. |
| KILN-CONOPS-003 | CONOPS Scenario 3 | Policy handoff. |
| KILN-CONOPS-004 | CONOPS Scenario 4 | Package or registry handoff. |
| KILN-CONOPS-005 | CONOPS Scenario 5 | CAL boundary check. |
| KILN-CONOPS-006 | CONOPS Scenario 6 | Enterprise review display. |

## Requirement Table

| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| KILN-REQ-001 | KILN shall define a public build/check/package contract for managed-agent capabilities. | KILN-NEED-001 | This is the core mission and prevents every runtime/product from inventing a private build gate. | must | KILN | inspection / review | accepted |
| KILN-REQ-002 | KILN shall remain product-neutral and shall not require enterprise-only dependencies. | KILN-NEED-002 | Public adoption requires the core contract to stand outside LATTICE, WITNESS, BAKER incubation records, or enterprise connectors. | must | KILN | inspection / review | accepted |
| KILN-REQ-003 | KILN shall check declared capability inputs without provider calls, network calls, product writes, runtime execution, registry mutation, package publication, or policy authorization. | KILN-NEED-001; KILN-CONOPS-001 | The foundation operation must be side-effect-free before it can become trusted infrastructure. | must | KILN | test / inspection | accepted |
| KILN-REQ-004 | KILN shall produce diagnostics when required declaration data, references, gates, or evidence obligations are missing or ambiguous. | KILN-CONOPS-001 | Authors and maintainers need actionable failure modes rather than hidden assumptions. | must | KILN | test | accepted |
| KILN-REQ-005 | KILN shall distinguish ready, not-ready, and degraded handoff states for declared managed-agent capabilities. | KILN-CONOPS-001; KILN-CONOPS-002 | Runtime, registry, policy, conformance, and review consumers need deterministic status, not prose-only readiness. | must | KILN | test / analysis | accepted |
| KILN-REQ-006 | KILN shall record what was checked and what remains gated before a runtime consumes a handoff. | KILN-CONOPS-002 | Runtime execution must not infer unproven build, policy, package, or compatibility claims. | must | KILN | test / inspection | accepted |
| KILN-REQ-007 | KILN shall describe policy needs as declaration data but shall not decide authorization. | KILN-CONOPS-003 | Policy decision ownership belongs to WARDEN or another policy system; KILN owns declaration completeness only. | must | KILN | inspection / test | accepted |
| KILN-REQ-008 | KILN shall describe package metadata and publication intent without mutating registries, signing trust, or publishing packages. | KILN-CONOPS-004 | DEPOT or another registry owns trust and distribution; KILN only prepares package handoff data. | must | KILN | inspection / test | accepted |
| KILN-REQ-009 | KILN shall reference common agent primitives without defining their semantics inside KILN core. | KILN-CONOPS-005 | CAL must own reusable agent primitives; KILN should not become the Base Class Library. | must | KILN / CAL boundary | inspection / review | accepted |
| KILN-REQ-010 | KILN shall allow enterprise review clients to consume public KILN evidence without adding enterprise-only fields or dependencies to KILN core. | KILN-CONOPS-006 | WITNESS or Workbench-style displays should consume KILN records through adapters while preserving public core boundaries. | should | KILN / enterprise adapter boundary | inspection / demonstration | accepted |
| KILN-REQ-011 | KILN shall make missing evidence explicit as a gap, deferred requirement, degraded status, or later-stage obligation. | KILN-NEED-001; KILN-CONOPS-001 | VTRACE requires missing proof to remain visible instead of becoming implied readiness. | must | KILN | inspection / review | accepted |
| KILN-REQ-012 | KILN shall preserve traceability from mission needs and CONOPS scenarios to requirements, specifications, interfaces, verification, validation, and evidence. | KILN-NEED-001 | Future agents and maintainers need resumable engineering records rather than chat-dependent intent. | must | KILN | inspection / trace review | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Verification Method Notes

| Method | Meaning for KILN |
|---|---|
| inspection | Review controlled docs, schemas, records, or source surfaces for required content and prohibited scope. |
| review | Apply `.roles` fixed-point review to determine whether a requirement is coherent, owned, and traceable. |
| analysis | Compare states, boundaries, or compatibility rules without requiring executable code. |
| test | Execute future fixtures or validators once implementation work packages authorize code. |
| demonstration | Show future downstream consumption without claiming KILN owns the downstream system. |
| trace review | Inspect `TRACE.md` rows for parent need, requirement, spec, interface/design, verification, validation, and evidence links. |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| KILN-DREQ-001 | Exact KILN record fields, schema names, compatibility rules, and output formats belong in the specification baseline and interfaces, not requirements prose. | Drafting `docs/vtrace/SPECIFICATION_BASELINE.md` and `docs/vtrace/INTERFACES.md`. |
| KILN-DREQ-002 | First CLI commands, crate names, fixture paths, and validation commands are implementation/interface choices that remain pre-baseline. | Drafting specification baseline, interfaces, code rigor, implementation plan, and work packages. |
| KILN-DREQ-003 | RUNE descriptor intake is a likely early integration but should not be required until the public build-record baseline says how declared inputs work. | After specification baseline defines declared input classes. |
| KILN-DREQ-004 | CAL, WARDEN, DEPOT, and GAUGE handoff contracts need separate interface and architecture requirements before KILN claims integration readiness. | Drafting architecture and interfaces. |
| KILN-DREQ-005 | Security, supply-chain, signing, registry trust, and provenance depth require policy/package interface detail before they can become complete executable requirements. | Drafting interfaces, package boundary notes, and verification plan. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Requirements should define KILN's build/check/package obligation without turning it into runtime, policy, registry, or library scope. | Addressed by KILN-REQ-001 through KILN-REQ-010 and deferred boundary requirements. |
| Requirements Traceability Auditor | Requirements need stable IDs and parent links from Mission and CONOPS. | Addressed by KILN-NEED, KILN-CONOPS, KILN-REQ, and KILN-DREQ IDs. |
| Verification and Validation Lead | Requirements must name verification methods without overstating current executable evidence. | Addressed by method notes and deferral of tests until work packages authorize code. |
| Software Assurance Guardian | Requirements must not authorize premature code, CLI, crate, or fixture implementation. | Addressed by scope statement and KILN-DREQ-002. |
| Security Privacy Guardian | Requirements must preserve side-effect-free operation and avoid ambient authority or registry trust claims. | Addressed by KILN-REQ-003, KILN-REQ-007, KILN-REQ-008, and KILN-DREQ-005. |
| Source Custody Counsel | Requirements should not turn analogies or enterprise product names into endorsement or dependency claims. | Addressed by KILN-REQ-002 and KILN-REQ-010. |
| Repo Maintainer | Requirements should be small enough for incremental adoption and review. | Addressed by limiting must requirements to mission-critical boundaries and deferring implementation detail. |
| Future Agent | Requirements should be searchable and traceable from later stages. | Addressed by stable IDs and explicit deferred requirements. |

## Fixed-Point Decision

Decision: pass.

The requirements stage is settled for the current VTRACE slice because:

- no role has a critical or major actionable finding against the requirement set;
- deferred requirements have named later stages and triggers;
- implementation remains blocked until specification, interfaces, code rigor,
  implementation plan, and work packages settle.

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `.roles/ROLE.md`
- TRACKER `dependency-systems/managed-agent-platform-roadmap.md`
- VTRACE `docs/framework/vtrace-process.md`
