use crate::errors::map_transpile_error;
use crate::helpers::{normalize_complexity_guard, resolve_dialect, to_python_object};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use pythonize::depythonize;

#[pyfunction(signature = (sql, options = None, dialect = "generic", *, complexity_guard = None))]
pub fn analyze_query(
    py: Python<'_>,
    sql: &str,
    options: Option<&Bound<'_, PyAny>>,
    dialect: &str,
    complexity_guard: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let dialect_impl = resolve_dialect(dialect)?;
    let mut options = match options {
        Some(options) => {
            depythonize::<polyglot_sql::AnalyzeQueryOptions>(options).map_err(|err| {
                pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid analyze_query options object: {err}"
                ))
            })?
        }
        None => polyglot_sql::AnalyzeQueryOptions::default(),
    };

    if dialect != "generic" && options.dialect == polyglot_sql::DialectType::Generic {
        options.dialect = dialect_impl.dialect_type();
    }

    let complexity_guard = normalize_complexity_guard(complexity_guard)?;
    let analysis = py.detach(|| {
        polyglot_sql::analyze_query_with_complexity_guard(sql, options, complexity_guard)
            .map_err(map_transpile_error)
    })?;
    to_python_object(py, &analysis)
}
