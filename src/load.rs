use std::fs::File;

use crate::scenario::Scenario;

pub fn read_scenario_from_file(path: &str) -> Result<Scenario, Box<dyn std::error::Error>> {
    // Read the YAML file
    let file = File::open(path)?;

    // Parse the file as a scenario
    let scenario: Scenario = serde_yaml::from_reader(&file)?;

    Ok(scenario)
}
