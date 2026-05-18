# FAUNA Product Plan

## One-line product

FAUNA is the animal-life knowledge system for portfolio worlds.

## Problem

Animals in settlement, history, and economy systems often collapse into static
resources: meat, wool, pests, predators, or flavor. That loses the behaviors
that make animal systems matter: migration, reproduction, habitat pressure,
fear, hunger, disease vectors, domestication, and seasonal conflict with people.

## Product promise

FAUNA turns animal life into reviewable evidence artifacts. It lets a repo say
which species or herd is present, what habitat it depends on, what seasonal
behaviors create pressure, and which source or fixture supports the claim.

## Core objects

| Object | Meaning |
|---|---|
| Species record | Stable record for an animal species, herd, breed, pest, predator, pollinator, or disease vector. |
| Habitat link | Place, biome, corridor, route, or settlement edge where the animal can persist or move. |
| Behavior pattern | Seasonal, daily, reproductive, predatory, social, domestic, or pest behavior. |
| Animal pressure | A downstream-relevant pressure such as crop loss, traction, food, disease, fear, manure, hide, wool, or route risk. |
| Evidence reference | Source path, source id, and optional anchor supporting a record. |

## First wave

**Wave:** FAUNA Foundation

Goal: establish the repo, Rust core, CLI validator, first seed fixture, and
dependency-chain placement without prematurely building a full ecology simulator.

Pulses:

1. **Repo foundation and intake** - scaffold docs, Rust workspace, skills, seed
   fixture, and TRACKER integration.
2. **Species and habitat schema** - harden species, habitat, and evidence rules.
3. **Behavior and pressure schema** - harden behavior patterns and downstream
   pressure categories.
4. **Consumer probes** - test one fixture each for BANISH, PORTO, CERES, LUCIA,
   and CANON.
5. **Publisher contract** - define CROP/PEBBLE/FLETCH/PROOF output boundaries.

### Foundation acceptance checks

| Pulse | Acceptance check |
|---:|---|
| 1 | Rust workspace builds, seed fixture validates, and TRACKER records FAUNA as a Knowledge Systems submodule. |
| 2 | Species and habitat records distinguish evidence-backed presence from speculative worldbuilding. |
| 3 | Behavior and pressure records can explain why animal life changes routes, food, labor, risk, or settlement choices. |
| 4 | Consumer probes do not move BANISH mechanics, PORTO geography, CERES economics, LUCIA interpretation, or CANON identity into FAUNA. |
| 5 | Publisher contract names generated artifacts and keeps fetch/cache/crop/render ownership outside FAUNA. |

## Dependency placement

FAUNA is a Knowledge Systems repo and source-corpus candidate. It may eventually
provide a runtime crate, but the first wave treats generated artifacts and
validation as safer than forcing downstream runtime linkage.

| System | Initial status | Reason |
|---|---|---|
| PROOF | Planned | Validate and render fauna docs and generated status reports. |
| CROP | Planned | Index fauna documents and crop species/habitat neighborhoods. |
| PEBBLE | Planned | Package portable fauna bundles for AI/context transfer. |
| FLETCH | Planned | Publish fetchable fauna packs and registries downstream. |
| RLINE | Deferred | Extract shared ecology/timeline primitives only after repeated consumer need is proven. |
| SLICE | Planned later | Query fauna documents once selectors stabilize. |
| ROLES | Planned | Add ecology/source-custody/downstream-readiness review panels after schema stabilizes. |

## Non-goals

- No full ecological simulation engine in the foundation wave.
- No CERES price/cost ownership, PORTO map ownership, or BANISH rules ownership.
- No LUCIA cultural interpretation policy or CANON identity ownership.
- No fetch/cache, graph-cut selection, portable pack, or rendering ownership.
- No shared RLINE extraction until at least two downstream repos need the same
  primitive.

## Validation commands

```powershell
cargo fmt --check
cargo test
cargo run -p fauna-cli -- validate fixtures\seed-fauna.json
git grep -n "FAUNA" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
