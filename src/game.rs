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
        match &mut self.game_state {
            GameState::SelectStack { input, error } => {
                input.pop();
                *error = None;
            }
            _ => (),
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
            GameState::WriteLeaf { typing, .. } if typing.is_perfect()
        );
        if !complete {
            return false;
        }

        let GameState::WriteLeaf { leaf, .. } = std::mem::replace(
            &mut self.game_state,
            GameState::SelectStack {
                input: String::new(),
                error: None,
            },
        ) else {
            unreachable!("completed transcription should be in the writing state");
        };
        self.desktop.finished_leaves.push(leaf);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{GameContext, GameState};
    use crate::desktop::Desktop;
    use crate::leaf_pool::LeafPool;

    #[test]
    fn selecting_and_perfectly_typing_a_leaf_saves_it() {
        let pool = LeafPool {
            leaves: vec!["alpha beta".to_owned()],
        };
        let desktop = Desktop::new(&pool, 2, 1);
        let mut game = GameContext::new(desktop);

        for character in "alpha".chars() {
            assert!(!game.type_character(character));
        }
        assert!(game.press_enter());
        assert!(matches!(game.game_state, GameState::WriteLeaf { .. }));
        assert_eq!(game.desktop.stacks[0].len(), 0);

        let mut transitioned = false;
        for character in "alpha beta".chars() {
            transitioned = game.type_character(character);
        }
        assert!(transitioned);
        assert_eq!(game.desktop.finished_leaves.len(), 1);
        assert!(matches!(game.game_state, GameState::SelectStack { .. }));
    }

    #[test]
    fn incorrect_transcription_is_not_saved() {
        let pool = LeafPool {
            leaves: vec!["alpha".to_owned()],
        };
        let mut game = GameContext::new(Desktop::new(&pool, 1, 1));
        for character in "alpha".chars() {
            game.type_character(character);
        }
        game.press_enter();

        for character in "xxxxx".chars() {
            game.type_character(character);
        }

        assert!(game.desktop.finished_leaves.is_empty());
        assert!(matches!(game.game_state, GameState::WriteLeaf { .. }));
    }
}
