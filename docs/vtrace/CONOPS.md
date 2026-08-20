# KILN Concept of Operations

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | CONOPS |
| Parent stage | `docs/vtrace/MISSION.md` |
| Stage status | Role-reviewed fixed point for CONOPS; implementation remains blocked until later VTRACE work packages authorize code. |

This CONOPS describes how KILN is intended to be used in managed-agent
workflows. It does not authorize implementation. Requirements, specification
baseline, architecture, interfaces, code rigor, implementation plan, and work
packages must settle before any code work is treated as accepted KILN behavior.

## Operational Summary

KILN is the public build/check/package layer for managed-agent capabilities.
An operator or tool presents declared capability inputs to KILN. KILN checks the
declaration without side effects, reports diagnostics, and emits or prepares an
evidence-bearing build record for later handoff to package, policy, runtime,
conformance, or review systems.

```text
actor declares capability
  -> KILN checks declared inputs and gates
  -> KILN reports build decision and evidence obligations
  -> downstream system consumes the checked record
```

KILN's operational center is the check decision. KILN does not execute the
agent, decide authorization, operate a registry, provide common agent library
functions, or display an enterprise review experience.

## Actors

| Actor | Responsibility | Needs |
|---|---|---|
| Capability author | Declares managed-agent capability inputs, outputs, policy needs, package metadata, and intended runtime/review handoffs. | Clear diagnostics and a stable way to know whether the declaration is ready for later systems. |
| Repo maintainer | Decides whether the repo may adopt KILN and which KILN evidence is required before implementation or runtime use. | Product-neutral checks, no enterprise-only dependency, and explicit non-goals. |
| Runtime operator | Consumes a checked KILN handoff before executing managed-agent work in ARCADE-style runtimes or future hosts. | A deterministic record of what was checked and what remains gated. |
| Policy owner | Reviews policy needs emitted by KILN and decides authorization in WARDEN or another policy system. | A clear separation between KILN's declaration checking and policy decision authority. |
| Package/registry owner | Uses KILN metadata as input to DEPOT-style package or capability publication. | Package metadata that is checkable without KILN mutating registries or signing trust decisions. |
| CAL owner | Defines common agent primitives that KILN declarations may reference later. | KILN must not hard-code or duplicate common library semantics. |
| Conformance reviewer | Uses KILN records as evidence for GAUGE-style bakeoffs or compatibility checks. | Stable pass/fail/degraded records and traceable fixture expectations. |
| Enterprise review client | Displays checked records and evidence in WITNESS or Workbench-style surfaces. | Public KILN records that can be consumed without making KILN depend on enterprise repos. |
| Future agent | Resumes KILN work from VTRACE docs, review ledgers, and evidence pointers. | Stable stage records, deferred findings, and clear next-stage triggers. |

## Scenarios

### Scenario 1: Public build-record declaration

| Field | Description |
|---|---|
| Trigger | A public repo wants to declare a managed-agent capability before implementation or runtime execution. |
| Inputs | Capability identity, version, declared source files/specs, expected outputs, policy needs, package metadata, validation expectations, and intended downstream handoffs. |
| Normal path | The author prepares a KILN declaration; KILN checks that required sections and references are present; KILN reports a side-effect-free build decision and evidence obligations. |
| Failure or degraded path | Required fields, references, or gates are missing; KILN reports diagnostics and does not produce a ready handoff. |
| Outputs | Build decision, diagnostics, evidence obligations, and a candidate build-record shape for later specification. |
| Handoffs | Requirements and Specification Baseline define exact fields; Verification defines objective evidence. |
| Validation evidence | Future fixture showing valid, missing-field, and degraded declarations. |

### Scenario 2: Runtime handoff

| Field | Description |
|---|---|
| Trigger | An ARCADE-style runtime or future host is asked to execute a managed-agent capability. |
| Inputs | Checked KILN record, runtime target, declared inputs/outputs, policy gate status, package identity, and compatibility metadata. |
| Normal path | Runtime inspects the KILN record, confirms the build/check gate passed, confirms unresolved policy/package gates are not hidden, and only then proceeds according to its own execution rules. |
| Failure or degraded path | KILN record is missing, stale, incompatible, or explicitly degraded; runtime refuses execution or routes to review according to its own policy. |
| Outputs | Runtime-ready handoff status, not runtime execution performed by KILN. |
| Handoffs | Architecture and Interfaces define handoff ownership; ARCADE or another runtime owns execution. |
| Validation evidence | Future mock-runtime scenario proving KILN can be consumed without executing work itself. |

### Scenario 3: Policy handoff

| Field | Description |
|---|---|
| Trigger | A capability declaration includes data access, tool authority, network, write, provider, budget, approval, or publication needs. |
| Inputs | Policy needs declared in the KILN record and any relevant capability metadata. |
| Normal path | KILN checks that policy needs are declared and handoff-ready; WARDEN or another policy engine decides authorization later. |
| Failure or degraded path | Policy needs are implicit, ambiguous, or missing; KILN marks the record not ready for policy handoff. |
| Outputs | Policy-needs summary and diagnostics, not an authorization decision. |
| Handoffs | WARDEN owns policy decisions; KILN owns declaration completeness. |
| Validation evidence | Future negative fixtures for hidden authority, undeclared write access, and ambiguous approval gates. |

### Scenario 4: Package or registry handoff

