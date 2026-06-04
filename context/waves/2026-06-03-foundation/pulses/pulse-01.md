# Pulse 01 - Foundation scaffold

## Objective

Create the KILN foundation scaffold as a public, product-neutral build/check
contract repo through the accepted VTRACE work packages.

## Allowed files

- `README.md`
- `PRODUCT_PLAN.md`
- `docs\vtrace\**`
- `context\waves\**`
- `.claude\skills\**`
- `.gitignore`

Later work packages add:

- `Cargo.toml`
- `crates\kiln-core\**`
- `crates\kiln-cli\**`
- `fixtures\**`

## Work package

1. Reconcile pre-VTRACE docs with the accepted VTRACE baseline.
2. Create the Rust workspace after dependency posture is recorded.
3. Add retained fixtures and a minimal declaration checker in `kiln-core`.
4. Add side-effect-free `kiln check <path-to-kiln.yaml>`.
5. Record evidence in Verification, Validation, Trace, and Review.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p kiln-cli -- check fixtures\valid\kiln.yaml --format json --out target\kiln\valid.build.json
git diff --check
```
