pub mod martingale;
use martingale::{EndCondition, GameSetting, martingale_main};
fn main() {
    let end= EndCondition {
        n_simulations: 1000,
        max_games: 365,
        init_wealth: 100_000.0,
    };
    let game = GameSetting {
        win_rate: 0.5,
        odds: 1.75,
        stake: 100.0,
    };
    martingale_main(&end, &game);
}