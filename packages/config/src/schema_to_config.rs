use indexmap::IndexMap;

use crate::config::*;
use crate::config_schema::*;
use crate::error::*;
use std::rc::Rc;

pub struct ConfigDefaults {
    base_url: String,
    files: Vec<String>,
    ignores: Vec<String>,
}

impl Default for ConfigDefaults {
    fn default() -> Self {
        Self {
            base_url: ".".to_string(),
            files: vec![
                "**/*.ts".to_string(),
                "**/*.tsx".to_string(),
                "**/*.js".to_string(),
                "**/*.jsx".to_string(),
            ],
            ignores: vec!["**/node_modules/**".to_string()],
        }
    }
}

type AbstractionPath = String;
type AbstractionCache = IndexMap<AbstractionPath, AbstractionConfigRef>;

/// Преобразует ConfigSchema в Config, разрешая локальные ссылки.
pub fn convert_schema_to_config(
    schema: ConfigSchema,
    defaults: &ConfigDefaults,
) -> Result<Config, ConfigError> {
    let base_url = schema.base_url.unwrap_or_else(|| defaults.base_url.clone());
    let files = schema.files.unwrap_or_else(|| defaults.files.clone());
    let ignores = schema.ignores.unwrap_or_else(|| defaults.ignores.clone());

    let abstractions = schema.abstractions.unwrap_or_else(|| IndexMap::new());

    let mut converted_abstractions: AbstractionCache = IndexMap::new();

    // Преобразуем корневую абстракцию с разрешением ссылок
    let root =
        convert_abstraction_field_value(&mut converted_abstractions, &schema.root, &abstractions)?;

    Ok(Config {
        root,
        base_url,
        files,
        ignores,
    })
}

/// Преобразует AbstractionConfigLink в AbstractionConfig, разрешая ссылки.
fn convert_abstraction_field_value<'a>(
    cache: &mut AbstractionCache,
    link: &AbstractionFieldValueSchema,
    abstractions: &IndexMap<String, AbstractionSchema>,
) -> Result<AbstractionConfigRef, ConfigError> {
    match link {
        AbstractionFieldValueSchema::Schema(schema) => {
            convert_inline_abstraction_schema(cache, schema, abstractions)
        }
        AbstractionFieldValueSchema::Link(link) => {
            if let Some(config) = cache.get(&link.ref_) {
                return Ok(config.clone());
            }

            let ref_name = ref_to_name(&link.ref_);
            // Находим абстракцию по ссылке в коллекции абстракций
            if let Some(schema) = abstractions.get(&ref_name) {
                convert_abstraction_schema(cache, schema, abstractions, &ref_name)
            } else {
                Err(ConfigError::InvalidConfig(format!(
                    "Abstraction not found by link: {}",
                    &link.ref_
                )))
            }
        }
    }
}

fn ref_to_name(ref_name: &str) -> String {
    ref_name.replace("#/abstractions/", "")
}

