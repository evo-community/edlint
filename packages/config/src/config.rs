use std::rc::Rc;

use indexmap::IndexMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type AbstractionName = String;
pub type AbstractionMatcher = String;

#[derive(Debug, Clone)]
pub enum RuleConfig {
    RequiredChildren {
        severity: Severity,
        names: Option<Vec<AbstractionName>>,
        pattern: Option<Vec<AbstractionMatcher>>,
        ignore_pattern: Option<Vec<AbstractionMatcher>>,
    },
    NoUnabstractionFiles {
        severity: Severity,
    },
    RestrictCrossImports {
        severity: Severity,
        ignore_pattern: Option<Vec<AbstractionMatcher>>,
    },
    DependenciesDirection {
        severity: Severity,
        order: Vec<String>,
    },
    PublicApi {
        severity: Severity,
        names: Option<Vec<AbstractionName>>,
        pattern: Option<Vec<AbstractionMatcher>>,
        ignore_pattern: Option<Vec<AbstractionMatcher>>,
    },
    DependenciesPort {
        severity: Severity,
        names: Option<Vec<AbstractionName>>,
        pattern: Option<Vec<AbstractionMatcher>>,
        ignore_pattern: Option<Vec<AbstractionMatcher>>,
    },
}

pub type AbstractionConfigRef = Rc<AbstractionConfig>;

#[derive(Debug, Clone)]
pub struct AbstractionConfig {
    pub name: AbstractionName,
    pub children: Vec<(AbstractionMatcher, AbstractionConfigRef)>,
    pub rules: Vec<RuleConfig>,
}

impl AbstractionConfig {
    pub fn new(name: AbstractionName) -> Self {
        Self {
            name,
            children: Vec::new(),
            rules: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Config {
    /** Root abstraction */
    pub root: AbstractionConfigRef,
    /** Base url */
    pub base_url: String,
    /** Globs of files to check */
    pub files: Vec<String>,
    /** Globs of files to ignore */
    pub ignores: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Off,
    Warn,
    Error,
}
