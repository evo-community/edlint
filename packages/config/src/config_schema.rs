use indexmap::IndexMap;

// To generate json schema replace IndexMap with HashMap and add JsonSchema derive. and generate_schema.rs run
// use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::Severity;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequiredChildrenRuleSchema {
    pub severity: Option<Severity>,
    pub names: Option<Vec<String>>,
    pub pattern: Option<Vec<String>>,
    pub ignore_pattern: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoUnabstractionFilesRuleSchema {
    pub severity: Option<Severity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicApiRuleSchema {
    pub severity: Option<Severity>,
    pub names: Option<Vec<String>>,
    pub pattern: Option<Vec<String>>,
    pub ignore_pattern: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictCrossImportsRuleSchema {
    pub severity: Option<Severity>,
    pub ignore_pattern: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependenciesDirectionRuleSchema {
    pub severity: Option<Severity>,
    pub order: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependenciesPortRuleSchema {
    pub names: Option<Vec<String>>,
    pub severity: Option<Severity>,
    pub pattern: Option<Vec<String>>,
    pub ignore_pattern: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RulesConfigSchema {
    #[serde(rename = "required-children")]
    pub required_children: Option<RequiredChildrenRuleSchema>,
    #[serde(rename = "no-unabstraction-files")]
    pub no_unabstraction_files: Option<NoUnabstractionFilesRuleSchema>,
    #[serde(rename = "public-api")]
    pub public_api: Option<PublicApiRuleSchema>,
    #[serde(rename = "restrict-cross-imports")]
    pub restrict_cross_imports: Option<RestrictCrossImportsRuleSchema>,
    #[serde(rename = "dependencies-direction")]
    pub dependencies_direction: Option<DependenciesDirectionRuleSchema>,
    #[serde(rename = "dependencies-port")]
    pub dependencies_port: Option<DependenciesPortRuleSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractionLinkSchema {
    #[serde(rename = "$ref")]
    pub ref_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AbstractionFieldValueSchema {
    Link(AbstractionLinkSchema),
    Schema(NamedAbstractionSchema),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractionSchema {
    pub extend: Option<AbstractionLinkSchema>,

    pub children: Option<IndexMap<String, AbstractionFieldValueSchema>>,
    pub rules: Option<RulesConfigSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedAbstractionSchema {
    pub name: String,
    pub extend: Option<AbstractionLinkSchema>,
    pub children: Option<IndexMap<String, AbstractionFieldValueSchema>>,
    pub rules: Option<RulesConfigSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSchema {
    /** Root abstraction */
    pub root: AbstractionFieldValueSchema,
    /** Base url */
    pub base_url: Option<String>,
    /** Globs of files to check */
    pub files: Option<Vec<String>>,
    /** Globs of files to ignore */
    pub ignores: Option<Vec<String>>,
    /** Reusable abstractions */
    pub abstractions: Option<IndexMap<String, AbstractionSchema>>,
}