fn convert_abstraction_schema(
    cache: &mut AbstractionCache,
    schema: &AbstractionSchema,
    abstractions: &IndexMap<String, AbstractionSchema>,
    name: &str,
) -> Result<AbstractionConfigRef, ConfigError> {
    // Проверяем кэш, возможно эта абстракция уже преобразована
    if let Some(cached) = cache.get(name) {
        return Ok(cached.clone());
    }

    // Создаем пустую абстракцию и добавляем в кэш до рекурсивных вызовов,
    // чтобы избежать циклических зависимостей
    let empty_abstraction = Rc::new(AbstractionConfig {
        name: name.to_string(),
        children: Vec::new(),
        rules: vec![],
    });

    cache.insert(name.to_string(), empty_abstraction.clone());

    // Обрабатываем extends, если указан
    let base_config = if let Some(extend) = &schema.extend {
        let ref_name = ref_to_name(&extend.ref_);
        // Находим базовую абстракцию
        if let Some(base_schema) = abstractions.get(&ref_name) {
            // Рекурсивно обрабатываем базовую абстракцию
            let base = convert_abstraction_schema(cache, base_schema, abstractions, &ref_name)?;
            Some(base)
        } else {
            return Err(ConfigError::InvalidConfig(format!(
                "Base abstraction not found: {}",
                &extend.ref_
            )));
        }
    } else {
        None
    };

    // Преобразуем дочерние абстракции текущей схемы
    let mut children = Vec::new();
    if let Some(schema_children) = &schema.children {
        for (matcher, child_link) in schema_children {
            let child = convert_abstraction_field_value(cache, child_link, abstractions)?;
            children.push((matcher.clone(), child));
        }
    }

    // Преобразуем правила текущей схемы
    let current_rules = if let Some(schema_rules) = &schema.rules {
        convert_rules(schema_rules)
    } else {
        vec![]
    };

    // Создаем новую абстракцию, объединяя базовую (если есть) и текущую
    let new_abstraction = if let Some(base) = base_config {
        // Клонируем базовую конфигурацию
        let mut merged_children = base.children.clone();
        let mut merged_rules = base.rules.clone();

        // Добавляем/переопределяем дочерние абстракции
        for (matcher, child) in children {
            // Ищем существующего ребенка с таким же именем
            if let Some(pos) = merged_children.iter().position(|(m, _)| m == &matcher) {
                // Заменяем существующего ребенка
                merged_children[pos] = (matcher, child);
            } else {
                // Добавляем нового ребенка
                merged_children.push((matcher, child));
            }
        }

        // Объединяем правила с переопределением
        for rule in current_rules {
            // Удаляем старое правило того же типа (если есть)
            merged_rules.retain(|r| !is_same_rule_type(r, &rule));
            // Добавляем новое правило
            merged_rules.push(rule);
        }

        Rc::new(AbstractionConfig {
            name: name.to_string(),
            children: merged_children,
            rules: merged_rules,
        })
    } else {
        // Если нет базовой конфигурации, создаем новую
        Rc::new(AbstractionConfig {
            name: name.to_string(),
            children,
            rules: current_rules,
        })
    };

    // Обновляем значение в кэше
    cache.insert(name.to_string(), new_abstraction.clone());

    Ok(new_abstraction)
}

fn convert_inline_abstraction_schema(
    cache: &mut AbstractionCache,
    schema: &NamedAbstractionSchema,
    abstractions: &IndexMap<String, AbstractionSchema>,
) -> Result<AbstractionConfigRef, ConfigError> {
    let name = schema.name.clone();

    // Проверяем кэш, возможно эта абстракция уже преобразована
    if let Some(cached) = cache.get(&name) {
        return Ok(cached.clone());
    }

    // Создаем пустую абстракцию и добавляем в кэш до рекурсивных вызовов,
    // чтобы избежать циклических зависимостей
    let empty_abstraction = Rc::new(AbstractionConfig {
        name: name.clone(),
        children: Vec::new(),
        rules: vec![],
    });

    cache.insert(name.clone(), empty_abstraction.clone());

    // Обрабатываем extends, если указан
    let base_config = if let Some(extend) = &schema.extend {
        let ref_name = ref_to_name(&extend.ref_);
        // Находим базовую абстракцию
        if let Some(base_schema) = abstractions.get(&ref_name) {
            // Рекурсивно обрабатываем базовую абстракцию
            let base = convert_abstraction_schema(cache, base_schema, abstractions, &ref_name)?;
            Some(base)
        } else {
            return Err(ConfigError::InvalidConfig(format!(
                "Base abstraction not found: {}",
                &extend.ref_
            )));
        }
    } else {
        None
    };

    // Преобразуем дочерние абстракции текущей схемы
    let mut children = Vec::new();
    if let Some(schema_children) = &schema.children {
        for (matcher, child_link) in schema_children {
            let child = convert_abstraction_field_value(cache, child_link, abstractions)?;
            children.push((matcher.clone(), child));
        }
    }

    // Преобразуем правила текущей схемы
    let current_rules = if let Some(schema_rules) = &schema.rules {
        convert_rules(schema_rules)
    } else {
        vec![]
    };

    // Создаем новую абстракцию, объединяя базовую (если есть) и текущую
    let new_abstraction = if let Some(base) = base_config {
        // Клонируем базовую конфигурацию
        let mut merged_children = base.children.clone();
        let mut merged_rules = base.rules.clone();

        // Добавляем/переопределяем дочерние абстракции
        for (matcher, child) in children {
            // Ищем существующего ребенка с таким же именем
            if let Some(pos) = merged_children.iter().position(|(m, _)| m == &matcher) {
                // Заменяем существующего ребенка
                merged_children[pos] = (matcher, child);
            } else {
                // Добавляем нового ребенка
                merged_children.push((matcher, child));
            }
        }

        // Объединяем правила с переопределением
        for rule in current_rules {
            // Удаляем старое правило того же типа (если есть)
            merged_rules.retain(|r| !is_same_rule_type(r, &rule));
            // Добавляем новое правило
            merged_rules.push(rule);
        }

        Rc::new(AbstractionConfig {
            name: name.clone(),
            children: merged_children,
            rules: merged_rules,
        })
    } else {
        // Если нет базовой конфигурации, создаем новую
        Rc::new(AbstractionConfig {
            name: name.clone(),
            children,
            rules: current_rules,
        })
    };

    // Обновляем значение в кэше
    cache.insert(name.clone(), new_abstraction.clone());

    Ok(new_abstraction)
}

