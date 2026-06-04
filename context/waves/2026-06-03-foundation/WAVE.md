# KILN Foundation Wave

## Goal

Establish KILN as the public MSBuild-style layer for managed-agent work: a
product-neutral repo that checks declared capability inputs and emits build
record evidence before runtimes, registries, policy engines, or review clients
consume the work.

## Scope

| Area | In scope |
|---|---|
| Repo scaffold | README, product plan, wave docs, skills, VTRACE docs, Rust workspace, and fixtures. |
| Core model | Minimal declaration, build-record, status, and diagnostic types. |
| CLI | Side-effect-free `kiln check <path-to-kiln.yaml>` command. |
| TRACKER | Roadmap and dependency intake updates that classify KILN as public. |

## Non-goals

- No provider calls.
- No network calls.
- No registry mutation or package publication.
- No runtime execution.
- No enterprise-only dependency.
- No CAL, WARDEN, DEPOT, GAUGE, or Workbench implementation.

## Pulse plan

| Pulse | Status | Purpose |
|---|---|---|
| `pulse-01` | active | Complete VTRACE baseline and execute foundation work packages. |
| `pulse-02` | planned | Define the stable build record schema and fixture snapshots. |
| `pulse-03` | planned | Add RUNE descriptor collection intake as an optional declared input. |
| `pulse-04` | planned | Draft CAL/WARDEN/DEPOT handoff boundaries without taking ownership. |

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
git diff --check
```
