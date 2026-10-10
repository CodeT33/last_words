use crate::desktop::Desktop;
use crate::leaves::Leaf;
use crate::typing::TypingSession;

#[derive(Debug)]
pub(crate) enum GameState {
    SelectStack {
        input: String,
        error: Option<String>,
    },
    WriteLeaf {
        leaf: Leaf,
        typing: TypingSession,
    },
}

pub(crate) struct GameContext {
    pub(crate) desktop: Desktop,
    pub(crate) game_state: GameState,
}

impl GameContext {
    pub(crate) fn new(desktop: Desktop) -> Self {
        Self {
            desktop,
            game_state: GameState::SelectStack {
                input: String::new(),
                error: None,
            },
        }
    }

    pub(crate) fn type_character(&mut self, character: char) -> bool {
        match &mut self.game_state {
            GameState::SelectStack { input, error } => {
                if character == ' ' {
                    return self.select_stack();
                } else if !character.is_control() {
                    input.push(character);
                    *error = None;
                }
                false
            }
            GameState::WriteLeaf { typing, .. } => {
                typing.type_character(character);
                self.save_if_complete()
            }
        }
    }

    pub(crate) fn backspace(&mut self) {
        if let GameState::SelectStack { input, error } = &mut self.game_state {
            input.pop();
            *error = None;
        }
    }

    pub(crate) fn press_enter(&mut self) -> bool {
        match self.game_state {
            GameState::SelectStack { .. } => self.select_stack(),
            GameState::WriteLeaf { .. } => self.type_character('\n'),
        }
    }

    pub(crate) fn selection_matches_stack(&self, stack_index: usize) -> bool {
        let GameState::SelectStack { input, .. } = &self.game_state else {
            return false;
        };
        if input.is_empty() {
            return true;
        }

        self.desktop
            .stacks
            .get(stack_index)
            .and_then(|stack| stack.top_word())
            .map(|word| word.to_lowercase().starts_with(&input.to_lowercase()))
            .unwrap_or(false)
    }

    fn select_stack(&mut self) -> bool {
        let GameState::SelectStack { input, .. } = &self.game_state else {
            return false;
        };
        let input = input.trim();
        let selected = self.desktop.stacks.iter().position(|stack| {
            stack
                .top_word()
                .map(|word| word.eq_ignore_ascii_case(input))
                .unwrap_or(false)
        });

        if let Some(index) = selected {
            if let Some(leaf) = self.desktop.stacks[index].take_first() {
                self.game_state = GameState::WriteLeaf {
                    typing: TypingSession::new(leaf.content.clone()),
                    leaf,
                };
                return true;
            }
        }

        self.game_state = GameState::SelectStack {
            input: String::new(),
            error: Some("No available stack has that top word.".to_owned()),
        };
        false
    }

    fn save_if_complete(&mut self) -> bool {
        let complete = matches!(
            &self.game_state,
            GameState::WriteLeaf { typing, .. } if typing.is_complete()
        );

        if !complete {
            return false;
        }

        let GameState::WriteLeaf { mut leaf, .. } = std::mem::replace(
            &mut self.game_state,
            GameState::SelectStack {
                input: String::new(),
                error: None,
            },
        ) else {
            unreachable!("completed transcription should be in the writing state");
        };
        leaf.typing_data.number_typing_errors = match &self.game_state {
            GameState::WriteLeaf { typing, .. } => typing.typing_errors() as u32,
            _ => 0,
        };
        self.desktop.finished_leaves.push(leaf);
        true
    }
}
