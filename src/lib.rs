mod martingale;
pub use martingale::martingale_main;
use std::{collections::HashMap, str::FromStr};
use pyo3::prelude::*;

#[pyfunction]
fn play_martingale() -> PyResult<HashMap<String, Vec<i32>>> {
    let output_df = martingale_main();
    let mut output_map: HashMap<String, Vec<i32>> = HashMap::new();
    output_map.insert(String::from_str("nth").unwrap(), output_df.nth);
    output_map.insert(String::from_str("play_times").unwrap(), output_df.play_times);
    output_map.insert(String::from_str("final_wealth").unwrap(), output_df.final_wealth);
    Ok(output_map)
}

#[pymodule]
fn gambling_simulator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play_martingale, m)?)?;
    Ok(())
}
