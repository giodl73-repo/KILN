# KILN

**A public build/check/package contract for managed-agent work.**

KILN is the MSBuild-style layer in the managed-agent stack. It is public,
product-neutral infrastructure for checking declared managed-agent capabilities
before runtime, registry, policy, conformance, or review systems consume them.
The accepted source of truth is the VTRACE package under `docs\vtrace`.

```text
declared managed-agent capability inputs
  -> side-effect-free KILN check
  -> diagnostics and evidence obligations
  -> build record
  -> runtime handoff
```

KILN is public infrastructure, like RUNE. Its core model must stay product
neutral so public repos can adopt the build/check contract without depending on
enterprise-only systems such as LATTICE, WITNESS, or BAKER incubation records.

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
- Not an enterprise product surface; WITNESS/Workbench-style clients can
  consume KILN records later.
- Not tied to one provider, host, workflow language, or product vocabulary.

## Planned foundation surfaces

The first controlled implementation slice is defined by
`docs\vtrace\WORK_PACKAGES.md` and must start with KILN-WP-001 before code.

| Planned surface | Purpose |
|---|---|
| `kiln-core` | Product-neutral declaration, diagnostic, status, and build-record logic. |
| `kiln-cli` | Side-effect-free `kiln check <path-to-kiln.yaml>` command. |
| `fixtures\` | Retained valid, invalid, degraded, and boundary scenarios. |

## Target CLI surface

```powershell
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
```

This command is target behavior, not evidence of current implementation. The
foundation checker must remain local-file-only and side-effect-free except for an
explicit `--out` path.

## Placement in the managed-agent stack

```text
RUNE
  -> KILN
       -> CAL (planned)
       -> WARDEN (planned)
       -> DEPOT (planned)
       -> ARCADE / runtimes (planned consumers)
       -> WITNESS / review clients (planned consumers)
```

RUNE supplies Rust-neutral contract descriptors. KILN records how declared
capabilities are checked and packaged. CAL, WARDEN, DEPOT, GAUGE, and Workbench
should remain separate repos only when their boundaries become real.

## Validation posture

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
git diff --check
```

These commands become required as the matching work packages create the
workspace, fixtures, parser, checker, emitter, and CLI. Until then, VTRACE docs
record planned evidence and accepted risks.
