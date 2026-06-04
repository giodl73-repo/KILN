# KILN Trace Matrix

## Scope

| Field | Value |
|---|---|
| Repo | KILN |
| VTRACE stage | Trace |
| Parent stages | `docs/vtrace/MISSION.md`, `docs/vtrace/CONOPS.md`, `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/ARCHITECTURE.md`, `docs/vtrace/INTERFACES.md`, `docs/vtrace/DESIGN.md`, `docs/vtrace/CODE_RIGOR.md`, `docs/vtrace/WORK_PACKAGES.md`, `docs/vtrace/VERIFICATION.md`, `docs/vtrace/VALIDATION.md` |
| Stage status | Role-reviewed fixed point |

This trace matrix connects KILN's mission and CONOPS through requirements,
specification items, design/interface surfaces, code-rigor constraints, work
packages, verification, validation, and evidence. It is a control record, not an
implementation claim. Implementation and executable evidence remain pending
until the relevant work packages produce code, fixtures, build records, and
retained outputs.

## Requirement Trace Matrix

| Requirement ID | Parent Need | Requirement Summary | Specification Item | Design / Interface Element | Code Rigor Constraint | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| KILN-REQ-001 | KILN-NEED-001 | Define public build/check/package contract. | KILN-SPEC-001; KILN-SPEC-003; KILN-SPEC-011 | KILN-ARCH-001; KILN-ARCH-002; KILN-IF-001; KILN-DES-001 | KILN-CR-015 | KILN-WP-001; KILN-WP-003 | VTRACE docs; future declaration model and fixtures | inspection / review | KILN-VAL-001 | KILN-EVID-VER-001; KILN-EVID-VAL-001 | accepted; evidence pending |
| KILN-REQ-002 | KILN-NEED-002 | Remain product-neutral with no enterprise-only dependencies. | KILN-SPEC-001; KILN-SPEC-012 | KILN-ARCH-010; KILN-IF-009; KILN-DES-008 | KILN-CR-010; KILN-CR-011 | KILN-WP-001; KILN-WP-002 | future Cargo manifests/imports; boundary docs | inspection / dependency review | KILN-VAL-007 | KILN-EVID-VER-002; KILN-EVID-VAL-007 | accepted; evidence pending |
| KILN-REQ-003 | KILN-NEED-001; KILN-CONOPS-001 | Check declared inputs without side effects. | KILN-SPEC-002; KILN-SPEC-003; KILN-SPEC-011 | KILN-ARCH-003; KILN-IF-002; KILN-DES-003; KILN-DES-008 | KILN-CR-003; KILN-CR-004; KILN-CR-009 | KILN-WP-004; KILN-WP-005; KILN-WP-007 | future parser/checker/CLI and side-effect fixtures | test / inspection | KILN-VAL-001; KILN-VAL-007 | KILN-EVID-VER-003; KILN-EVID-VAL-001 | accepted; evidence pending |
| KILN-REQ-004 | KILN-CONOPS-001 | Produce diagnostics for missing or ambiguous data. | KILN-SPEC-004; KILN-SPEC-011 | KILN-ARCH-004; KILN-IF-004; KILN-DES-005 | KILN-CR-004; KILN-CR-005; KILN-CR-006 | KILN-WP-003; KILN-WP-005 | future diagnostics module and negative fixtures | test | KILN-VAL-001; KILN-VAL-008 | KILN-EVID-VER-004; KILN-EVID-VAL-001 | accepted; evidence pending |
| KILN-REQ-005 | KILN-CONOPS-001; KILN-CONOPS-002 | Distinguish ready, not-ready, and degraded states. | KILN-SPEC-003; KILN-SPEC-005 | KILN-ARCH-003; KILN-ARCH-005; KILN-IF-003; KILN-DES-004 | KILN-CR-004; KILN-CR-005; KILN-CR-008 | KILN-WP-005; KILN-WP-006 | future classifier and build-record emitter | test / analysis | KILN-VAL-001; KILN-VAL-002 | KILN-EVID-VER-005; KILN-EVID-VAL-002 | accepted; evidence pending |
| KILN-REQ-006 | KILN-CONOPS-002 | Record what was checked and what remains gated before runtime handoff. | KILN-SPEC-005 | KILN-ARCH-005; KILN-ARCH-009; KILN-IF-003; KILN-IF-008; KILN-DES-006 | KILN-CR-008; KILN-CR-015 | KILN-WP-006; KILN-WP-007 | future `kiln.build.json` and runtime handoff fixture | test / inspection | KILN-VAL-002 | KILN-EVID-VER-006; KILN-EVID-VAL-002 | accepted; evidence pending |
| KILN-REQ-007 | KILN-CONOPS-003 | Describe policy needs without deciding authorization. | KILN-SPEC-006 | KILN-ARCH-006; KILN-IF-005; KILN-DES-007 | KILN-CR-005; KILN-CR-009; KILN-CR-010 | KILN-WP-005; KILN-WP-006; KILN-WP-007 | future policy handoff section and negative fixture | inspection / test | KILN-VAL-003 | KILN-EVID-VER-007; KILN-EVID-VAL-003 | accepted; evidence pending |
| KILN-REQ-008 | KILN-CONOPS-004 | Describe package metadata without registry mutation or publication. | KILN-SPEC-007 | KILN-ARCH-007; KILN-IF-006; KILN-DES-007 | KILN-CR-005; KILN-CR-009; KILN-CR-010 | KILN-WP-005; KILN-WP-006; KILN-WP-007 | future package handoff section and negative fixture | inspection / test | KILN-VAL-004 | KILN-EVID-VER-008; KILN-EVID-VAL-004 | accepted; evidence pending |
| KILN-REQ-009 | KILN-CONOPS-005 | Reference CAL primitives without defining semantics. | KILN-SPEC-008 | KILN-ARCH-008; KILN-IF-007; KILN-DES-002; KILN-DES-007; KILN-DES-009 | KILN-CR-005; KILN-CR-010 | KILN-WP-005; KILN-WP-006; KILN-WP-008 | future CAL reference fixture and boundary review | inspection / review | KILN-VAL-005 | KILN-EVID-VER-009; KILN-EVID-VAL-005 | accepted; evidence pending |
| KILN-REQ-010 | KILN-CONOPS-006 | Allow enterprise review clients to consume public evidence without enterprise core dependencies. | KILN-SPEC-009; KILN-SPEC-012 | KILN-ARCH-010; KILN-IF-009; KILN-DES-007 | KILN-CR-010; KILN-CR-011 | KILN-WP-002; KILN-WP-006; KILN-WP-008 | future build record, dependency review, adapter fixture | inspection / demonstration | KILN-VAL-006; KILN-VAL-007 | KILN-EVID-VER-010; KILN-EVID-VAL-006 | accepted; evidence pending |
| KILN-REQ-011 | KILN-NEED-001; KILN-CONOPS-001 | Make missing evidence explicit. | KILN-SPEC-004; KILN-SPEC-010; KILN-SPEC-011 | KILN-ARCH-004; KILN-ARCH-011; KILN-IF-010; KILN-DES-005; KILN-DES-010 | KILN-CR-004; KILN-CR-006; KILN-CR-015 | KILN-WP-001; KILN-WP-005; KILN-WP-008 | diagnostics, evidence manifest, VTRACE proof package | inspection / review | KILN-VAL-008 | KILN-EVID-VER-011; KILN-EVID-VAL-008 | accepted; evidence pending |
| KILN-REQ-012 | KILN-NEED-001 | Preserve traceability from mission through evidence. | KILN-SPEC-010 | KILN-ARCH-001; KILN-ARCH-011; KILN-IF-010 | KILN-CR-005; KILN-CR-014; KILN-CR-015 | KILN-WP-001; KILN-WP-008 | VTRACE docs; future evidence manifest | trace review | KILN-VAL-008 | KILN-EVID-VER-012; KILN-EVID-VAL-010 | accepted; evidence pending |

