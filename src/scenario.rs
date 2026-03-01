use serde::Deserialize;

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
