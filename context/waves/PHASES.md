# KILN Waves

Work is organized into small waves and pulses.

## Active wave

| Wave | Status | Purpose |
|---|---|---|
| `2026-06-03-foundation` | active | Establish KILN as a public build/check/package contract repo with a fixture-backed CLI. |

## Wave registry

| Wave | Status | Purpose |
|---|---|---|
| `2026-06-03-foundation` | active | Establish public repo scaffold, product-neutral model, fixture-backed CLI, wave docs, and validation commands. |

## Protocol

1. Read this file.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Confirm the pulse names allowed files and validation commands.
5. Implement the smallest complete work-package slice.
6. Keep KILN product-neutral and free of enterprise-only dependencies.
7. Update docs and wave/pulse status.
8. Run the pulse validation commands.
9. Commit when green.

## Validation commands

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- status
cargo run -q -p kiln-cli -- check fixtures\tiny\kiln.yaml
git diff --check
```