| Field | Description |
|---|---|
| Trigger | A capability is ready to become a DEPOT-style package, capability pack, or registry candidate. |
| Inputs | Package identity, version, source/build record links, compatibility metadata, declared outputs, evidence pointers, and publication intent. |
| Normal path | KILN checks package metadata for completeness and emits a DEPOT-ready handoff shape. DEPOT or another registry service handles signing, publication, trust, and distribution. |
| Failure or degraded path | Package metadata is incomplete, mismatched, or claims unsupported trust; KILN reports diagnostics and blocks ready status. |
| Outputs | Package-metadata handoff and diagnostics, not registry mutation. |
| Handoffs | DEPOT owns registry protocol/service decisions; Source Custody review owns rights posture. |
| Validation evidence | Future fixture proving package metadata can be checked without publishing. |

### Scenario 5: CAL boundary check

| Field | Description |
|---|---|
| Trigger | A KILN declaration wants to reference common agent primitives such as files, tools, approvals, budgets, artifacts, receipts, or retries. |
| Inputs | Referenced primitive identifiers and version/compatibility expectations. |
| Normal path | KILN records that the capability references common primitives and leaves their semantics to CAL. |
| Failure or degraded path | KILN tries to define or duplicate common-library behavior; review marks the design as boundary creep. |
| Outputs | Boundary finding and future CAL integration requirement. |
| Handoffs | CAL owns primitive definitions; KILN owns build-record references to those definitions. |
| Validation evidence | Future boundary fixture showing KILN rejects or flags undeclared/nonexistent primitive references without implementing them. |

### Scenario 6: Enterprise review display

| Field | Description |
|---|---|
| Trigger | WITNESS, Workbench, or another enterprise review client wants to display KILN evidence for a managed-agent capability. |
| Inputs | Public KILN build record, diagnostics, evidence pointers, status, and unresolved gates. |
| Normal path | Review client displays KILN-owned evidence and preserves the boundary that KILN is public infrastructure while the client is enterprise/product surface. |
| Failure or degraded path | Review client requires private-only fields or product-specific vocabulary in KILN core; integration is rejected or moved to an adapter. |
| Outputs | Display-ready evidence shape and boundary findings. |
| Handoffs | Review client owns UI and workflow; KILN owns public evidence record semantics. |
| Validation evidence | Future adapter/display fixture that consumes KILN output without adding enterprise dependency to KILN. |

## Operational Assumptions

- KILN starts from declared records and fixtures before broad integration.
- KILN checks declarations and evidence obligations, not runtime effects.
- KILN can be adopted by public repos without enterprise-only dependencies.
- KILN may reference planned layers such as CAL, WARDEN, DEPOT, and GAUGE, but
  those systems own their own semantics and readiness claims.
- Downstream consumers may reject a KILN record even after KILN passes it if
  their own policy, runtime, package, or validation gates fail.
- Existing README, product-plan, wave, and pulse language that mentions crates,
  CLI commands, or implementation remains pre-baseline until later VTRACE stages
  classify it.

## Operational Constraints

- No provider calls.
- No network calls.
- No product writes.
- No runtime execution.
- No registry mutation.
- No package publication.
- No policy authorization decision.
- No enterprise-only dependency.
- No common-library implementation inside KILN.

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | CONOPS must describe real operating workflows without turning KILN into every managed-agent layer. | Addressed by actor/scenario handoffs and explicit non-boundaries. |
| Requirements Traceability Auditor | Scenarios must become traceable parents for requirements without claiming implementation exists. | Addressed by scenario names, inputs, outputs, handoffs, and future evidence pointers. |
| Verification and Validation Lead | Each scenario needs future evidence that separates KILN verification from downstream validation. | Addressed by scenario validation-evidence rows and downstream ownership. |
| Software Assurance Guardian | CONOPS must not authorize code before code-rigor and work packages. | Addressed by Scope, Operational Assumptions, and Constraints. |
| Security Privacy Guardian | Policy/package scenarios must not create ambient authority, hidden execution, or registry trust. | Addressed by policy and package handoff boundaries. |
| Source Custody Counsel | Enterprise and .NET-style analogies must remain boundary descriptions, not endorsement or dependency claims. | Addressed by public/enterprise separation and no-affiliation framing from Mission. |
| Repo Maintainer | The first operational scenarios must be adoptable incrementally. | Addressed by fixture-first assumptions and clear degraded paths. |
| Future Agent | Future requirements should be derivable from CONOPS without chat history. | Addressed by named scenarios and deferred findings. |

## Deferred Findings

| Finding | Deferred To | Trigger |
|---|---|---|
| Scenario IDs need stable identifiers before requirements and trace rows are written. | Requirements / Trace | When drafting `docs/vtrace/REQUIREMENTS.md` and `docs/vtrace/TRACE.md`. |
| Exact KILN record fields, diagnostics, compatibility rules, and output schemas are not yet controlled. | Specification Baseline / Interfaces | Before any fixture, CLI, crate, or package handoff is accepted. |
| Existing wave and pulse docs still describe pre-VTRACE Rust workspace and CLI work. | Specification Baseline / Implementation Plan | Before implementation planning or any claim that KILN has accepted code scope. |
| CAL, WARDEN, DEPOT, and GAUGE need explicit interface boundaries. | Architecture / Interfaces | Before KILN claims handoff readiness with those layers. |

## Open Questions

- What is the smallest declared record shape that proves KILN's value without
  becoming a product-specific workflow language?
- Should KILN's first fixture reference RUNE descriptors, or should RUNE intake
  wait until the second controlled slice?
- Which degraded states should be first-class: missing field, stale input,
  incompatible version, unresolved policy, package-not-ready, runtime-not-ready,
  or evidence-missing?
- What evidence format should GAUGE or PROOF consume from KILN first?

## Source Links

- `docs/vtrace/MISSION.md`
- `.roles/ROLE.md`
- `README.md`
- `PRODUCT_PLAN.md`
- TRACKER `dependency-systems/managed-agent-platform-roadmap.md`
- VTRACE `docs/framework/vtrace-process.md`
