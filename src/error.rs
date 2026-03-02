pub enum GameError {
    InvalidChoice,
    MissingItem(String),
}

pub struct ParseError;

pub enum ScenarioError {
    NonExistingStartScene,
    NonUniqueSceneIds(String),
    NonExistingChoice(String),
}

impl GameError {
    pub fn display(&self) {
        match self {
            GameError::InvalidChoice => { println!("La destination indiquée est invalide. Veuillez réessayer.") },
            GameError::MissingItem(item) => {println!("Pour aller dans cette direction, il vous manque l'objet : {item}.")}
        }
    }
}

impl ScenarioError {
    pub fn display(&self) {
        match self {
            ScenarioError::NonExistingStartScene => {
                println!("La scène initiale indiquée n'existe pas");
            },
            ScenarioError::NonUniqueSceneIds(id) => {
                println!("Deux scènes ont le même identifiant : {id}");
            },
            ScenarioError::NonExistingChoice(choice) => {
                println!("Un des choix indiqués n'existe pas : {choice}");
            },
        }
    }
}
