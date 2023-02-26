use pyo3::exceptions::PyConnectionError;
use pyo3::prelude::*;

async fn get_fettblog_rss() -> Result<String, Box<dyn std::error::Error>> {
    let result = reqwest::get("https://fettblog.eu/feed.xml")
        .await?
        .text()
        .await?;

    Ok(result)
}

#[pyfunction]
pub fn get_rss(py: Python<'_>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        let res = get_fettblog_rss().await;
        res.map_err(|err| PyErr::new::<PyConnectionError, _>(err.to_string()))
    })
}

#[pymodule]
pub fn pyo3_async(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_rss, m)?)?;

    Ok(())
}
