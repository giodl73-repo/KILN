# KILN Product Plan

## Thesis

The managed-agent platform needs a public build/check/package layer before
runtimes, registries, policy engines, and review clients can interoperate.

KILN is that layer: the MSBuild analogue for agent capabilities. It checks
declared work inputs and prepares evidence-bearing build records, diagnostics,
package metadata, and runtime handoff metadata without executing provider calls
or mutating products.

## Placement

KILN belongs in **Standards & Protocols** with RUNE because its contracts should
be reusable by public and enterprise repos. Enterprise repos may consume KILN
records, but KILN must not depend on enterprise-only systems.

| Layer | Visibility | Role |
|---|---|---|
| RUNE | Public | Rust-neutral contract descriptors and retained evidence. |
| KILN | Public | Build/check/package records for managed-agent capabilities. |
| CAL | Public planned | Common Agent Library: reusable typed primitives and standard capabilities. |
| WARDEN | Public kernel, enterprise packs later | Policy and authorization decision contracts. |
| DEPOT | Public protocol, enterprise service later | Signed package/capability registry contracts. |
| GAUGE | Public planned | Conformance and bakeoff runner for managed-agent contracts. |
| WORKBENCH | Enterprise/product first | Application framework and review surfaces over governed agent workflows. |
| CHANNEL/BRIDGE | Enterprise adapters first | M365, GitHub, Azure, SQL, ticketing, identity, and tenant connectors. |

## Product shape

| Area | Target |
|---|---|
| Build record | Stable identity, version, source inputs, outputs, policy needs, package metadata, and diagnostics. |
| Check pipeline | Side-effect-free validation of required sections, references, gates, and planned outputs. |
| Runtime handoff | A deterministic record that runtimes can execute only after policy and package checks pass. |
| Package manifest | DEPOT-ready metadata for publishing capabilities without owning the registry. |
| Evidence | Machine-readable diagnostics and receipts that downstream PROOF/GAUGE flows can retain. |
| Adapters | Future RUNE input, CAL primitive, WARDEN policy, DEPOT package, and ARCADE runtime handoff adapters. |

The exact fields, fixtures, commands, and evidence are controlled by
`docs\vtrace\WORK_PACKAGES.md`, `docs\vtrace\VERIFICATION.md`,
`docs\vtrace\VALIDATION.md`, and `docs\vtrace\TRACE.md`.

## First consumers

| Consumer | Need |
|---|---|
| RUNE | Contract descriptor collections that can be referenced from KILN build records. |
| CAL | Standard primitives that KILN can require without hard-coding product workflows. |
| ARCADE | Runtime plans that only execute after KILN check gates pass. |
| BAKER | Incubation specs that can point to a public build/check layer instead of owning every implementation. |
| FLETCHER | Review/debug surfaces for checked build records and runtime handoffs. |
| GAUGE | Conformance fixtures and pass/fail records for managed-agent builds. |

## Wave plan

| Wave | Goal |
|---|---|
| Foundation | Establish VTRACE baseline, product-neutral model, fixture-backed CLI, wave docs, and validation commands. |
| Build record contract | Define stable build record fields, diagnostics, versioning, and fixture snapshots. |
| RUNE intake | Accept retained RUNE descriptor collections as declared inputs without making RUNE vocabulary part of KILN core. |
| CAL boundary | Identify which common primitives belong in CAL instead of KILN. |
| Policy/package handoff | Emit WARDEN-ready policy needs and DEPOT-ready package manifests without owning either system. |
| Runtime handoff | Produce ARCADE-style checked plans for mock execution. |
| Conformance | Add GAUGE-style fixtures for valid, invalid, degraded, and compatibility scenarios. |

## Non-goals

- No provider calls, network calls, product writes, publication, or registry
  mutation in the foundation slice.
- No enterprise-only dependency.
- No agent runtime execution.
- No policy decision engine.
- No package registry service.
- No common agent standard library implementation.
- No product-specific workflow syntax.

## Validation posture

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
git diff --check
```

These commands become required as implementation work packages create the
workspace and CLI. They are not current evidence until recorded in
`docs\vtrace\VERIFICATION.md`.
