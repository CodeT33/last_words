use crate::desktop::Desktop;
use crate::player::Player;

struct GameContext {
    desktop: Desktop,
    player: Player,
    game_state: GameState,
}

enum GameState {
    Paused,
    SelectStack,
    WriteLeaf,
    End,
}