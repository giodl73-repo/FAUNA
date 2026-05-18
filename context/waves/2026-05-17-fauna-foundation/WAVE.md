# FAUNA Foundation Wave

## Goal

Create the first durable FAUNA scaffold: repository, Rust validation core, seed
fixture, repo-local skills, and TRACKER dependency placement.

## Scope

- Species, habitat, behavior, animal pressure, and evidence primitives.
- CLI fixture validation.
- First BANISH/PORTO/CERES/LUCIA/CANON boundary statements.
- Planned publisher boundaries for PROOF, CROP, PEBBLE, and FLETCH.

## Non-goals

- No full ecosystem simulator.
- No downstream game mechanics, map geometry, economic prices, or cultural
  interpretation.
- No RLINE extraction until repeated consumer needs are proven.

## Pulses

| Pulse | Title | Validation |
|---:|---|---|
| 1 | Repo foundation and intake | `cargo fmt --check`; `cargo test`; `cargo run -p fauna-cli -- validate fixtures\seed-fauna.json` |
| 2 | Species and habitat schema | Add fixtures that prove evidence-backed presence and habitat constraints. |
| 3 | Behavior and pressure schema | Add fixtures that prove migration, pest, predator, livestock, and disease-vector pressures. |
| 4 | Consumer probes | Validate BANISH, PORTO, CERES, LUCIA, and CANON fixture slices. |
| 5 | Publisher contract | First FLETCH local registry seed; CROP, PEBBLE, and PROOF boundaries remain planned. Validate with `fletch registry validate --file .fletch\registries\fauna-foundation-assets.json`. |
