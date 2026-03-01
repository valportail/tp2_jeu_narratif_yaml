use tp2_jeu_narratif_yaml::Scenario;

use tp2_jeu_narratif_yaml::ScenarioError;
use tp2_jeu_narratif_yaml::load_scenario_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chargement du scénario
    let scen: Scenario = load_scenario_from_file("story.yaml")?;

    // Vérification du scénario
    let _ = match scen.validate() {
        Err(ScenarioError::NonExistingStartScene) => println!("La scène initiale indiquée n'existe pas"),
        Err(ScenarioError::NonUniqueSceneIds(id)) => println!("Deux scènes ont le même identifiant : {id}"),
        Err(ScenarioError::NonExistingChoice(choice)) => println!("Un des choix indiqués n'existe pas : {choice}"),
        _ => println!("Le scénario est bien valide !"),
    };

    Ok(())
}
