# KILN Mission

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Mission / Need |
| VTRACE adoption scope | Define why KILN exists before CONOPS, requirements, specification, architecture, interfaces, code rigor, implementation planning, or code work. |
| Stage status | Role-reviewed fixed point for Mission; implementation remains blocked until later VTRACE work packages authorize code. |

This mission is the controlling VTRACE entry point for KILN. Earlier
pre-VTRACE scaffold text may describe intended implementation directions, but it
is not an accepted implementation baseline until later VTRACE stages settle the
requirements, specification baseline, interfaces, code-rigor posture, and work
packages.

## Mission Need

KILN exists because managed-agent capabilities need a public, repeatable
build/check/package layer before they are trusted by runtimes, registries,
policy engines, review clients, or enterprise products.

Without KILN, each agent runtime or product can invent its own way to declare
inputs, policy needs, tool surfaces, context obligations, package metadata,
validation gates, diagnostics, and runtime handoff records. That fragmentation
makes agent work hard to review, hard to package, hard to replay, and hard to
adopt across public and enterprise repos.

KILN's mission is to define the public build-system contract for managed-agent
work:

```text
declared managed-agent capability inputs
  -> side-effect-free build/check decision
  -> evidence-bearing build record
  -> package, policy, runtime, and review handoff metadata
```

KILN should be public infrastructure like RUNE. Enterprise repos may consume
KILN records, but KILN must not depend on enterprise-only systems.

## Users

| User | Need | Success Signal |
|---|---|---|
| Public repo maintainers | A product-neutral way to declare and check managed-agent capabilities before implementation or runtime execution. | They can identify what KILN will check, what it will not own, and which later VTRACE stage must define each missing detail. |
| Runtime authors | A checked handoff record they can execute only after build, package, and policy gates are satisfied. | Runtime integration is described as a future consumer need, not claimed as present behavior. |
| Package/registry authors | Stable metadata that can become DEPOT-ready without making KILN own registry operation or trust service behavior. | Package handoff is a mission need with registry mutation explicitly out of scope. |
| Policy authors | Policy needs that can be handed to WARDEN without KILN making authorization decisions. | Policy decision ownership is separated from build/check ownership. |
| CAL authors | A clear boundary between common agent primitives and KILN build records. | Common library scope is assigned to CAL, not absorbed into KILN. |
| Enterprise review clients | A public build/check record they can display or review without KILN depending on enterprise-only repos. | FLETCHER or Workbench-style usage is listed as future consumption, not a KILN dependency. |
| Future agents | Stable mission, trace, review, and evidence records that let work resume without chat history. | Missing evidence and deferred stages are named in VTRACE artifacts. |

## Operating Context

KILN is intended to sit in the public Standards & Protocols area of the
portfolio. Its operating context is managed-agent platform work where a
capability must be declared, checked, packaged, reviewed, and handed off before
runtime execution.

KILN is adjacent to, but distinct from:

| Neighbor | Relationship |
|---|---|
| RUNE | Upstream public contract descriptors that KILN may later accept as declared inputs. |
| CAL | Planned public Common Agent Library for reusable primitives that KILN should reference, not own. |
| WARDEN | Planned policy decision layer; KILN may emit policy needs but must not make authorization decisions. |
| DEPOT | Planned package/capability registry layer; KILN may emit package metadata but must not operate a registry. |
| GAUGE | Planned conformance and bakeoff layer; KILN should eventually produce evidence GAUGE can test. |
| ARCADE | Runtime consumer candidate; KILN should not execute workflows itself. |
| BAKER | Enterprise incubation umbrella that may cite KILN as the public build/check layer. |
| LATTICE / FLETCHER | Enterprise consumers or review surfaces; KILN must not depend on them. |

## Constraints

- KILN is public infrastructure and must remain product-neutral.
- KILN must not depend on enterprise-only repos or private product surfaces.
- Foundation scope is side-effect-free: no provider calls, network calls,
  registry mutation, package publication, product writes, or runtime execution.
- Implementation must wait for later VTRACE stages: CONOPS, requirements,
  specification baseline, architecture, interfaces, code rigor, implementation
  plan, and work packages.
- KILN must not absorb CAL, WARDEN, DEPOT, GAUGE, ARCADE, Workbench, or
  enterprise connector responsibilities.
- .NET, MSBuild, NuGet, and related names are analogies only, not affiliation,
  endorsement, or source authority.
