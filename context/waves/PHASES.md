# KILN Waves

Work is organized into small waves and pulses.

## Active wave

| Wave | Status | Purpose |
|---|---|---|
| `2026-06-03-foundation` | active | Establish KILN's VTRACE-controlled public build/check/package contract and then execute the foundation work packages. |

## Wave registry

| Wave | Status | Purpose |
|---|---|---|
| `2026-06-03-foundation` | active | Establish VTRACE baseline, product-neutral model, fixture-backed CLI, wave docs, and validation commands. |

## Protocol

1. Read this file.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Confirm the VTRACE work package names allowed files and validation commands.
5. Implement the smallest complete work-package slice.
6. Keep KILN product-neutral and free of enterprise-only dependencies.
7. Update docs and wave/pulse status.
8. Run the pulse validation commands.
9. Commit when green.

## Validation commands

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
git diff --check
```
