# Pulse 01 - Foundation scaffold

## Objective

Create the KILN foundation scaffold as a public, product-neutral build/check
contract repo.

## Allowed files

- `README.md`
- `PRODUCT_PLAN.md`
- `Cargo.toml`
- `crates\kiln-core\**`
- `crates\kiln-cli\**`
- `fixtures\tiny\kiln.yaml`
- `context\waves\**`
- `.claude\skills\**`
- `.gitignore`

## Work package

1. Create the Rust workspace.
2. Add a minimal build fixture checker in `kiln-core`.
3. Add `kiln-cli status` and `kiln-cli check <fixture>`.
4. Record KILN placement, non-goals, first consumers, and next waves.
5. Add repo-local wave, pulse, and research skills.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- status
cargo run -q -p kiln-cli -- check fixtures\tiny\kiln.yaml
git diff --check
```
