use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, stdin},
    path::PathBuf,
};

use anyhow::{Context, anyhow};

use crate::{
    deck_generator::generate_game,
    denormalized::DenormalizedState,
    solver::solve,
    state::{CardStack, Output},
};

mod actions;
mod ansi;
mod collection;
mod deck_generator;
mod denormalized;
mod godot_shuffle;
mod parser;
mod printer;
mod solver;
mod state;
mod validators;

fn read_from(read: impl BufRead) -> anyhow::Result<DenormalizedState> {
    let initial_stacks: [CardStack; 6] = read
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Args {
    #[command(name = "seed", about = "load scramble from seed")]
    Seed { seed: String },
    #[command(name = "cards", about = "load scramble from card disposition")]
    Cards { file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let state = match args {
        Args::Seed { seed } => generate_game(seed.parse()?),
        Args::Cards { file } => read_from(BufReader::new(File::open(file)?))?,
    };
    println!("{state}");

    let (state, denormalization_information) = state.normalize();

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