## Scenario Trace Matrix

| Scenario ID | Need / Actor | Requirements | Specs | Interfaces | Verification | Validation | Evidence | Status |
|---|---|---|---|---|---|---|---|---|
| KILN-CONOPS-001 | Public build-record declaration | KILN-REQ-003; KILN-REQ-004; KILN-REQ-005; KILN-REQ-011 | KILN-SPEC-002; KILN-SPEC-003; KILN-SPEC-004; KILN-SPEC-011 | KILN-IF-001; KILN-IF-002; KILN-IF-003; KILN-IF-004 | VER-KILN-002; VER-KILN-003; VER-KILN-004; VER-KILN-011 | KILN-VAL-001 | KILN-EVID-VAL-001 | accepted; evidence pending |
| KILN-CONOPS-002 | Runtime handoff | KILN-REQ-005; KILN-REQ-006 | KILN-SPEC-005 | KILN-IF-003; KILN-IF-008 | VER-KILN-005 | KILN-VAL-002 | KILN-EVID-VAL-002 | accepted; evidence pending |
| KILN-CONOPS-003 | Policy handoff | KILN-REQ-007 | KILN-SPEC-006 | KILN-IF-005 | VER-KILN-006 | KILN-VAL-003 | KILN-EVID-VAL-003 | accepted; evidence pending |
| KILN-CONOPS-004 | Package or registry handoff | KILN-REQ-008 | KILN-SPEC-007 | KILN-IF-006 | VER-KILN-007 | KILN-VAL-004 | KILN-EVID-VAL-004 | accepted; evidence pending |
| KILN-CONOPS-005 | CAL boundary check | KILN-REQ-009 | KILN-SPEC-008 | KILN-IF-007 | VER-KILN-008 | KILN-VAL-005 | KILN-EVID-VAL-005 | accepted; evidence pending |
| KILN-CONOPS-006 | Enterprise review display | KILN-REQ-010 | KILN-SPEC-009; KILN-SPEC-012 | KILN-IF-009 | VER-KILN-009; VER-KILN-012 | KILN-VAL-006; KILN-VAL-007 | KILN-EVID-VAL-006; KILN-EVID-VAL-007 | accepted; evidence pending |

