use crate::consts::COUNTDOWN_IN_SECONDS;
use crate::desktop::Desktop;
use crate::leaves::Leaf;
use crate::timer::Timer;
use crate::typing::TypingSession;

#[derive(Debug)]
pub(crate) enum GameState {
    SelectStack { input: String },
    WriteLeaf { leaf: Leaf, typing: TypingSession },
    TotalPoints { input: String },
}

#[derive(Debug)]
pub(crate) struct GameContext {
    pub(crate) desktop: Desktop,
    pub(crate) game_state: GameState,
    pub(crate) timer: Timer,
}

impl GameContext {
    pub(crate) fn new(desktop: Desktop) -> Self {
        Self {
            desktop,
            game_state: GameState::SelectStack {
                input: String::new(),
            },
            timer: Timer::new(COUNTDOWN_IN_SECONDS),
        }
    }

    pub(crate) fn update(&mut self, delta_seconds: f32) {
        self.timer.update(delta_seconds, &mut self.game_state);
    }

    pub(crate) fn type_character(&mut self, character: char) -> bool {
        match &mut self.game_state {
            GameState::SelectStack { input } => {
                if character.is_control() {
                    return false;
                }

                input.push(character);
                let valid_prefix = self.desktop.stacks.iter().any(|stack| {
                    stack
                        .top_word()
                        .map(|word| Self::word_prefix_matches(word, input))
                        .unwrap_or(false)
                });
                if !valid_prefix {
                    input.pop();
                    return false;
                }

                let complete_word = self.desktop.stacks.iter().any(|stack| {
                    stack
                        .top_word()
                        .map(|word| word.eq_ignore_ascii_case(input))
                        .unwrap_or(false)
                });
                if complete_word {
                    self.select_stack()
                } else {
                    false
                }
            },
            GameState::WriteLeaf { typing, .. } => {
                typing.type_character(character);
                self.save_if_complete()
            },
            _ => {
                false
            }
        }
    }

    pub(crate) fn backspace(&mut self) {
        if let GameState::SelectStack { input } = &mut self.game_state {
            input.pop();
        }
    }

    pub(crate) fn press_enter(&mut self) -> bool {
        match self.game_state {
            GameState::SelectStack { .. } => false,
            GameState::WriteLeaf { .. } => self.type_character('\n'),
            _ => {false},
        }
    }

    fn select_stack(&mut self) -> bool {
        let GameState::SelectStack { input, .. } = &self.game_state else {
            return false;
        };
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

        let GameState::WriteLeaf { mut leaf, typing } = std::mem::replace(
            &mut self.game_state,
            GameState::SelectStack {
                input: String::new(),
            },
        ) else {
            unreachable!("Completed transcription should be in the writing state");
        };
        leaf.typing_data.number_typing_errors = typing.typing_errors() as u32;
        leaf.typing_data.wpm = typing.words_per_minute();
        self.desktop.finished_leaves.push(leaf);
        true
    }

    fn word_prefix_matches(word: &str, input: &str) -> bool {
        let mut word_characters = word.chars();
        input.chars().all(|input_character| {
            word_characters
                .next()
                .map(|word_character| word_character.eq_ignore_ascii_case(&input_character))
                .unwrap_or(false)
        })
    }
}
