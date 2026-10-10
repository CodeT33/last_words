use crate::consts::CURSOR_SPEED;
use raylib::prelude::Vector2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CharacterState {
    Untyped,
    Correct,
    Incorrect,
}

#[derive(Debug)]
pub struct TypingSession {
    pub(crate) target: Vec<char>,
    pub(crate) states: Vec<CharacterState>,
    pub(crate) cursor: usize,
    cursor_position: Option<Vector2>,
}

impl TypingSession {
    pub fn new(target: String) -> Self {
        let target: Vec<char> = target.chars().collect();
        Self {
            states: vec![CharacterState::Untyped; target.len()],
            target,
            cursor: 0,
            cursor_position: None,
        }
    }

    pub(crate) fn animate_cursor(&mut self, target: Vector2, frame_time: f32) -> Vector2 {
        let position = self.cursor_position.get_or_insert(target);
        let smoothing = 1.0 - (-CURSOR_SPEED * frame_time.max(0.0)).exp();
        position.x += (target.x - position.x) * smoothing;
        position.y += (target.y - position.y) * smoothing;
        *position
    }

    pub fn type_character(&mut self, character: char) {
        if self.is_complete() || (character.is_control() && character != '\n') {
            return;
        }

        self.states[self.cursor] = if character == self.target[self.cursor] {
            CharacterState::Correct
        } else {
            CharacterState::Incorrect
        };
        self.cursor += 1;
    }

    pub fn undo_last_character(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.states[self.cursor] = CharacterState::Untyped;
    }

    pub fn typed_characters(&self) -> usize {
        self.cursor
    }

    pub fn character_count(&self) -> usize {
        self.target.len()
    }

    pub fn is_complete(&self) -> bool {
        self.cursor == self.target.len()
    }

    pub fn is_perfect(&self) -> bool {
        self.is_complete()
            && self
                .states
                .iter()
                .all(|state| *state == CharacterState::Correct)
    }

    pub fn typing_errors(&self) -> usize {
        self.states
            .iter()
            .filter(|state| **state == CharacterState::Incorrect)
            .count()
    }
}
