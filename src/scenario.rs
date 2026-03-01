use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

use serde::Deserialize;

use crate::ScenarioError;

#[derive(Debug, Deserialize)]
pub struct Choice {
    label: String,
    next: String,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    id: String,
    title: String,
    text: String,
    #[serde(default)]
    hp_delta: i8,
    #[serde(default)]
    found_item: String,
    #[serde(default)]
    choices: Vec<Choice>,
    #[serde(default)]
    ending: String
}

#[derive(Debug, Deserialize)]
pub struct Scenario {
    start_scene: String,
    initial_hp: u8,
    scenes: Vec<Scene>,
}

impl Scenario {
    pub fn validate(&self) -> Result<(), ScenarioError> {
        // Existing start_scene

        if self.scenes.iter().filter(|s| s.id == self.start_scene).count() == 0 {
            return Err(ScenarioError::NonExistingStartScene);
        }

        // Unique scene IDs

        let mut scene_map = HashMap::new();

        for scene in &self.scenes {
            match scene_map.entry(&scene.id) {
                Vacant(entry) => { entry.insert(&scene.id); }
                Occupied(_) => { return Err(ScenarioError::NonUniqueSceneIds(scene.id.clone())); }
            }
        }

        // All choices exist

        for scene in &self.scenes {
            for choice in &scene.choices {
                let exist = self.scenes.iter().filter(|s| s.id == choice.next).count();
                
                if exist == 0 { return Err(ScenarioError::NonExistingChoice(choice.next.clone())); }
            }
        }

        Ok(())
    }
}
