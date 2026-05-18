use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaunaDocument {
    pub document_id: String,
    #[serde(default)]
    pub species: Vec<SpeciesRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeciesRecord {
    pub species_id: String,
    pub label: String,
    pub role: AnimalRole,
    #[serde(default)]
    pub habitats: Vec<HabitatLink>,
    #[serde(default)]
    pub behaviors: Vec<BehaviorPattern>,
    #[serde(default)]
    pub pressures: Vec<AnimalPressure>,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimalRole {
    WildHerd,
    Livestock,
    Predator,
    Pest,
    Pollinator,
    Scavenger,
    DiseaseVector,
    DraftAnimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HabitatLink {
    pub habitat_id: String,
    pub label: String,
    #[serde(default)]
    pub season: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub behavior_id: String,
    pub kind: BehaviorKind,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorKind {
    Migration,
    Grazing,
    Predation,
    Breeding,
    Nesting,
    PestOutbreak,
    Domestication,
    DiseaseTransmission,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimalPressure {
    pub pressure_id: String,
    pub category: PressureCategory,
    pub downstream_effect: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureCategory {
    Food,
    Labor,
    Material,
    RouteRisk,
    CropLoss,
    Disease,
    Fear,
    Soil,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source_id: String,
    pub path: String,
    #[serde(default)]
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: FindingSeverity,
    pub code: String,
    pub location: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Error,
    Warning,
}

pub fn validate_document(document: &FaunaDocument) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    require_non_empty(
        &mut findings,
        "document_id",
        "document.document_id",
        &document.document_id,
    );

    let mut seen_species_ids = HashSet::new();
    for species in &document.species {
        require_non_empty(
            &mut findings,
            "species_id",
            "species.species_id",
            &species.species_id,
        );
        require_non_empty(
            &mut findings,
            "species_label",
            &species.species_id,
            &species.label,
        );
        if !seen_species_ids.insert(species.species_id.as_str()) {
            findings.push(error(
                "duplicate_species_id",
                &species.species_id,
                "species id appears more than once",
            ));
        }
        if species.habitats.is_empty() {
            findings.push(warning(
                "species_without_habitat",
                &species.species_id,
                "species has no habitat link",
            ));
        }
        if species.behaviors.is_empty() {
            findings.push(warning(
                "species_without_behavior",
                &species.species_id,
                "species has no behavior pattern",
            ));
        }
        if species.pressures.is_empty() {
            findings.push(warning(
                "species_without_pressure",
                &species.species_id,
                "species has no downstream pressure",
            ));
        }
        if species.evidence_refs.is_empty() {
            findings.push(warning(
                "species_without_evidence",
                &species.species_id,
                "species has no evidence reference",
            ));
        }
        for habitat in &species.habitats {
            require_non_empty(
                &mut findings,
                "habitat_id",
                &species.species_id,
                &habitat.habitat_id,
            );
            require_non_empty(
                &mut findings,
                "habitat_label",
                &species.species_id,
                &habitat.label,
            );
        }
        for behavior in &species.behaviors {
            require_non_empty(
                &mut findings,
                "behavior_id",
                &species.species_id,
                &behavior.behavior_id,
            );
            require_non_empty(
                &mut findings,
                "behavior_description",
                &behavior.behavior_id,
                &behavior.description,
            );
        }
        for pressure in &species.pressures {
            require_non_empty(
                &mut findings,
                "pressure_id",
                &species.species_id,
                &pressure.pressure_id,
            );
            require_non_empty(
                &mut findings,
                "downstream_effect",
                &pressure.pressure_id,
                &pressure.downstream_effect,
            );
        }
    }

    findings
}

pub fn has_errors(findings: &[ValidationFinding]) -> bool {
    findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Error)
}

fn require_non_empty(
    findings: &mut Vec<ValidationFinding>,
    code: &str,
    location: &str,
    value: &str,
) {
    if value.trim().is_empty() {
        findings.push(error(code, location, "required field is empty"));
    }
}

fn error(code: &str, location: &str, message: &str) -> ValidationFinding {
    ValidationFinding {
        severity: FindingSeverity::Error,
        code: code.to_string(),
        location: location.to_string(),
        message: message.to_string(),
    }
}

fn warning(code: &str, location: &str, message: &str) -> ValidationFinding {
    ValidationFinding {
        severity: FindingSeverity::Warning,
        code: code.to_string(),
        location: location.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_species_ids_are_errors() {
        let document = FaunaDocument {
            document_id: "fauna:test".to_string(),
            species: vec![species("species:wolf"), species("species:wolf")],
        };
        let findings = validate_document(&document);
        assert!(findings
            .iter()
            .any(|finding| finding.code == "duplicate_species_id"));
        assert!(has_errors(&findings));
    }

    #[test]
    fn complete_species_record_has_no_errors() {
        let document = FaunaDocument {
            document_id: "fauna:test".to_string(),
            species: vec![species("species:wolf")],
        };
        assert!(!has_errors(&validate_document(&document)));
    }

    fn species(species_id: &str) -> SpeciesRecord {
        SpeciesRecord {
            species_id: species_id.to_string(),
            label: "Grey wolf".to_string(),
            role: AnimalRole::Predator,
            habitats: vec![HabitatLink {
                habitat_id: "habitat:forest-edge".to_string(),
                label: "Forest edge".to_string(),
                season: Some("winter".to_string()),
            }],
            behaviors: vec![BehaviorPattern {
                behavior_id: "behavior:winter-pack-hunting".to_string(),
                kind: BehaviorKind::Predation,
                description: "Packs test isolated livestock and weak travel parties.".to_string(),
            }],
            pressures: vec![AnimalPressure {
                pressure_id: "pressure:route-fear".to_string(),
                category: PressureCategory::RouteRisk,
                downstream_effect: "Winter routes need escorts or alternate timing.".to_string(),
            }],
            evidence_refs: vec![EvidenceRef {
                source_id: "seed".to_string(),
                path: "fixtures\\seed-fauna.json".to_string(),
                anchor: Some(species_id.to_string()),
            }],
        }
    }
}
