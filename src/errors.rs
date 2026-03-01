pub enum ScenarioError {
    NonExistingStartScene,
    NonUniqueSceneIds(String),
    NonExistingChoice(String),
}