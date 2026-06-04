# KILN Dependency Decision

## Scope

| Field | Value |
|---|---|
| Work package | KILN-WP-002 |
| Decision | std-only foundation workspace |
| Date | 2026-06-03 |
| Status | accepted for foundation slice |

KILN starts with a std-only Rust workspace. No parser, serializer, CLI, runtime,
registry, policy, provider, enterprise, or test dependency is introduced by
KILN-WP-002.

## Rationale

The `v0` foundation slice is fixture-backed and intentionally constrained. The
record shape, parser depth, and JSON output contract are not stable enough to
justify a YAML or serialization dependency before KILN-WP-003 through
KILN-WP-006 prove the required fixture behavior.

This closes KILN-WAIVER-001 for the first implementation pass by declining the
waiver: no non-std parser or serializer dependency is accepted.

## Verification

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

## Revisit Trigger

Reopen this decision only if a later work package demonstrates that constrained
fixture parsing or std-only JSON emission cannot satisfy the accepted interface
and code-rigor requirements without reducing correctness.
