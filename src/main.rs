mod common;
mod players;
use common::{play_season, Player, StrategyType};

fn main() {
    let player_pool: Vec<Player> = vec![
        Player::new_from_strategy_name(StrategyType::AlwaysCooperate),
        Player::new_from_strategy_name(StrategyType::Random),
        Player::new_from_strategy_name(StrategyType::AlwaysSteal),
        Player::new_from_strategy_name(StrategyType::TitForTat),
        Player::new_from_strategy_name(StrategyType::OneBittenTwiceShy),
        Player::new_from_strategy_name(StrategyType::TakesTimeToForgive),
        Player::new_from_strategy_name(StrategyType::MostlyGood),
        Player::new_from_strategy_name(StrategyType::MaybeRelaliate),
        Player::new_from_strategy_name(StrategyType::MainlyRelaliate),
        Player::new_from_strategy_name(StrategyType::RarelyRelaliate),
    ];

    play_multi_season(player_pool, 10);

    // let final_score = play_season(&player_pool);

    // for score in final_score {
    //     println!("{} - average score: {}", score.0.name, score.1)
    // }
}

fn play_multi_season(players: Vec<Player>, season_length: i32) {
    let mut new_season_players = players;
    for i in 0..season_length {
        println!("Season {}", i+1);
        let final_score = play_season(&new_season_players);
        let mut new_players_to_add: Vec<StrategyType> = vec!();
        for (player, player_average_score) in final_score {
            println!("{} - average score: {}", player.name, player_average_score);
            if player_average_score > 2.7 {
                new_players_to_add.push(player.strategy_type.clone())  
                }
            if player_average_score > 3.0 {
                new_players_to_add.push(player.strategy_type.clone())  
            }
        }
        for strategy_type in new_players_to_add {
            new_season_players.push(Player::new_from_strategy_name(strategy_type))
        }
    }
}

//TODO:
// Add noise
// Add ML
