use std::{
    fs::read,
    io::{BufRead, stdin},
};

use anyhow::{Context, anyhow};

use crate::{
    deck_generator::generate_game,
    denormalized::DenormalizedState,
    solver::solve,
    state::{CardStack, Output, PlaceHolders, State},
};

mod actions;
mod collection;
mod deck_generator;
mod denormalized;
mod godot_shuffle;
mod parser;
mod printer;
mod solver;
mod state;
mod validators;

fn read_from_stdin() -> anyhow::Result<DenormalizedState> {
    let stdin = stdin().lock();

    let initial_stacks: [CardStack; 6] = stdin
        .lines()
        .map(|line| {
            line.map_err(|read_error| anyhow!("line read error: {read_error}"))?
                .parse()
                .context("parsing card stack")
        })
        .take(6)
        .collect::<Result<Vec<_>, _>>()
        .context("parsing stacks")?
        .try_into()
        .map_err(|_| anyhow!("not enough card stacks"))?;

    let state = DenormalizedState {
        placeholders: Default::default(),
        output: Output::default(),
        board: initial_stacks.into(),
    };

    Ok(state)
}

fn read_from_seed_stdin() -> anyhow::Result<DenormalizedState> {
    let mut stdin = stdin().lock();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let seed = line.trim().parse()?;

    let state = generate_game(seed);

    Ok(state)
}

fn main() -> anyhow::Result<()> {
    let state = read_from_seed_stdin()?;
    //let state = read_from_stdin()?;
    println!("{state}");

    let (state, denormalization_information) = state.normalize();

    //dbg!(&state);

    state.is_valid().context("validation error")?;

    if let Some(solution) = solve(&state) {
        for (i, step) in solution.into_iter().enumerate() {
            println!();
            println!("==============");
            println!();
            println!("STEP {i}:");
            println!("{}", step.denormalize(&denormalization_information));
        }
    } else {
        println!("no solution");
    }

    Ok(())
}
