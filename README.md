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

## Planned foundation surfaces

The first controlled implementation slice is defined by
`docs\vtrace\WORK_PACKAGES.md` and must start with KILN-WP-001 before code.

| Planned surface | Purpose |
|---|---|
| `kiln-core` | Product-neutral declaration, diagnostic, status, and build-record logic. |
| `kiln-cli` | Side-effect-free `kiln check <path-to-kiln.yaml>` command. |
| `fixtures\` | Retained valid, invalid, degraded, and boundary scenarios. |

## CLI surface

```powershell
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
```

The checker remains local-file-only and side-effect-free except for an explicit
`--out` path.

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

## Validation posture

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
git diff --check
```

These commands become required as the matching work packages create the
workspace, fixtures, parser, checker, emitter, and CLI.

### Retained readiness proof

The focused fixture matrix records both an accepted declaration and a
structured unsupported-version rejection:

```powershell
cargo test -p kiln-core fixture_statuses_match_code_rigor_matrix
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json
cargo run -q -p kiln-cli -- check fixtures\unsupported-version\kiln.yaml --format json
```

The valid fixture reports `ready`. The rejected fixture reports `not_ready`
with an `unsupported_version` diagnostic; unsupported contracts are not
silently accepted.
