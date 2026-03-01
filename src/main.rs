use tp2_jeu_narratif_yaml::Scenario;

use tp2_jeu_narratif_yaml::load_scenario_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _scen: Scenario = load_scenario_from_file("story.yaml")?;

    Ok(())
}
