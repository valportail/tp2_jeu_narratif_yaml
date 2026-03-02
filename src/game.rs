use std::{collections::HashSet, io};

use crate::{CommandOutcome, Scenario, ScenarioError, Scene, parse_command};

pub struct GameState {
    pub scene: Scene,
    pub hp: i8,
    pub inventory: HashSet<String>,
}

impl GameState {
    pub fn load_from_scenario(scenario: &Scenario) -> Result<Self, ScenarioError> {
        if let Some(scene) = scenario.get_scene(&scenario.start_scene) {
            Ok(GameState {
                scene,
                hp: scenario.initial_hp,
                inventory: HashSet::new(),
            })
        } else {
            Err(ScenarioError::NonExistingStartScene)
        }
    }

    pub fn display_hp(&self) {
        println!("- Nombre de points de vie actuels : {0}", self.hp);
    }

    pub fn display_inventory(&self) {
        println!("- Liste des objets présents dans l'inventaire :");

        for item in &self.inventory {
            println!("* {item}");
        }
    }

    pub fn display_scene(&self) {
        println!("- Scène courante : {0}", self.scene.title);
    }

    pub fn display_options(&self) {
        println!("");
        println!("> Que souhaitez-vous faire ?");
        println!("* look : Regarder à nouveau la scène");
        println!("* choose <n> : Vous déplacer vers la scène numéro n");
        println!("* inventory : Afficher votre inventaire");
        println!("* status : Afficher les informations du personnage");
        println!("* quit : Quitter la partie");
    }

    pub fn play(&mut self, scenario: &Scenario) -> Result<(), Box<dyn std::error::Error>> {
        let mut outcome = CommandOutcome::Continue;
        let mut line = String::new();

        self.scene.display();

        while outcome == CommandOutcome::Continue {
            self.display_options();

            io::stdin().read_line(&mut line)?;

            let command = parse_command(&line);
        
            match command {
                Ok(command) => {
                    println!("");

                    let new_outcome = command.execute(&scenario, self);
        
                    match new_outcome {
                        Ok(out) => {
                            outcome = out;
                        }
                        Err(error) => { error.display(); }
                    }
                }
                Err(_error) => { println!("Commande '{line}' non reconnue. Veuillez réessayer.") }
            }

            line.clear();
        }

        Ok(())
    }
}
