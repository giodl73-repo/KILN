# KILN Validation Plan

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Validation |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/VERIFICATION.md` |
| Stage status | Role-reviewed fixed point |

This plan defines how KILN will validate that the foundation checker is the
right product for the stated mission and operating scenarios. Verification asks
whether the implementation satisfies controlled requirements. Validation asks
whether the resulting behavior is useful, bounded, and credible for the users
and handoffs described by Mission and CONOPS.

KILN-WP-008 records implementation evidence for the foundation slice. Scenario
results are accepted for local fixture/mock validation; real downstream CAL,
WARDEN, DEPOT, GAUGE, runtime, and enterprise integrations remain deferred.

## Executed Acceptance Summary

| Scenario | Result | Evidence |
|---|---|---|
| KILN-VAL-001 | pass | `valid=0`; missing/degraded fixtures produce diagnostics and exit `1`. |
| KILN-VAL-002 | pass_with_risk | Build record exposes checked inputs, status, diagnostics, gates, obligations, handoffs, and trace; no runtime execution. |
| KILN-VAL-003 | pass | Policy authorization claim fixture is `not_ready`; no authorization success. |
| KILN-VAL-004 | pass | Package publication/signing claim fixture is `not_ready`; no registry mutation. |
| KILN-VAL-005 | pass | CAL semantic-definition fixture is `not_ready`; KILN records identifiers only. |
| KILN-VAL-006 | pass_with_risk | Enterprise-required fixture is rejected from core; real UI adapter deferred. |
| KILN-VAL-007 | pass | Workspace is std-only; no enterprise/provider/runtime/registry/policy dependency. |
| KILN-VAL-008 | pass | VTRACE docs, work packages, evidence summaries, and commits make work resumable. |

## Validation Method Legend

| Method | Meaning for KILN |
|---|---|
| scenario review | Inspect whether the planned behavior satisfies a Mission or CONOPS scenario. |
| operator review | A repo maintainer or future agent can understand the decision and next action. |
| boundary review | Confirm KILN remains public/product-neutral and does not absorb adjacent systems. |
| fixture demonstration | Use retained local fixtures to demonstrate the scenario without downstream mutation. |
| record inspection | Inspect `kiln.build.json`, diagnostics, or evidence manifests for scenario usefulness. |
| trace review | Confirm the scenario has requirement, verification, evidence, and review coverage. |

## Validation Scenarios

| Scenario ID | Parent | User / Actor | Need | Workflow | Success Criteria | Evidence Pointer | Result |
|---|---|---|---|---|---|---|---|
| KILN-VAL-001 | KILN-NEED-001; KILN-CONOPS-001 | Public repo maintainer / capability author | Declare and check a managed-agent capability without enterprise-only systems. | Author prepares `kiln.yaml`; KILN checks required sections, readiness, diagnostics, and evidence obligations. | Maintainer can tell whether the declaration is ready, degraded, or not ready, and why. | KILN-EVID-VAL-001 | pending |
| KILN-VAL-002 | KILN-CONOPS-002 | Runtime operator | Know what was checked before any runtime executes. | Runtime receives a KILN build record and inspects status, checked inputs, unresolved gates, and handoffs. | Runtime can refuse or continue under its own rules without KILN executing anything. | KILN-EVID-VAL-002 | pending |
| KILN-VAL-003 | KILN-CONOPS-003 | Policy owner | See policy needs without KILN deciding authorization. | KILN reports policy-needs metadata and unresolved authority gates. | Policy owner sees no `authorized` success claim and can route to WARDEN or later policy work. | KILN-EVID-VAL-003 | pending |
| KILN-VAL-004 | KILN-CONOPS-004 | Package / registry owner | Use package metadata without KILN publishing or signing. | KILN checks package identity, version, publication intent, and trust claims. | Package owner receives complete metadata or explicit diagnostics; registry mutation is absent. | KILN-EVID-VAL-004 | pending |
| KILN-VAL-005 | KILN-CONOPS-005 | CAL owner / capability author | Reference common primitives without KILN becoming CAL. | Declaration names CAL primitive IDs or expectations; KILN records references only. | Primitive semantics remain out of KILN; invalid semantic definitions are flagged. | KILN-EVID-VAL-005 | pending |
| KILN-VAL-006 | KILN-CONOPS-006 | Enterprise review client / future agent | Display public KILN evidence without adding enterprise dependencies to KILN core. | Review client consumes public build record, diagnostics, evidence pointers, and unresolved gates. | Public core remains enterprise-neutral; product-specific fields move to adapter/downstream scope. | KILN-EVID-VAL-006 | pending |
| KILN-VAL-007 | KILN-NEED-002 | Public adopter / source custodian | Trust that KILN is public infrastructure, not a private product dependency. | Dependency and interface review inspect KILN core scope. | No enterprise-only repo, provider SDK, tenant data, private connector, or product-only vocabulary is required. | KILN-EVID-VAL-007 | pending |
| KILN-VAL-008 | KILN-NEED-001; KILN-REQ-011 | Future agent / maintainer | Resume work without hidden claims or chat-only context. | Future agent reads VTRACE docs, work packages, verification, validation, trace, and review. | Missing proof is visible as pending evidence, deferred validation, risk, or work-package obligation. | KILN-EVID-VAL-008 | pending |

## Acceptance Evidence

| Evidence ID | Scenario ID | Evidence | Acceptance Standard | Result |
|---|---|---|---|---|
| KILN-EVID-VAL-001 | KILN-VAL-001 | Valid, missing-field, and degraded declaration fixtures plus diagnostics. | A maintainer can identify readiness and required next action from local output. | pending |
| KILN-EVID-VAL-002 | KILN-VAL-002 | Mock runtime handoff record inspection. | Runtime-facing record exposes checked inputs and unresolved gates; KILN performs no execution. | pending |
| KILN-EVID-VAL-003 | KILN-VAL-003 | Policy negative fixture and build-record handoff section. | Policy needs are explicit; KILN makes no authorization decision. | pending |
| KILN-EVID-VAL-004 | KILN-VAL-004 | Package negative fixture and package handoff section. | Package metadata is checkable; KILN does not publish, sign, distribute, or mutate registry state. | pending |
| KILN-EVID-VAL-005 | KILN-VAL-005 | CAL boundary fixture and role review. | KILN references primitive IDs only and flags attempts to define semantics. | pending |
| KILN-EVID-VAL-006 | KILN-VAL-006 | Enterprise adapter/dependency review fixture. | Public core build record is sufficient for display; enterprise fields/deps are not required in KILN. | pending |
| KILN-EVID-VAL-007 | KILN-VAL-007 | Dependency inspection and source-custody review. | Public core has no private, provider, enterprise-only, runtime, registry, or policy-engine dependency. | pending |
| KILN-EVID-VAL-008 | KILN-VAL-008 | VTRACE proof package inspection. | Work can resume from docs and ledgers without using chat history as authority. | pending |
| KILN-EVID-VAL-009 | all | Role-review ledger. | No unresolved critical or major finding against scenario usefulness or boundaries. | pending |
| KILN-EVID-VAL-010 | all | `TRACE.md` review. | Each validation scenario links to need, CONOPS, requirements, verification, and evidence. | pending |

## Scenario Acceptance Detail

### KILN-VAL-001: Public build-record declaration

Validation question: Can a public repo use KILN to understand whether a
managed-agent capability declaration is ready without depending on enterprise
systems?

Acceptance criteria:

- A valid fixture produces a ready decision and build-record evidence.
- Missing required declaration data produces visible diagnostics.
- Degraded output remains useful and does not look like full readiness.
- No enterprise-only dependency or product-specific vocabulary is required.

Evidence source: KILN-WP-003 through KILN-WP-007.

Status: pending.

### KILN-VAL-002: Runtime handoff

Validation question: Can a runtime consumer determine what KILN checked before
deciding whether to execute under its own rules?

Acceptance criteria:

- Build record includes status, checked inputs, diagnostics, unresolved gates,
  evidence obligations, and runtime handoff summary.
- KILN never executes runtime work.
- Unresolved policy/package/evidence gates are not hidden.

Evidence source: KILN-WP-006, KILN-WP-007, and KILN-WP-008.

Status: pending.

### KILN-VAL-003: Policy handoff

Validation question: Can KILN make policy needs visible while leaving
authorization decisions to WARDEN or another policy system?

Acceptance criteria:

- Policy-needs section is accepted as declaration/handoff data.
- Missing or implicit authority emits diagnostics.
- No output implies `authorized`, `approved`, or equivalent KILN-owned policy
  success.

Evidence source: KILN-WP-005 through KILN-WP-007.

Status: pending.

### KILN-VAL-004: Package or registry handoff

Validation question: Can KILN prepare package metadata without becoming DEPOT or
a registry service?

Acceptance criteria:

- Package identity, version, publication intent, and evidence pointers are
  inspectable.
- Publication, signing, distribution, and registry mutation claims are rejected
  or marked not ready.
- Source-custody review confirms the boundary wording stays public and neutral.

Evidence source: KILN-WP-005 through KILN-WP-008.

Status: pending.

### KILN-VAL-005: CAL boundary check

Validation question: Can KILN reference common agent primitives without defining
their semantics?

Acceptance criteria:

- KILN records CAL primitive identifiers and version expectations only.
- Attempts to define primitive behavior in KILN produce boundary diagnostics.
- CAL ownership remains a downstream/public-library boundary, not KILN core.

Evidence source: KILN-WP-005 through KILN-WP-008.

Status: pending.

### KILN-VAL-006: Enterprise review display

Validation question: Can an enterprise review surface consume public KILN
evidence without changing KILN core?

Acceptance criteria:

- Public build records and diagnostics are sufficient for a display adapter.
- Enterprise-only fields, private data, and enterprise repo dependencies are not
  required in KILN core.
- Adapter-specific behavior is recorded as downstream or future work.

Evidence source: KILN-WP-006 through KILN-WP-008.

Status: pending.

### KILN-VAL-007: Public adoption boundary

Validation question: Is KILN credible as public infrastructure like RUNE?

Acceptance criteria:

- Core dependencies are public and product-neutral.
- .NET, MSBuild, and NuGet framing remains analogy only.
- LATTICE, WITNESS, BAKER, enterprise connectors, provider SDKs, tenant data,
  and private product surfaces remain outside KILN core.

Evidence source: KILN-WP-001, KILN-WP-002, and KILN-WP-008.

Status: pending.

### KILN-VAL-008: Future-agent resumption

Validation question: Can a future agent continue from repository evidence rather
than chat history?

Acceptance criteria:

- VTRACE docs have stable IDs, stage ledgers, and source links.
- Missing implementation proof remains pending rather than claimed.
- Work packages define the implementation sequence and validation closure.

Evidence source: KILN-WP-001 and KILN-WP-008.

Status: pending.

## Validation Levels

| Level | Purpose | Scenario Coverage | Evidence |
|---|---|---|---|
| L0 | Active work-package usefulness check. | One scenario or one fixture path. | Targeted fixture, doc hygiene, or role review. |
| L1 | Foundation-slice acceptance before push or PR. | KILN-VAL-001 through KILN-VAL-005 plus public dependency posture. | Full fixture matrix, build records, diagnostics, dependency inspection. |
| L2 | Public/readiness claim or downstream adoption. | All validation scenarios. | Mock runtime/review evidence, trace closure, role-review fixed point, accepted risk ledger. |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Real ARCADE/runtime execution | KILN must not execute workflows in the foundation slice. | Runtime consumers may need later compatibility proof. | After KILN emits stable build records and a runtime repo requests integration. |
| Real WARDEN policy decision | WARDEN may not exist yet and KILN must not decide authorization. | Policy handoff may remain abstract. | When WARDEN or another policy contract exists. |
| Real DEPOT publication | DEPOT may not exist yet and KILN must not publish packages. | Package handoff may remain fixture-only. | When DEPOT or another registry contract exists. |
| Real CAL semantic compatibility | CAL is planned separately and KILN must not own primitive semantics. | CAL references may be identifiers only. | When CAL baseline is created and interface-reviewed. |
| GAUGE/PROOF conformance report | KILN should expose evidence but not own conformance tooling. | Public readiness claims may need later conformance depth. | When GAUGE/PROOF-style tools exist or are selected. |
| Enterprise review UI | Enterprise surfaces must remain downstream/adapters. | Display validation may be mock-only. | When WITNESS/Workbench-style adapter work is explicitly planned. |

## Current Gaps

| Gap | Impact | Disposition |
|---|---|---|
| KILN local Git repository exists. | Local SHA-based validation closure is available. | Resolved locally; remote/submodule snapshot remains future portfolio work. |
| Foundation implementation exists. | Fixture demonstrations and build-record inspection run locally. | Resolved for foundation slice. |
| Fixtures exist. | Scenario acceptance evidence can close for local fixture scope. | Resolved in KILN-WP-003. |
| No downstream CAL/WARDEN/DEPOT/GAUGE repos are baselined. | L2 validation must use boundary/mock evidence. | Accept for first slice; defer real integration. |
| Pre-VTRACE README/Product Plan/wave docs may still imply implementation. | Future agents could mistake intent for accepted evidence. | Resolve in KILN-WP-001 before code. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Validation must test whether KILN is the right build/check layer, not just whether tests pass. | Addressed by mission and CONOPS scenario acceptance criteria. |
| Requirements Traceability Auditor | Validation scenarios need stable IDs and source anchors for later trace rows. | Addressed by `KILN-VAL-*` IDs and parent links. |
| Verification and Validation Lead | Validation must stay distinct from verification and record pending evidence honestly. | Addressed by method legend, pending results, and deferred validation table. |
| Software Assurance Guardian | Validation must not authorize implementation before proof package closure. | Addressed by scope, current gaps, and work-package evidence sources. |
| Security Privacy Guardian | Policy, package, runtime, and enterprise scenarios must not imply hidden authority or mutation. | Addressed by boundary acceptance criteria and deferred real integrations. |
| Source Custody Counsel | Public/enterprise and .NET analogy boundaries must remain clear in acceptance criteria. | Addressed by KILN-VAL-007 and source-custody evidence. |
| Repo Maintainer | Validation must be practical for the foundation slice. | Addressed by L0/L1/L2 levels and fixture/mock evidence. |
| Future Agent | Validation must be resumable without chat history. | Addressed by evidence IDs, gaps, and stage ledger. |

## Validation Gate

Decision: pass_with_risk.

Rationale: The validation plan is specific enough to proceed to Trace and Review
planning. It validates the right mission scenarios for the foundation slice while
keeping actual acceptance evidence pending until implementation work packages
produce fixtures, build records, dependency evidence, and retained outputs.

Stage ledger:

| Repo | Stage | File | Status | Input SHA | Output SHA | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|---|---|---|
| KILN | VALIDATION | `docs/vtrace/VALIDATION.md` | settled | pending repo init | pending repo init | KILN `.roles` simulated fixed-point review | No unresolved critical/major findings; evidence gaps recorded | pass_with_risk | TRACE |

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/WORK_PACKAGES.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
