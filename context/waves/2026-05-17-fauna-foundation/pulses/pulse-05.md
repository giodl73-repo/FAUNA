# Pulse 05: FLETCH registry seed

## Goal

Publish the first FAUNA-owned FLETCH registry for stable animal-life fixtures,
consumer probes, rubric, and capability-map assets.

## Change

- Added `.fletch\registries\fauna-foundation-assets.json`.
- Registered the seed fixture, BANISH nature-rite probe, quality rubric, and
  capability expansion document as repo-owned local file assets.

## Validation

```powershell
cargo fmt --check
cargo test
cargo run -p fauna-cli -- validate fixtures\seed-fauna.json
fletch registry validate --file .fletch\registries\fauna-foundation-assets.json
```
