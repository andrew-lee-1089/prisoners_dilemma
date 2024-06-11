use crate::common::{Player, Round, RoundChoice, StrategyType, BOTH_COOPERATE_ROUND};
use rand::Rng;

pub fn always_cooperate_strategy(_rounds: &Vec<Round>) -> RoundChoice {
    RoundChoice::Cooperate
}

pub fn always_steal_strategy(_rounds: &Vec<Round>) -> RoundChoice {
    RoundChoice::Steal
}

pub fn random_strategy(_rounds: &Vec<Round>) -> RoundChoice {
    if rand::thread_rng().gen_range(0..100) < 50 {
        return RoundChoice::Steal;
    }
    RoundChoice::Cooperate
}

// Orginally assume good faith, but mirror what your opponent did last time
pub fn tit_for_tat_strategy(rounds: &Vec<Round>) -> RoundChoice {
    rounds.last().unwrap_or(&BOTH_COOPERATE_ROUND).choices.1
}

// Orginally assume good faith, but if wronged, steal for rest of game
pub fn once_bitten_twice_shy(rounds: &Vec<Round>) -> RoundChoice {
    if rounds
        .iter()
        .any(|round| round.choices.1 == RoundChoice::Steal)
    {
        return RoundChoice::Steal;
    }
    RoundChoice::Cooperate
}

// Assume good faith, but if stolen from, but need two rounds where opponent cooperates to forgive
pub fn i_need_time_to_forgive(rounds: &Vec<Round>) -> RoundChoice {
    if rounds
        .iter()
        .map(|round| round.choices.1)
        .rev()
        .take(2)
        .any(|choice| choice == RoundChoice::Steal)
    {
        return RoundChoice::Steal;
    }
    RoundChoice::Cooperate
}

// Be good 90% of the time, and steal 10% of the time
pub fn mostly_good(_rounds: &Vec<Round>) -> RoundChoice {
    if rand::thread_rng().gen_range(0..100) < 10 {
        return RoundChoice::Steal;
    }
    RoundChoice::Cooperate
}

// If opponent stole, 50% of the time reteliate
pub fn maybe_retaliate(rounds: &Vec<Round>) -> RoundChoice {
    if rounds.last().unwrap_or(&BOTH_COOPERATE_ROUND).choices.1 == RoundChoice::Steal {
        if rand::thread_rng().gen_range(0..100) < 50 {
            return RoundChoice::Steal;
        }
    }
    RoundChoice::Cooperate
}

// If opponent stole, 90% of the time reteliate
pub fn mainly_retaliate(rounds: &Vec<Round>) -> RoundChoice {
    if rounds.last().unwrap_or(&BOTH_COOPERATE_ROUND).choices.1 == RoundChoice::Steal {
        if rand::thread_rng().gen_range(0..100) < 90 {
            return RoundChoice::Steal;
        }
    }
    RoundChoice::Cooperate
}

// If opponent stole, 90% of the time reteliate
pub fn rarely_retaliate(rounds: &Vec<Round>) -> RoundChoice {
    if rounds.last().unwrap_or(&BOTH_COOPERATE_ROUND).choices.1 == RoundChoice::Steal {
        if rand::thread_rng().gen_range(0..100) < 10 {
            return RoundChoice::Steal;
        }
    }
    RoundChoice::Cooperate
}

pub fn get_strategy(strategy_type: StrategyType) -> Box<dyn Fn(&Vec<Round>) -> RoundChoice> {
    match strategy_type {
        StrategyType::AlwaysCooperate => Box::new(always_cooperate_strategy),
        StrategyType::Random => Box::new(random_strategy),
        StrategyType::AlwaysSteal => Box::new(always_steal_strategy),
        StrategyType::TitForTat => Box::new(tit_for_tat_strategy),
        StrategyType::OneBittenTwiceShy => Box::new(once_bitten_twice_shy),
        StrategyType::TakesTimeToForgive => Box::new(i_need_time_to_forgive),
        StrategyType::MostlyGood => Box::new(mostly_good),
        StrategyType::MaybeRelaliate => Box::new(maybe_retaliate),
        StrategyType::MainlyRelaliate => Box::new(mainly_retaliate),
        StrategyType::RarelyRelaliate => Box::new(rarely_retaliate),
    }
}

impl Player {
    pub fn new_from_strategy_name(strategy_type: StrategyType) -> Self {
        Player::new(strategy_type, get_strategy(strategy_type))
    }
}
