# KILN

**A public build and validation pipeline for managed agent work.**

KILN is the MSBuild-style layer in the managed-agent stack. It turns stable
agent-facing inputs such as AgentMaps, workflow specs, policy requirements,
tool declarations, context obligations, and package metadata into checked,
versioned build records before any runtime executes them.

```text
AgentMap / workflow / policy / package inputs
  -> KILN check
  -> build plan
  -> validation gates
  -> package manifest
  -> runtime handoff
```

KILN is public infrastructure, like RUNE. Its core model must stay product
neutral so public repos can adopt the build/check contract without depending on
enterprise-only systems such as LATTICE, FLETCHER, or BAKER incubation records.

## Purpose

Agents need a repeatable build step before they do real work. KILN answers:

- What files, specs, policies, tools, context inputs, and outputs are part of
  this managed-agent capability?
- Can the capability be checked without provider calls, network calls, writes,
  registry mutation, publication, or runtime side effects?
- Which downstream runtime, package registry, policy engine, or review surface
  may consume the checked record?
- What evidence proves the capability was built from the declared inputs?

## What KILN is not

- Not an agent runtime; ARCADE-style runtimes execute checked work.
- Not a common library; CAL should own reusable agent primitives.
- Not a package registry; DEPOT should own distribution and trust catalogs.
- Not a policy engine; WARDEN should own authorization decisions.
- Not an enterprise product surface; FLETCHER/Workbench-style clients can
  consume KILN records later.
- Not tied to one provider, host, workflow language, or product vocabulary.

## Foundation crates

| Crate | Purpose |
|---|---|
| `kiln-core` | Product-neutral build record, diagnostics, and fixture check types. |
| `kiln-cli` | Fixture-backed `status` and `check` commands for the foundation slice. |

## Current CLI surface

```powershell
cargo run -p kiln-cli -- status
cargo run -p kiln-cli -- check fixtures\tiny\kiln.yaml
```

The foundation checker is intentionally narrow. It validates that a KILN fixture
declares the minimum build-record sections without parsing or executing any
domain-specific language.

## Placement in the managed-agent stack

```text
RUNE
  -> KILN
       -> CAL (planned)
       -> WARDEN (planned)
       -> DEPOT (planned)
       -> ARCADE / runtimes (planned consumers)
       -> FLETCHER / review clients (planned consumers)
```

RUNE supplies Rust-neutral contract descriptors. KILN records how declared
capabilities are checked and packaged. CAL, WARDEN, DEPOT, GAUGE, and Workbench
should remain separate repos only when their boundaries become real.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- status
cargo run -q -p kiln-cli -- check fixtures\tiny\kiln.yaml
git diff --check
```
