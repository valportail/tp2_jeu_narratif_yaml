use crate::GameError;
use crate::GameState;
use crate::ParseError;
use crate::Scenario;

#[derive(Debug, PartialEq)]
pub enum CommandOutcome {
    Continue,
    Exit,
    GameOver,
}

pub trait GameCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError>;
}

pub fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError> {
    let args: Vec<&str> = line.trim().split(' ').collect();
    match args[0] {
        "look" => Ok(Box::new(LookCommand)),
        "choose" => {
            if let Ok(n) = args[1].parse::<usize>() {
                Ok(Box::new(ChooseCommand { n }))
            } else {
                Err(ParseError)
            }
        }
        "inventory" => Ok(Box::new(InventoryCommand)),
        "status" => Ok(Box::new(StatusCommand)),
        "quit" => Ok(Box::new(QuitCommand)),
        _ => Err(ParseError),
    }
}

// Definition and implementation of every possible in-game command

pub struct LookCommand;

pub struct ChooseCommand {
    pub n: usize,
}

pub struct InventoryCommand;

pub struct StatusCommand;

pub struct QuitCommand;

// Implementation of `execute` for each possible command

impl GameCommand for LookCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        state.scene.display();
        Ok(CommandOutcome::Continue)
    }
}

impl GameCommand for ChooseCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        if let Some(choices) = &state.scene.choices
            && self.n < choices.len()
        {
            // Verify the required item
            if let Some(item) = &choices[self.n].required_item
                && !state.inventory.contains(item)
            {
                Err(GameError::MissingItem(item.to_string()))
            } else {
                // Try to find the new scene
                let Some(new_scene) = scenario.get_scene(&choices[self.n].next) else {
                    return Err(GameError::InvalidChoice);
                };

                // Move to the next scene
                state.scene = new_scene;

                // Check items
                if let Some(item) = &state.scene.found_item {
                    state.inventory.insert(item.to_string());
                }

                // Check HP delta
                if let Some(delta) = state.scene.hp_delta {
                    state.hp += delta;
                }

                // Display the new scene
                state.scene.display();

                // Check ending condition
                if state.hp <= 0 {
                    println!("Vous êtes mort.");
                    Ok(CommandOutcome::GameOver)
                } else if let Some(_ending) = &state.scene.ending {
                    Ok(CommandOutcome::Exit)
                } else {
                    Ok(CommandOutcome::Continue)
                }
            }
        } else {
            Err(GameError::InvalidChoice)
        }
    }
}

impl GameCommand for InventoryCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        state.display_inventory();

        Ok(CommandOutcome::Continue)
    }
}

impl GameCommand for StatusCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        state.display_hp();
        state.display_scene();

        Ok(CommandOutcome::Continue)
    }
}

impl GameCommand for QuitCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        _state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome::Exit)
    }
}
