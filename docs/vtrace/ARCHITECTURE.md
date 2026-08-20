# KILN Architecture

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Architecture |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md` |
| Stage status | Role-reviewed fixed point for Architecture; implementation remains blocked until Interfaces, Code Rigor, Implementation Plan, and Work Packages authorize code. |

This architecture defines KILN's controlled system shape before interfaces,
code rigor, implementation planning, work packages, or code. It names future
components and boundaries so implementation can be planned without absorbing
runtime, policy, registry, common-library, conformance, or enterprise product
responsibilities.

## Architecture Summary

KILN is a public build/check/package contract with a layered architecture:

```text
declared capability record
  -> declaration model
  -> side-effect-free checker
  -> diagnostics/status/evidence obligations
  -> build record
  -> boundary handoffs
       -> policy needs
       -> package metadata
       -> CAL primitive references
       -> runtime readiness
       -> conformance evidence
       -> enterprise review display
```

The center of KILN is not execution. The center is a deterministic check over a
declared managed-agent capability. KILN's first implementation should prove the
core loop with fixtures before integrating with RUNE, CAL, WARDEN, DEPOT, GAUGE,
ARCADE, WITNESS, or Workbench-style consumers.

## Components

| Component | Boundary ID | Responsibility | Requirement IDs | Interfaces | Evidence |
|---|---|---|---|---|---|
| VTRACE control package | KILN-ARCH-001 | Own mission, CONOPS, requirements, specification baseline, architecture, interfaces, verification, validation, trace, and review records. | KILN-REQ-011; KILN-REQ-012 | Markdown VTRACE docs | Role review, trace review |
| Declaration model | KILN-ARCH-002 | Represent declared capability identity, inputs, expected outputs, gates, evidence obligations, package metadata, policy needs, and handoff targets. | KILN-REQ-001; KILN-REQ-003; KILN-REQ-005 | Future record/schema interface | Fixture/schema inspection |
| Check engine | KILN-ARCH-003 | Evaluate declared records for completeness, readiness, degraded states, and prohibited side effects. | KILN-REQ-003; KILN-REQ-004; KILN-REQ-005 | Future library/CLI boundary | Fixture tests |
| Diagnostic model | KILN-ARCH-004 | Report missing, ambiguous, unsupported, incompatible, or unproven declaration/evidence conditions without silent success. | KILN-REQ-004; KILN-REQ-011 | Future diagnostic record | Negative fixture tests |
| Build record emitter | KILN-ARCH-005 | Produce evidence-bearing records that state what was checked, what remains gated, and which handoffs are possible. | KILN-REQ-005; KILN-REQ-006; KILN-REQ-012 | Future build-record file/schema | Record inspection |
| Policy handoff boundary | KILN-ARCH-006 | Carry policy needs to WARDEN or another policy system without deciding authorization. | KILN-REQ-007 | Future policy-needs section/interface | Negative policy fixture |
| Package handoff boundary | KILN-ARCH-007 | Carry package metadata and publication intent to DEPOT or another registry without publishing, signing, or mutating registry state. | KILN-REQ-008 | Future package metadata section/interface | Negative package fixture |
| CAL reference boundary | KILN-ARCH-008 | Reference common agent primitive identifiers while CAL owns primitive semantics and compatibility. | KILN-REQ-009 | Future primitive-reference section/interface | Boundary review |
| Runtime handoff boundary | KILN-ARCH-009 | Tell runtimes what was checked and what remains gated before runtime execution. | KILN-REQ-006 | Future runtime-readiness section/interface | Mock-runtime fixture |
| Enterprise review adapter boundary | KILN-ARCH-010 | Allow enterprise clients to display public KILN evidence without adding enterprise-only dependencies to KILN core. | KILN-REQ-010; KILN-REQ-002 | Future adapter/display record | Dependency inspection |
| Conformance evidence boundary | KILN-ARCH-011 | Provide records and fixtures that GAUGE/PROOF-style tools can inspect later without KILN owning those systems. | KILN-REQ-011; KILN-REQ-012 | Future evidence manifest | Trace/verification review |

## Package / Language Boundaries

Detailed package boundaries belong in a later `PACKAGE_BOUNDARIES.md` if risk
requires it. Architecture-level boundaries are:

| Boundary ID | Package / Crate / Module | Language | Responsibility | Allowed Dependencies |
|---|---|---|---|---|
| KILN-ARCH-001 | `docs/vtrace` | Markdown | Controlled VTRACE records and fixed-point review ledgers. | VTRACE process references only. |
| KILN-ARCH-002 | future core model package | TBD | Product-neutral declaration/build-record data model. | std-only or minimal public dependencies; no enterprise deps. |
| KILN-ARCH-003 | future checker package | TBD | Side-effect-free checks over declared records. | Core model; fixture/test support. |
| KILN-ARCH-004 | future diagnostics package/module | TBD | Diagnostic categories and reporting. | Core model. |
| KILN-ARCH-005 | future record emitter package/module | TBD | Serialize checked build records and evidence obligations. | Core model, diagnostics. |
| KILN-ARCH-006 | future policy handoff adapter | TBD | Policy-needs metadata. | Public policy contract only; no authorization service dependency in core. |
| KILN-ARCH-007 | future package handoff adapter | TBD | Package metadata handoff. | Public package contract only; no registry service mutation. |
| KILN-ARCH-008 | future CAL reference adapter | TBD | Common primitive references. | Public CAL contract only after CAL exists/stabilizes. |
| KILN-ARCH-009 | future runtime handoff adapter | TBD | Runtime-readiness handoff. | Public runtime contract or fixture only; no runtime execution. |
| KILN-ARCH-010 | future enterprise display adapter | TBD | Optional downstream consumption for enterprise review clients. | Adapter may live outside KILN core if enterprise-specific. |
| KILN-ARCH-011 | future conformance/evidence fixtures | TBD | Evidence records for GAUGE/PROOF-style inspection. | Public fixture/schema dependencies only. |

## Data Flow

```text
author / tool
  -> declared capability record
       -> declaration model
            -> side-effect-free checker
                 -> diagnostics
                 -> readiness status
                 -> evidence obligations
                 -> checked build record
                      -> policy-needs handoff
                      -> package-metadata handoff
                      -> CAL-reference handoff
                      -> runtime-readiness handoff
                      -> conformance/review evidence handoff
