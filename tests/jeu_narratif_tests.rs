use tp2_jeu_narratif_yaml::{ChooseCommand, CommandOutcome, GameCommand, GameError, GameState, Scenario, ScenarioError};

fn test_scenario() -> Scenario {
    let test = r#"
start_scene: entrance
initial_hp: 10

scenes:
- id: entrance
  title: Entrée
  text: Plusieurs portes sont visibles et vous guideront vers votre destinée.

  choices:
  - label: Aller dans le jardin
    next: garden
  - label: Aller dans la cave
    next: basement
  - label: Aller au grenier
    next: attic
    required_item: key

- id: attic
  title: Grenier de la Déception
  text: Il n'y a rien ici, à part quelqu'un qui s'énerve sur des jeux pourris !.
  choices:
  - label: Redescendre
    next: entrance

- id: garden
  title: Jardin de la Victoire
  text: Prélassez-vous, vous avez réussi !
  ending: victory

- id: basement
  title: Cave de la Défaite
  text: Il fait froid ici, quelqu'un pourrait ramener un radiateur ?
  hp_delta: -12
  ending: defeat
    "#;

    serde_yaml::from_str(test).unwrap()
}

#[test]
fn victory_path() {
    let scen = test_scenario();

    let state = GameState::load_from_scenario(&scen);
    assert!(state.is_ok());

    if let Ok(mut state) = state {
        let comm = ChooseCommand{ n: 0 };
        let _ = comm.execute(&scen, &mut state);
        assert_eq!(state.scene.id, "garden");
    }
}

#[test]
fn invalid_choice_path() {
    let scen = test_scenario();

    let state = GameState::load_from_scenario(&scen);
    assert!(state.is_ok());

    if let Ok(mut state) = state {
        let comm = ChooseCommand{ n: 99 };
        let result = comm.execute(&scen, &mut state);
        assert_eq!(result, Err(GameError::InvalidChoice));
    }
}

#[test]
fn missing_item_path() {
    let scen = test_scenario();

    let state = GameState::load_from_scenario(&scen);
    assert!(state.is_ok());

    if let Ok(mut state) = state {
        let comm = ChooseCommand{ n: 2 };
        let result = comm.execute(&scen, &mut state);
        assert_eq!(result, Err(GameError::MissingItem("key".to_string())));
    }
}

#[test]
fn game_over_path() {
    let scen = test_scenario();

    let state = GameState::load_from_scenario(&scen);
    assert!(state.is_ok());

    if let Ok(mut state) = state {
        let comm = ChooseCommand{ n: 1 };
        let result = comm.execute(&scen, &mut state);
        assert_eq!(result, Ok(CommandOutcome::GameOver));
    }
}

#[test]
fn non_existing_start_scene_scenario() {
    let test = r#"
start_scene: kitchen
initial_hp: 10

scenes:
- id: entrance
  title: Entrée
  text: Plusieurs portes sont visibles et vous guideront vers votre destinée.
    "#;

    let scen: Scenario = serde_yaml::from_str(test).unwrap();
    let result = scen.validate();

    assert_eq!(result, Err(ScenarioError::NonExistingStartScene));
}

#[test]
fn double_id_scenario() {
    let test = r#"
start_scene: entrance
initial_hp: 10

scenes:
- id: entrance
  title: Entrée
  text: Plusieurs portes sont visibles et vous guideront vers votre destinée.

  choices:
  - label: Aller dans le jardin
    next: garden
  - label: Aller dans la cave
    next: basement

- id: basement
  title: Grenier de la Déception
  text: Il n'y a rien ici, à part quelqu'un qui s'énerve sur des jeux pourris !.
  choices:
  - label: Redescendre
    next: entrance

- id: garden
  title: Jardin de la Victoire
  text: Prélassez-vous, vous avez réussi !
  ending: victory

- id: basement
  title: Cave de la Défaite
  text: Il fait froid ici, quelqu'un pourrait ramener un radiateur ?
  hp_delta: -12
  ending: defeat
    "#;

    let scen: Scenario = serde_yaml::from_str(test).unwrap();
    let result = scen.validate();

    assert_eq!(result, Err(ScenarioError::NonUniqueSceneIds("basement".to_string())));
}

#[test]
fn missing_scene_scenario() {
    let test = r#"
start_scene: entrance
initial_hp: 10

scenes:
- id: entrance
  title: Entrée
  text: Plusieurs portes sont visibles et vous guideront vers votre destinée.

  choices:
  - label: Aller dans le jardin
    next: garden
  - label: Aller dans la cave
    next: basement
  - label: Aller au grenier
    next: attic
    required_item: key

- id: garden
  title: Jardin de la Victoire
  text: Prélassez-vous, vous avez réussi !
  ending: victory

- id: basement
  title: Cave de la Défaite
  text: Il fait froid ici, quelqu'un pourrait ramener un radiateur ?
  hp_delta: -12
  ending: defeat
    "#;

    let scen: Scenario = serde_yaml::from_str(test).unwrap();
    let result = scen.validate();

    assert_eq!(result, Err(ScenarioError::NonExistingChoice("attic".to_string())));
}