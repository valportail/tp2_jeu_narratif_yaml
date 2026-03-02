use tp2_jeu_narratif_yaml::GameState;
use tp2_jeu_narratif_yaml::Scenario;

use tp2_jeu_narratif_yaml::read_scenario_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chargement du scénario
    let scen: Scenario = read_scenario_from_file("story.yaml")?;

    // Vérification du scénario
    let _ = match scen.validate() {
        Ok(()) => println!("Le scénario est bien valide !"),
        Err(scen_error) => scen_error.display(),
    };

    // Création d'une nouvelle partie
    
    if let Ok(mut state) = GameState::load_from_scenario(&scen) {
        let _ = state.play(&scen)?;
    }

    Ok(())
}
