use itertools::Itertools;
use rand::{distributions::Alphanumeric, Rng};
use std::{collections::HashMap, fmt};
use enum_derived::Rand;

pub struct Player {
    pub name: String,
    pub strategy_type: StrategyType,
    pub strategy: Box<dyn Fn(&Vec<Round>) -> RoundChoice>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrategyType {
    AlwaysCooperate,
    Random,
    AlwaysSteal,
    TitForTat,
    OneBittenTwiceShy,
    TakesTimeToForgive,
    MostlyGood,
    MaybeRelaliate,
    MainlyRelaliate,
    RarelyRelaliate,
}

impl fmt::Display for StrategyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Player {
    pub fn new(
        strategy_type: StrategyType,
        strategy: Box<dyn Fn(&Vec<Round>) -> RoundChoice>,
    ) -> Self {
        let suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        Player {
            name: format!("{strategy_type:?}-{suffix}"),
            strategy_type: strategy_type,
            strategy: strategy,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct PrivateRound {
    pub round: Round,
    pub has_noise: bool,
    pub round_with_noise: Round
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Round {
    pub choices: (RoundChoice, RoundChoice),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Rand)]
pub enum RoundChoice {
    Cooperate,
    Steal,
}

impl RoundChoice {
    pub fn get_opposite(&self) -> RoundChoice {
        if self == &RoundChoice::Cooperate {
            return RoundChoice::Steal;
        }
        RoundChoice::Cooperate
    }
}

pub const BOTH_COOPERATE_ROUND: Round = Round {
    choices: (RoundChoice::Cooperate, RoundChoice::Cooperate),
};
pub const FIRST_PLAYER_STEALS_ROUND: Round = Round {
    choices: (RoundChoice::Steal, RoundChoice::Cooperate),
};
pub const SECOND_PLAYER_STEALS_ROUND: Round = Round {
    choices: (RoundChoice::Cooperate, RoundChoice::Steal),
};
pub const BOTH_STEAL_ROUND: Round = Round {
    choices: (RoundChoice::Steal, RoundChoice::Steal),
};

impl PrivateRound {
    pub fn get_round_history(&self) -> Round {
        if self.has_noise {
            Round {
                choices: (self.round.choices.0, self.round_with_noise.choices.1),
            }
        } else {
            self.round
        }
    }

    pub fn get_round_history_for_b(&self) -> Round {
        if self.has_noise {
            Round {
                choices: (self.round.choices.1, self.round_with_noise.choices.0),
            }
        } else {
            Round {
                choices: (self.round.choices.1, self.round.choices.0),
            }
        }
    }
}

pub fn play_round(
    players: (&Player, &Player),
    private_rounds: &Vec<PrivateRound>,
    noise: i32,
) -> PrivateRound {
    PrivateRound {
        round: Round {
            choices: (
                (players.0.strategy)(
                    &private_rounds
                        .iter()
                        .map(|p| p.get_round_history())
                        .collect(),
                ),
                (players.1.strategy)(
                    &private_rounds
                        .iter()
                        .map(|p| p.get_round_history_for_b())
                        .collect(),
                ),
            ),
        },
        has_noise: rand::thread_rng().gen_range(0..100) < noise,
        round_with_noise: Round {
            choices: (
                RoundChoice::rand(), RoundChoice::rand()
            )
        }
    }
}

fn round_outcome(round: &Round) -> (i32, i32) {
    match round {
        &BOTH_COOPERATE_ROUND => (3, 3),
        &FIRST_PLAYER_STEALS_ROUND => (5, 0),
        &SECOND_PLAYER_STEALS_ROUND => (0, 5),
        &BOTH_STEAL_ROUND => (1, 1),
    }
}

pub fn get_scores(rounds: Vec<PrivateRound>) -> (i32, i32) {
    rounds
        .iter()
        .map(|private_round| round_outcome(&private_round.round))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}

fn play_match(players: (&Player, &Player), noise: i32) -> (i32, i32) {
    let mut rounds: Vec<PrivateRound> = vec![];

    for _i in 0..200 {
        let round = play_round(players, &rounds, noise);
        rounds.push(round);
    }

    get_scores(rounds)
}

pub fn play_season(players: &Vec<Player>, noise: i32) -> Vec<(&Player, f32)> {
    let mut results: HashMap<String, Vec<i32>> = HashMap::new();

    for game in players
        .iter()
        .combinations_with_replacement(2)
        .map(|v| (v[0], v[1]))
    {
        let result = play_match(game, noise);
        // println!(
        //     "{}: {}, {} {}",
        //     game.0.name, result.0, game.1.name, result.1
        // );

        if results.contains_key(&game.0.name) {
            results.get_mut(&game.0.name).unwrap().push(result.0);
        } else {
            results.insert(game.0.name.clone(), vec![result.0]);
        }
        if results.contains_key(&game.1.name) {
            results.get_mut(&game.1.name).unwrap().push(result.1);
        } else {
            results.insert(game.1.name.clone(), vec![result.1]);
        }
    }

    let mut final_score: Vec<(&Player, f32)> = vec![];

    for (player_name, player_results) in results.iter() {
        final_score.push((
            players
                .iter()
                .find(|player| &player.name == player_name)
                .unwrap(),
            player_results.iter().sum::<i32>() as f32 / (200 * player_results.len()) as f32,
        ))
    }

    final_score.sort_by(|a, b| f32::total_cmp(&a.1, &b.1));

    final_score
}
