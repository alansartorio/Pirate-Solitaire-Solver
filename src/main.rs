use std::io::{BufRead, stdin};

use anyhow::{Context, anyhow};

use crate::{
    solver::solve,
    state::{CardStack, Output, PlaceHolders, State},
};

mod actions;
mod parser;
mod printer;
mod solver;
mod state;
mod validators;
mod collection;

fn main() -> anyhow::Result<()> {
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

    let state = State {
        placeholders: PlaceHolders::default(),
        output: Output::default(),
        board: initial_stacks.into(),
    };

    //dbg!(&state);
    println!("{state}");

    state.is_valid().context("validation error")?;

    //dbg!(state.get_next_states().collect::<Vec<_>>().len());
    //for state in state.get_next_states() {
    //println!("{state}");
    //}

    if let Some(solution) = solve(&state) {
        for (i, step) in solution.into_iter().enumerate() {
            println!("step {i}:");
            println!("{step}");
        }
    } else {
        println!("no solution");
    }

    Ok(())
}
