# KILN Role Index

Roles for reviewing KILN as a public build/check/package contract for
managed-agent work.

> These roles are AI-simulated review lenses, not claims of real-person review.

## Parliament Roles

Use these roles before KILN VTRACE deliverables are treated as settled.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/systems-engineering-steward.md` | Systems Engineering Steward | Public build-system rigor vs. process theater |
| `parliament/requirements-traceability-auditor.md` | Requirements Traceability Auditor | Stable managed-agent trace links vs. vague platform ambition |
| `parliament/verification-validation-lead.md` | Verification and Validation Lead | Side-effect-free evidence vs. unproven build-system claims |
| `parliament/software-assurance-guardian.md` | Software Assurance Guardian | Pre-code rigor vs. premature implementation |
| `parliament/security-privacy-guardian.md` | Security Privacy Guardian | Safe build/package metadata vs. hidden execution or supply-chain risk |
| `parliament/source-custody-counsel.md` | Source Custody Counsel | .NET/build-system analogy grounding vs. attribution or endorsement mistakes |

## Stakeholder Roles

Use these roles to keep KILN adoptable by public and enterprise consumers.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/repo-maintainer.md` | Repo Maintainer | KILN can be adopted incrementally without blocking useful work |
| `stakeholders/future-agent.md` | Future Agent | KILN records are resumable from mission, trace IDs, evidence, and wave history |

## KILN-specific Review Order

1. Systems Engineering Steward
2. Requirements Traceability Auditor
3. Verification and Validation Lead
4. Software Assurance Guardian
5. Security Privacy Guardian
6. Source Custody Counsel
7. Repo Maintainer
8. Future Agent

For the first VTRACE stage, reviewers should inspect only
`docs/vtrace/MISSION.md` and the small set of product-context files it cites.
Findings that require CONOPS, requirements, architecture, implementation, or
code-rigor detail must be deferred to the matching later VTRACE stage rather
than forced into mission scope.
