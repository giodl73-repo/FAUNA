# FAUNA

Animal, herd, pest, predator, livestock, migration, habitat, and nonhuman
disease-vector knowledge for portfolio systems.

FAUNA is a Knowledge Systems repo. It does not own game mechanics, economic
pricing, route geometry, or cultural interpretation. It owns the evidence and
validation layer that lets downstream systems treat animals as living agents
with habitat, movement, reproduction, fear, hunger, seasonality, and ecological
pressure instead of static resource nodes.

## Product thesis

Settlement and world systems become shallow when animals only appear as meat,
danger, or decoration. FAUNA provides portable records for species, habitats,
behavior patterns, seasonal movement, livestock pressure, pests, predators, and
nonhuman disease vectors so game and knowledge repos can reason about animal
life as an active system.

## First consumers

| Consumer | Use |
|---|---|
| BANISH | Add living animal pressure to survival scenarios: herds, pests, predators, disease vectors, and livestock tradeoffs. |
| PORTO | Constrain animal movement by habitat, corridors, routes, terrain, season, and settlement reach. |
| CERES | Ground livestock, hides, wool, draft power, manure, pests, and animal-product production. |
| LUCIA | Connect animal practices to people-history, domestication, hunting, taboo, and livelihood records. |
| CANON | Preserve stable species, herd, habitat, and scenario ids across fixtures. |

## Initial scope

1. Define product-neutral Rust primitives for species, habitats, behavior
   patterns, animal pressures, evidence references, and validation findings.
2. Provide a CLI that validates FAUNA fixture records.
3. Establish the first wave/pulse process and research skill.
4. Document boundaries with BANISH, PORTO, CERES, LUCIA, CANON, PROOF, CROP,
   PEBBLE, FLETCH, RLINE, SLICE, and ROLES.
5. Defer simulation/runtime extraction until at least two consumers prove the
   same product-neutral animal-system primitives.

## Agent expansion

[`docs\agent-expansion-experiment.md`](docs\agent-expansion-experiment.md)
defines the agent-driven experiment for expanding FAUNA through 50-100 candidate
animal-life capability systems while improving the rubric after each batch.
The first expansion run is recorded in
[`docs\capability-expansion.md`](docs\capability-expansion.md).

## Non-goals

- FAUNA does not replace CERES economics or PORTO geography.
- FAUNA does not write LUCIA cultural interpretation or BANISH scenario rules.
- FAUNA does not become a full ecology simulator in the foundation wave.
- FAUNA does not own fetch/cache, context cropping, rendering, or portable pack
  distribution.
- FAUNA does not force downstream repos to link Rust crates before artifact
  contracts are stable.

## Crates

| Crate | Role |
|---|---|
| `fauna-core` | Species, habitat, behavior, pressure, evidence, and validation primitives. |
| `fauna-cli` | Validation CLI for FAUNA fixture records. |

## First validation

FAUNA uses [`QUALITY_RUBRIC.md`](QUALITY_RUBRIC.md) to score living pressure,
habitat grounding, behavior specificity, downstream boundaries, and evidence
readiness before an artifact is promoted beyond a draft seed.

```powershell
cargo fmt --check
cargo test
cargo run -p fauna-cli -- validate fixtures\seed-fauna.json
cargo run -p fauna-cli -- validate fixtures\consumers\banish-nature-rite-gamepack.json
git grep -n "FAUNA" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
