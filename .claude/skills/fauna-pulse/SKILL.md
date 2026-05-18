# FAUNA Pulse Skill

Use this skill for individual FAUNA implementation pulses.

## Pulse requirements

- State the animal-life object being changed: species, habitat, behavior,
  pressure, or evidence.
- Add or update a fixture when schema behavior changes.
- Run:

```powershell
cargo fmt --check
cargo test
cargo run -p fauna-cli -- validate fixtures\seed-fauna.json
```
