# Pulse 01 - Repo foundation and intake

## Intent

Create FAUNA as a Knowledge Systems repo with a minimal validation core and seed
fixture for animal-life records.

## Work

- Add README and product plan.
- Add Rust workspace with `fauna-core` and `fauna-cli`.
- Add `fixtures\seed-fauna.json`.
- Add repo-local wave, pulse, and research skills.
- Update TRACKER dependency intake and submodule records.

## Validation

```powershell
cargo fmt --check
cargo test
cargo run -p fauna-cli -- validate fixtures\seed-fauna.json
git grep -n "FAUNA" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
