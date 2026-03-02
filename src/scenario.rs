use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use serde::{Deserialize, Serialize};

use crate::ScenarioError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Choice {
    pub label: String,
    pub next: String,
    #[serde(default)]
    pub required_item: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub hp_delta: Option<i8>,
    #[serde(default)]
    pub found_item: Option<String>,
    #[serde(default)]
    pub choices: Option<Vec<Choice>>,
    #[serde(default)]
    pub ending: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub start_scene: String,
    pub initial_hp: i8,
    pub scenes: Vec<Scene>,
}

impl Scene {
    pub fn display(&self) {
        println!("==== {0} ====", self.title);
        println!("{0}", self.text);

        println!("");

        if let Some(item) = &self.found_item {
            println!("> Vous avez récupéré l'objet : {item}.");
        }

        if let Some(delta) = &self.hp_delta {
            println!("> Vous avez perdu {delta} points de vie.");
        }

        if let Some(choices) = &self.choices {
            println!("> Directions possibles :");

            for (n, choice) in choices.iter().enumerate() {
                println!("{n} : {0}", choice.label);
            }
        }

        if let Some(ending) = &self.ending {
            match ending.as_str() {
                "victory" => {
                    println!("Bravo, vous avez réussi !");
                }
                "escape" => {
                    println!("Vous vous êtes enfui.");
                }
                "defeat" => {
                    println!("Dommage, vous avez échoué.");
                }
                _ => {
                    println!("La partie est terminée.");
                }
            }
        }
    }
}

impl Scenario {
    pub fn get_scene(&self, id: &str) -> Option<Scene> {
        self.scenes.iter().find(|s| s.id == id).cloned()
    }

    pub fn validate(&self) -> Result<(), ScenarioError> {
        // Unique scene IDs

        let mut scene_map = HashMap::new();

        for scene in &self.scenes {
            match scene_map.entry(&scene.id) {
                Vacant(entry) => {
                    entry.insert(&scene.id);
                }
                Occupied(_) => {
                    return Err(ScenarioError::NonUniqueSceneIds(scene.id.clone()));
                }
            }
        }

        // Existing start_scene

        if !scene_map.contains_key(&self.start_scene) {
            return Err(ScenarioError::NonExistingStartScene);
        }

        // All choices exist

        for scene in &self.scenes {
            if let Some(choices) = &scene.choices {
                for choice in choices {
                    let exist = self.scenes.iter().filter(|s| s.id == choice.next).count();

                    if exist == 0 {
                        return Err(ScenarioError::NonExistingChoice(choice.next.clone()));
                    }
                }
            }
        }

        Ok(())
    }
}
