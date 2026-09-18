use crate::helpers::{normalize_complexity_guard, resolve_dialect};
use crate::types::{validation_result_from_core, ValidationResult};
use polyglot_sql::ValidationOptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pythonize::depythonize;

#[pyfunction(signature = (sql, dialect = "generic", *, strict_syntax = false, semantic = false, complexity_guard = None))]
pub fn validate(
    py: Python<'_>,
    sql: &str,
    dialect: &str,
    strict_syntax: bool,
    semantic: bool,
    complexity_guard: Option<&Bound<'_, PyDict>>,
) -> PyResult<ValidationResult> {
    let dialect = resolve_dialect(dialect)?;
    let complexity_guard = normalize_complexity_guard(complexity_guard)?;
    let options = ValidationOptions {
        strict_syntax,
        semantic,
    };

    let result = py.detach(|| {
        polyglot_sql::validate_with_dialect_and_complexity_guard(
            sql,
            &dialect,
            &options,
            complexity_guard,
        )
    });

    Ok(validation_result_from_core(result))
}

#[pyfunction(signature = (sql, schema, dialect = "generic", *, check_types = false, check_references = false, strict = None, semantic = false, strict_syntax = false, complexity_guard = None))]
#[allow(clippy::too_many_arguments)]
pub fn validate_with_schema(
    py: Python<'_>,
    sql: &str,
    schema: &Bound<'_, PyAny>,
    dialect: &str,
    check_types: bool,
    check_references: bool,
    strict: Option<bool>,
    semantic: bool,
    strict_syntax: bool,
    complexity_guard: Option<&Bound<'_, PyDict>>,
) -> PyResult<ValidationResult> {
    let dialect = resolve_dialect(dialect)?.dialect_type();
    let schema: polyglot_sql::ValidationSchema = depythonize(schema).map_err(|err| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid schema object (expected ValidationSchema shape): {err}"
        ))
    })?;
    let complexity_guard = normalize_complexity_guard(complexity_guard)?;
    let options = polyglot_sql::SchemaValidationOptions {
        check_types,
        check_references,
        strict,
        semantic,
        strict_syntax,
        ..Default::default()
    };
    let result = py.detach(|| {
        polyglot_sql::validate_with_schema_and_complexity_guard(
            sql,
            dialect,
            &schema,
            &options,
            complexity_guard,
        )
    });
    Ok(validation_result_from_core(result))
}