// Функция для проверки, являются ли правила одного типа
fn is_same_rule_type(rule1: &RuleConfig, rule2: &RuleConfig) -> bool {
    match (rule1, rule2) {
        (RuleConfig::RequiredChildren { .. }, RuleConfig::RequiredChildren { .. }) => true,
        (RuleConfig::NoUnabstractionFiles { .. }, RuleConfig::NoUnabstractionFiles { .. }) => true,
        (RuleConfig::RestrictCrossImports { .. }, RuleConfig::RestrictCrossImports { .. }) => true,
        (RuleConfig::PublicApi { .. }, RuleConfig::PublicApi { .. }) => true,
        (RuleConfig::DependenciesPort { .. }, RuleConfig::DependenciesPort { .. }) => true,
        (RuleConfig::DependenciesDirection { .. }, RuleConfig::DependenciesDirection { .. }) => {
            true
        }
        _ => false,
    }
}

/// Преобразует правила из RulesConfigSchema в Vec<RuleConfig>.
fn convert_rules(rules: &RulesConfigSchema) -> Vec<RuleConfig> {
    let mut result = Vec::new();

    if let Some(required_children) = &rules.required_children {
        result.push(RuleConfig::RequiredChildren {
            severity: required_children
                .severity
                .clone()
                .unwrap_or(Severity::Error),
            pattern: required_children.pattern.clone(),
            names: required_children.names.clone(),
            ignore_pattern: required_children.ignore_pattern.clone(),
        });
    }

    if let Some(no_unabstraction_files) = &rules.no_unabstraction_files {
        result.push(RuleConfig::NoUnabstractionFiles {
            severity: no_unabstraction_files
                .severity
                .clone()
                .unwrap_or(Severity::Error),
        });
    }

    if let Some(restrict_cross_imports) = &rules.restrict_cross_imports {
        result.push(RuleConfig::RestrictCrossImports {
            severity: restrict_cross_imports
                .severity
                .clone()
                .unwrap_or(Severity::Error),
            ignore_pattern: restrict_cross_imports.ignore_pattern.clone(),
        });
    }

    if let Some(public_api) = &rules.public_api {
        result.push(RuleConfig::PublicApi {
            severity: public_api.severity.clone().unwrap_or(Severity::Error),
            names: public_api.names.clone(),
            pattern: public_api.pattern.clone(),
            ignore_pattern: public_api.ignore_pattern.clone(),
        });
    }

    if let Some(dependencies_port) = &rules.dependencies_port {
        result.push(RuleConfig::DependenciesPort {
            severity: dependencies_port
                .severity
                .clone()
                .unwrap_or(Severity::Error),
            names: dependencies_port.names.clone(),
            pattern: dependencies_port.pattern.clone(),
            ignore_pattern: dependencies_port.ignore_pattern.clone(),
        });
    }

    if let Some(dependencies_direction) = &rules.dependencies_direction {
        result.push(RuleConfig::DependenciesDirection {
            severity: dependencies_direction
                .severity
                .clone()
                .unwrap_or(Severity::Error),
            order: dependencies_direction.order.clone(),
        });
    }

    result
}
