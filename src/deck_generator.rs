use std::{array, collections::VecDeque};

use itertools::Itertools;

use crate::{
    godot_shuffle::{self, Seed},
    state::{Beast, Card, CardColor, CardNumber, CardStack, NormalCard, State},
};

fn new_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];

    deck.extend(
        (1..=9)
            .cartesian_product([CardColor::Teal, CardColor::Red, CardColor::Blue])
            .map(|(number, color)| {
                Card::Normal(NormalCard {
                    number: CardNumber(number),
                    color,
                })
            }),
    );

    deck.extend(std::iter::repeat_n(Card::Pirate, 6));

    deck.extend([Beast::Tentacle, Beast::Whale, Beast::Ship].map(Card::Beast));

    deck
}

fn distribute(deck: Vec<Card>) -> State {
    let mut deck = VecDeque::from(deck);
    let mut stacks: [CardStack; 6] = array::repeat(CardStack { cards: vec![] });
    let mut stack_index_iterator = (0..stacks.len()).cycle();

    while let Some(card) = deck.pop_front() {
        let stack = &mut stacks[stack_index_iterator.next().unwrap()];
        stack.cards.push(card);
    }

    State {
        board: stacks.into(),
        output: Default::default(),
        placeholders: Default::default(),
    }
}

pub fn generate_game(seed: Seed) -> State {
    let mut deck = new_deck();
    godot_shuffle::shuffle(&mut deck, seed);
    distribute(deck)
}