## Specification Trace Matrix

| Spec ID | Requirements | Interfaces / Architecture | Work Packages | Verification | Validation | Evidence | Status |
|---|---|---|---|---|---|---|---|
| KILN-SPEC-001 | KILN-REQ-001; KILN-REQ-002 | KILN-ARCH-001; KILN-PC-001 | KILN-WP-001 | VER-KILN-001 | KILN-VAL-001; KILN-VAL-007 | KILN-EVID-VER-001 | accepted; evidence pending |
| KILN-SPEC-002 | KILN-REQ-003 | KILN-ARCH-003; KILN-IF-002; KILN-NF-002 | KILN-WP-004; KILN-WP-007 | VER-KILN-002 | KILN-VAL-001; KILN-VAL-007 | KILN-EVID-VER-003 | accepted; evidence pending |
| KILN-SPEC-003 | KILN-REQ-003; KILN-REQ-004; KILN-REQ-005 | KILN-ARCH-002; KILN-ARCH-003; KILN-IF-001; KILN-PC-002 | KILN-WP-003; KILN-WP-004; KILN-WP-005 | VER-KILN-003 | KILN-VAL-001 | KILN-EVID-VER-005 | accepted; evidence pending |
| KILN-SPEC-004 | KILN-REQ-004; KILN-REQ-011 | KILN-ARCH-004; KILN-IF-004; KILN-NF-003 | KILN-WP-005 | VER-KILN-004 | KILN-VAL-001; KILN-VAL-008 | KILN-EVID-VER-004 | accepted; evidence pending |
| KILN-SPEC-005 | KILN-REQ-006 | KILN-ARCH-005; KILN-IF-003; KILN-PC-002 | KILN-WP-006 | VER-KILN-005 | KILN-VAL-002 | KILN-EVID-VER-006 | accepted; evidence pending |
| KILN-SPEC-006 | KILN-REQ-007 | KILN-ARCH-006; KILN-IF-005; KILN-PC-003 | KILN-WP-005; KILN-WP-006 | VER-KILN-006 | KILN-VAL-003 | KILN-EVID-VER-007 | accepted; evidence pending |
| KILN-SPEC-007 | KILN-REQ-008 | KILN-ARCH-007; KILN-IF-006; KILN-PC-004 | KILN-WP-005; KILN-WP-006 | VER-KILN-007 | KILN-VAL-004 | KILN-EVID-VER-008 | accepted; evidence pending |
| KILN-SPEC-008 | KILN-REQ-009 | KILN-ARCH-008; KILN-IF-007; KILN-PC-005 | KILN-WP-005; KILN-WP-006 | VER-KILN-008 | KILN-VAL-005 | KILN-EVID-VER-009 | accepted; evidence pending |
| KILN-SPEC-009 | KILN-REQ-010 | KILN-ARCH-010; KILN-IF-009; KILN-PC-006 | KILN-WP-006; KILN-WP-008 | VER-KILN-009 | KILN-VAL-006 | KILN-EVID-VER-010 | accepted; evidence pending |
| KILN-SPEC-010 | KILN-REQ-012 | KILN-ARCH-001; KILN-ARCH-011; KILN-IF-010; KILN-NF-004 | KILN-WP-001; KILN-WP-008 | VER-KILN-010 | KILN-VAL-008 | KILN-EVID-VER-012; KILN-EVID-VAL-010 | accepted; evidence pending |
| KILN-SPEC-011 | KILN-REQ-001; KILN-REQ-003; KILN-REQ-004 | KILN-ARCH-003; KILN-ARCH-011; KILN-NF-003 | KILN-WP-003; KILN-WP-005 | VER-KILN-011 | KILN-VAL-001 | KILN-EVID-VER-004; KILN-EVID-VAL-001 | accepted; evidence pending |
| KILN-SPEC-012 | KILN-REQ-002; KILN-REQ-010 | KILN-ARCH-010; KILN-IF-009; KILN-PC-006; KILN-NF-001 | KILN-WP-002; KILN-WP-008 | VER-KILN-012 | KILN-VAL-007 | KILN-EVID-VER-002; KILN-EVID-VAL-007 | accepted; evidence pending |

