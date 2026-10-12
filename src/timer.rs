use crate::game::GameState;

#[derive(Debug)]
pub(crate) struct Timer {
    remaining_seconds: f32,
}

impl Timer {
    pub(crate) fn new(duration_seconds: u32) -> Self {
        Self {
            remaining_seconds: duration_seconds as f32,
        }
    }

    pub(crate) fn update(&mut self, delta_seconds: f32, game_state: &mut GameState) {
        if delta_seconds.is_finite() && delta_seconds > 0.0 {
            self.remaining_seconds = (self.remaining_seconds - delta_seconds).max(0.0);
        }
        if self.remaining_seconds == 0.0 {
            *game_state = GameState::TotalPoints {input: "".parse().unwrap() };
        }
    }

    pub(crate) fn remaining_seconds(&self) -> u32 {
        self.remaining_seconds.ceil() as u32
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.remaining_seconds <= 0.0
    }

    pub(crate) fn display(&self) -> String {
        let seconds = self.remaining_seconds();
        format!("{:02}:{:02}", seconds / 60, seconds % 60)
    }
}