- Missing evidence must be recorded as a gap or later-stage obligation, not
  filled with claims.

## Non-Goals

- No agent runtime.
- No common agent standard library.
- No policy decision engine.
- No package registry or trust service.
- No conformance test runner.
- No enterprise product surface.
- No tenant, M365, GitHub, Azure, SQL, ticketing, identity, or proprietary
  connector implementation.
- No provider SDK.
- No product-specific workflow syntax.
- No implementation claim until VTRACE work packages authorize code.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| KILN has a clear public mission distinct from enterprise incubation. | Role review by Systems Engineering Steward and Source Custody Counsel. | This file, `.roles/ROLE.md`, TRACKER managed-agent roadmap. |
| KILN's build/check/package boundary is distinct from CAL, WARDEN, DEPOT, GAUGE, ARCADE, Workbench, LATTICE, and FLETCHER. | Role review by Systems Engineering Steward, Repo Maintainer, and Future Agent. | This file's Operating Context, Constraints, and Non-Goals sections. |
| Mission claims are phrased as needs and future evidence obligations, not implementation readiness. | Role review by Verification and Validation Lead and Software Assurance Guardian. | This file's Scope, Constraints, and Success Criteria sections. |
| Candidate users and consumers are named without claiming adoption. | Role review by Requirements Traceability Auditor. | This file's Users and Operating Context sections. |
| The next VTRACE stage is clear. | Inspection against VTRACE process order. | Next stage: `docs/vtrace/CONOPS.md`. |

## First Validation Scenarios

These are mission-level validation scenarios only. They do not authorize code.

| Scenario | Mission Question | Later VTRACE Stage |
|---|---|---|
| Public build record | Can a public repo declare a managed-agent capability without depending on enterprise-only systems? | CONOPS, Requirements, Specification Baseline |
| Runtime handoff | Can a runtime identify what was checked before execution? | CONOPS, Interfaces, Verification |
| Policy handoff | Can KILN describe policy needs without deciding authorization? | Requirements, Architecture, Interfaces |
| Package handoff | Can KILN describe package metadata without operating a registry? | Requirements, Interfaces, DEPOT boundary work package |
| CAL boundary | Can KILN reference common primitives without becoming the common library? | CONOPS, Requirements, CAL boundary work package |
| Review client display | Can an enterprise review client display KILN evidence without becoming a KILN dependency? | CONOPS, Interfaces, Validation |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Mission should define the build/check/package decision and avoid process theater. | Addressed by mission flow, constraints, and non-goals. |
| Requirements Traceability Auditor | Candidate consumers must not become untraceable adoption claims. | Addressed by labeling consumers as needs and future stages. |
| Verification and Validation Lead | Mission must separate built-correctly evidence from right-thing validation. | Addressed by Success Criteria and First Validation Scenarios. |
| Software Assurance Guardian | Premature implementation must not be treated as baselined. | Addressed by Scope and Constraints; code requires later work packages. |
| Security Privacy Guardian | Build/package metadata must not imply hidden execution, ambient authority, or registry trust. | Addressed by side-effect-free constraint and WARDEN/DEPOT separation. |
| Source Custody Counsel | .NET/MSBuild/NuGet framing must remain analogy, not affiliation or endorsement. | Addressed by Constraints. |
| Repo Maintainer | First slice must be practical and identify the next evidence gap. | Addressed by limiting this stage to Mission and naming CONOPS next. |
| Future Agent | Future work must be resumable without chat history. | Addressed by stage status, next stage, and review ledger. |

## Deferred Findings

| Finding | Deferred To | Trigger |
|---|---|---|
| `README.md` and `PRODUCT_PLAN.md` still contain pre-VTRACE implementation direction, including planned crate and CLI language. | Specification Baseline / Communications Strategy | Before treating any implementation surface, CLI, crate, or validation command as accepted KILN behavior. |
| KILN needs stable mission/need IDs before requirements and trace rows are created. | Requirements / Trace | When drafting `docs/vtrace/REQUIREMENTS.md` and `docs/vtrace/TRACE.md`. |
| CAL, WARDEN, DEPOT, and GAUGE boundaries need sharper handoff definitions. | CONOPS / Architecture / Interfaces | Before KILN claims package, policy, common-library, runtime, or conformance integration. |

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `.roles/ROLE.md`
- `context/waves/PHASES.md`
- TRACKER `dependency-systems/managed-agent-platform-roadmap.md`
- VTRACE `docs/framework/vtrace-process.md`