## Work Package Trace Matrix

| Work Package | Primary Trace Coverage | Verification Closure | Validation Closure | Evidence Status |
|---|---|---|---|---|
| KILN-WP-001 | KILN-REQ-001; KILN-REQ-002; KILN-REQ-011; KILN-REQ-012; KILN-SPEC-001; KILN-SPEC-010 | Docs reconciled; doc hygiene evidence retained. | KILN-VAL-007; KILN-VAL-008 | pending |
| KILN-WP-002 | KILN-REQ-002; KILN-SPEC-012; KILN-CR-010; KILN-CR-011 | Workspace and dependency posture verified. | KILN-VAL-007 | pending |
| KILN-WP-003 | KILN-REQ-001; KILN-REQ-003; KILN-REQ-004; KILN-SPEC-011; KILN-IF-001 | Fixture inventory and model coverage verified. | KILN-VAL-001 | pending |
| KILN-WP-004 | KILN-REQ-003; KILN-SPEC-002; KILN-SPEC-003; KILN-DES-002; KILN-DES-003 | Parser/normalizer tests verified. | KILN-VAL-001 | pending |
| KILN-WP-005 | KILN-REQ-004; KILN-REQ-005; KILN-REQ-007; KILN-REQ-008; KILN-REQ-009; KILN-IF-004 | Rules, statuses, and diagnostics verified. | KILN-VAL-001; KILN-VAL-003; KILN-VAL-004; KILN-VAL-005 | pending |
| KILN-WP-006 | KILN-REQ-005; KILN-REQ-006; KILN-SPEC-005; KILN-IF-003 | Build-record snapshots verified. | KILN-VAL-002; KILN-VAL-006 | pending |
| KILN-WP-007 | KILN-REQ-003; KILN-REQ-004; KILN-REQ-005; KILN-REQ-006; KILN-IF-002 | CLI fixture and exit-code evidence verified. | KILN-VAL-001; KILN-VAL-002 | pending |
| KILN-WP-008 | KILN-REQ-011; KILN-REQ-012; KILN-SPEC-010; KILN-IF-010 | Verification, validation, trace, and review evidence closed. | all KILN-VAL scenarios | pending |

## Evidence Trace Matrix

| Evidence ID | Source Stage | Covers | Current Pointer | Status |
|---|---|---|---|---|
| KILN-EVID-VER-001 | Verification | public contract and source-of-truth posture | `docs/vtrace/VERIFICATION.md` planned evidence | pending |
| KILN-EVID-VER-002 | Verification | public dependency posture | future Cargo manifests/import review | pending |
| KILN-EVID-VER-003 | Verification | side-effect-free operation | future side-effect tests/review | pending |
| KILN-EVID-VER-004 | Verification | explicit diagnostics | future negative fixtures | pending |
| KILN-EVID-VER-005 | Verification | readiness state classification | future status fixture matrix | pending |
| KILN-EVID-VER-006 | Verification | build-record content | future build-record snapshots | pending |
| KILN-EVID-VER-007 | Verification | no authorization ownership | future policy negative fixture | pending |
| KILN-EVID-VER-008 | Verification | no registry/package trust mutation | future package negative fixture | pending |
| KILN-EVID-VER-009 | Verification | no CAL semantic ownership | future CAL boundary fixture | pending |
| KILN-EVID-VER-010 | Verification | enterprise-neutral core | future adapter/dependency inspection | pending |
| KILN-EVID-VER-011 | Verification | visible missing evidence | future diagnostics/records/VTRACE review | pending |
| KILN-EVID-VER-012 | Verification | full V continuity | this file and future proof package review | pending |
| KILN-EVID-VAL-001 | Validation | public build-record declaration usefulness | future declaration fixtures | pending |
| KILN-EVID-VAL-002 | Validation | runtime handoff usefulness | future mock runtime handoff record | pending |
| KILN-EVID-VAL-003 | Validation | policy handoff usefulness | future policy negative fixture | pending |
| KILN-EVID-VAL-004 | Validation | package handoff usefulness | future package negative fixture | pending |
| KILN-EVID-VAL-005 | Validation | CAL boundary usefulness | future CAL boundary fixture | pending |
| KILN-EVID-VAL-006 | Validation | enterprise review display usefulness | future adapter/dependency review | pending |
| KILN-EVID-VAL-007 | Validation | public adoption boundary | future dependency/source-custody review | pending |
| KILN-EVID-VAL-008 | Validation | future-agent resumption | VTRACE proof package inspection | pending |
| KILN-EVID-VAL-009 | Validation | role-review closure | stage review ledgers | pending |
| KILN-EVID-VAL-010 | Validation | validation-to-trace closure | this file and future review | pending |

