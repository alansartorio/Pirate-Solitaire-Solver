use pathfinding::prelude::*;

use crate::state::{CardOrBundle, State};

impl State {
    fn is_win(&self) -> bool {
        self.board.iter().all(|stack| stack.cards.is_empty())
    }
}

pub fn solve(start: &State) -> Option<Vec<State>> {
    // Faster, but worse solutions
    //dfs(
    //start.clone(),
    //|state: &State| state.get_next_states().collect::<Vec<_>>(),
    //|state: &State| state.is_win(),
    //)
    astar(
        start,
        |state: &State| {
            state
                .get_next_states()
                .map(|state| {
                    println!("{state}");
                    state
                })
                .map(|state| (state, 1))
                .collect::<Vec<_>>()
        },
        |state: &State| {
            state
                .output
                .by_color
                .iter()
                .map(|num| (9 - num.0) as usize)
                .sum::<usize>()
                + state
                    .placeholders
                    .holes
                    .iter()
                    .map(|hole| matches!(hole.0, Some(CardOrBundle::BeastBundle(_))) as usize)
                    .sum::<usize>()
        },
        |state: &State| state.is_win(),
    )
    .map(|solution| solution.0)
}
