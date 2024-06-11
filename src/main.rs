mod common;
mod players;
use std::collections::HashMap;

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

    //play_multi_season(player_pool, 1);
    play_with_noise(player_pool);

    // let final_score = play_season(&player_pool);

    // for score in final_score {
    //     println!("{} - average score: {}", score.0.name, score.1)
    // }
}

fn play_with_noise(players: Vec<Player>) {
    let mut results: HashMap<String, Vec<f32>> = HashMap::new();
    for noise in (0..=100).step_by(10) {
        let final_score = play_season(&players, noise);
        for (player, player_average_score) in final_score {
            let mut player_vec = results.get(&player.strategy_type.to_string()).unwrap_or(&vec!()).to_owned();
            player_vec.push(player_average_score);
            results.insert(player.strategy_type.to_string(), player_vec);
        }
    }
    for (key, value) in results
    {
        println!("{},{}", key, value.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(","));
    }
}

fn play_multi_season(players: Vec<Player>, season_length: i32) {
    let mut new_season_players = players;
    for i in 0..season_length {
        println!("Season {}", i + 1);
        let final_score = play_season(&new_season_players, 100);
        let mut new_players_to_add: Vec<StrategyType> = vec![];
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