## Orphan and Gap Check

| Check | Result | Notes |
|---|---|---|
| Every accepted `KILN-REQ-*` has a trace row. | pass | KILN-REQ-001 through KILN-REQ-012 covered. |
| Every accepted `KILN-SPEC-*` has a trace row. | pass | KILN-SPEC-001 through KILN-SPEC-012 covered. |
| Every CONOPS scenario has validation coverage. | pass_with_risk | Coverage is planned; executable evidence pending. |
| Every interface has verification coverage. | pass_with_risk | Covered in Verification; fixtures/code pending. |
| Every work package maps to verification and validation closure. | pass_with_risk | Closure requires implementation work packages. |
| Every evidence ID has a current status. | pass_with_risk | Most evidence is pending by design. |
| Repo SHA closure exists. | gap | KILN is not yet a Git repository. |
| Executable implementation evidence exists. | gap | Implementation remains blocked until Review and user approval. |

## Stage Review Ledger

| Role | Finding | Disposition |
|---|---|---|
| Systems Engineering Steward | Trace must show KILN's build/check mission without expanding scope into adjacent systems. | Addressed by scenario and boundary rows. |
| Requirements Traceability Auditor | Every accepted requirement/spec/interface/work package needs a visible trace path. | Addressed by requirement, scenario, spec, work-package, and evidence matrices. |
| Verification and Validation Lead | Trace must separate verified evidence from planned evidence. | Addressed by explicit pending statuses and orphan/gap check. |
| Software Assurance Guardian | Trace must tie code-rigor constraints to work packages before implementation. | Addressed by requirement and work-package trace rows. |
| Security Privacy Guardian | Policy, package, runtime, enterprise, and dependency boundaries must remain traceable. | Addressed by KILN-REQ-002 and KILN-REQ-007 through KILN-REQ-010 rows. |
| Source Custody Counsel | Public/enterprise and analogy boundaries must not become dependency or endorsement claims. | Addressed by public dependency and enterprise boundary trace. |
| Repo Maintainer | Trace must be useful for package-by-package execution. | Addressed by work-package trace matrix. |
| Future Agent | Trace must be resumable without chat history. | Addressed by stable IDs, evidence matrix, and gap table. |

## Trace Gate

Decision: pass_with_risk.

Rationale: Trace is coherent enough to proceed to Review. All accepted
requirements, specifications, CONOPS scenarios, work packages, verification
items, validation scenarios, and evidence IDs have trace paths. Accepted risks
are that KILN is not yet a Git repository and executable evidence is still
pending because implementation remains blocked.

Stage ledger:

| Repo | Stage | File | Status | Input SHA | Output SHA | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|---|---|---|
| KILN | TRACE | `docs/vtrace/TRACE.md` | settled | pending repo init | pending repo init | KILN `.roles` simulated fixed-point review | No unresolved critical/major findings; evidence gaps recorded | pass_with_risk | REVIEW |

## Source Links

- `docs/vtrace/MISSION.md`
- `docs/vtrace/CONOPS.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `docs/vtrace/ARCHITECTURE.md`
- `docs/vtrace/INTERFACES.md`
- `docs/vtrace/DESIGN.md`
- `docs/vtrace/CODE_RIGOR.md`
- `docs/vtrace/WORK_PACKAGES.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/VALIDATION.md`
- `.roles/ROLE.md`
- VTRACE `docs/framework/vtrace-process.md`
