# FAUNA Quality Rubric

Score each promoted FAUNA artifact 0-4 on each axis. A fixture is ready for
downstream prototype use only when every axis is at least 3.

| Axis | 0 | 2 | 4 |
|---|---|---|---|
| Living pressure | Animals are decorative. | Animal roles are named. | Behavior changes timing, safety, food, labor, disease, fear, or trust. |
| Habitat grounding | Habitat is absent. | A place or biome is named. | Habitat, route, season, shelter, or settlement edge constrains behavior. |
| Behavior specificity | Animals are static resources. | One behavior is named. | Movement, reproduction, predation, pest pressure, vector behavior, or domestication creates a decision. |
| Downstream boundary | FAUNA owns everything. | Downstream users are named. | FAUNA keeps animal evidence separate from BANISH rules, CERES economics, PORTO maps, LUCIA interpretation, and CANON identity. |
| Evidence readiness | No source or fixture support. | Seed evidence exists. | Evidence refs, ids, and fixture records can be validated and packed downstream. |

## BANISH nature-rite gamepack gate

The BANISH nature-rite pack should score **3+** on FAUNA's living-pressure axis
before a game moves from scenario pack to prototype planning. The gate is passed
when animal behavior is not flavor text: it must change a player's timing,
safety, food, labor, disease, fear, or trust decision.

Validation:

```powershell
cargo run -p fauna-cli -- validate fixtures\consumers\banish-nature-rite-gamepack.json
```