```

### Prohibited Data Flow

```text
KILN core
  -x-> provider call
  -x-> network call
  -x-> product write
  -x-> runtime execution
  -x-> policy authorization
  -x-> registry mutation
  -x-> package publication
  -x-> enterprise-only dependency
```

## Dependencies

| Dependency | Purpose | Boundary / Risk | Verification |
|---|---|---|---|
| VTRACE process | Controls staged engineering and evidence records. | Method dependency only; KILN remains a separate repo. | Inspect VTRACE artifacts and review ledgers. |
| RUNE | Future descriptor input candidate. | Planned optional input; not required for foundation core. | Architecture/interface review before adoption. |
| CAL | Future common primitive catalog. | KILN references identifiers only; CAL owns semantics. | Boundary review and interface tests. |
| WARDEN | Future policy decision layer. | KILN emits policy needs only; no authorization. | Negative policy fixture. |
| DEPOT | Future package/registry layer. | KILN emits package metadata only; no publication/signing/registry mutation. | Negative package fixture. |
| GAUGE / PROOF | Future conformance/report consumers. | KILN emits inspectable evidence; does not own those tools. | Evidence fixture and trace review. |
| ARCADE / runtime hosts | Future runtime consumers. | KILN emits readiness handoff; no runtime execution. | Mock-runtime fixture. |
| WITNESS / Workbench | Future enterprise review clients. | Adapter/downstream only; no KILN core dependency. | Dependency inspection. |
| LATTICE / BAKER | Context/umbrella references. | Enterprise context only; no KILN runtime/build dependency. | Dependency inspection. |

## Failure Modes

| Failure Mode | Impact | Mitigation | Evidence |
|---|---|---|---|
| Missing required declaration data | Capability may appear ready without enough evidence. | Not-ready status and explicit diagnostics. | Future missing-field fixture. |
| Ambiguous handoff target | Runtime, policy, package, or review consumer may infer unsupported readiness. | Degraded/not-ready status; require explicit handoff metadata. | Future degraded fixture. |
| Hidden side effect | Build check could mutate products, call providers, publish packages, or grant authority. | Side-effect-free architecture and negative tests/inspection. | Future side-effect inspection and tests. |
| Enterprise dependency leak | Public KILN core becomes unusable outside private/product repos. | Dependency boundary and adapter separation. | Future dependency inspection. |
| CAL semantic creep | KILN becomes the common agent library instead of a build layer. | CAL reference boundary; reject/flag semantic implementation. | Boundary review. |
| Policy authority creep | KILN appears to authorize tool/data/network/write access. | Policy-needs-only boundary; no authorization-success claim. | Negative policy fixture. |
| Registry trust creep | KILN appears to publish, sign, or distribute packages. | Package-metadata-only boundary. | Negative package fixture. |
| Pre-VTRACE scaffold drift | README/Product Plan/wave docs imply accepted implementation before gates settle. | Reconcile docs before implementation planning. | Specification baseline risk and later doc patch. |
| Untraceable change | Future code cannot be tied back to mission/spec/evidence. | Stable IDs and trace review before work packages close. | `TRACE.md` planned. |

## Architecture Decisions

| Decision ID | Decision | Rationale | Consequence |
|---|---|---|---|
| KILN-ADR-001 | Keep KILN core public and product-neutral. | Satisfies KILN-REQ-002 and enables public adoption. | Enterprise-specific behavior must live in adapters or downstream clients. |
| KILN-ADR-002 | Center KILN on declaration checking, not execution. | Satisfies KILN-REQ-003 and prevents runtime/policy/registry creep. | ARCADE/hosts execute; WARDEN authorizes; DEPOT publishes. |
| KILN-ADR-003 | Use fixture-backed foundation proof before broad integrations. | Satisfies KILN-SPEC-011 and reduces premature dependency risk. | First implementation work packages should build valid/missing/degraded fixtures. |
| KILN-ADR-004 | Treat RUNE, CAL, WARDEN, DEPOT, GAUGE, ARCADE, and enterprise clients as boundaries. | Keeps implementation modular and traceable. | Interface docs must define handoffs before integration claims. |
| KILN-ADR-005 | Reconcile pre-VTRACE scaffold docs before implementation planning. | Avoids conflicting source-of-truth claims. | README/Product Plan/wave docs need cleanup before code work packages. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Architecture must show a real system shape without turning KILN into every managed-agent layer. | Addressed by layered components and boundary decisions. |
| Requirements Traceability Auditor | Components must map to accepted requirements and spec IDs. | Addressed by Components and package/language boundary tables. |
| Verification and Validation Lead | Data flow and failure modes must name future evidence without claiming it exists. | Addressed by future fixture/review evidence pointers. |
| Software Assurance Guardian | Architecture must not authorize code before interfaces, code rigor, implementation plan, and work packages. | Addressed by Scope and deferred package/language decisions. |
| Security Privacy Guardian | Architecture must block hidden side effects, policy authority, registry trust, and enterprise dependency leaks. | Addressed by prohibited data flow and failure modes. |
| Source Custody Counsel | Enterprise/product references must remain boundaries, not dependencies or endorsement claims. | Addressed by dependency table and adapter separation. |
| Repo Maintainer | Architecture should identify the first implementable slice. | Addressed by fixture-backed foundation proof decision. |
| Future Agent | Architecture needs stable IDs for later interfaces/design/work packages. | Addressed by `KILN-ARCH-*` and `KILN-ADR-*` IDs. |

## Open Risks

- Exact record fields and schemas remain for Interfaces.
- Concrete crate/module/language names remain for Interfaces, Code Rigor, and
  Implementation Plan.
- README/Product Plan/wave docs still need reconciliation before code work.
- RUNE intake timing remains undecided until the declared input model is
  controlled.
- CAL, WARDEN, DEPOT, and GAUGE may not exist yet as repos, so first interfaces
  may need placeholder public contracts.

## Architecture Gate

Decision: pass_with_risk.

Rationale: The architecture is coherent enough to proceed to Interfaces. The
accepted risk is that package/language choices are intentionally unresolved and
pre-VTRACE scaffold docs still contain implementation language. Implementation
remains blocked until Interfaces, Code Rigor, Implementation Plan, and Work
Packages settle and the scaffold docs are reconciled.

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `.roles/ROLE.md`
- TRACKER `dependency-systems/managed-agent-platform-roadmap.md`
- VTRACE `docs/framework/vtrace-process.md`
