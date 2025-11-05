use std::collections::HashMap;

use crate::state::{Beast, Card, CardColor, CardNumber, NormalCard, State};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidInitialState {
    #[error("the {card:?} card count is not valid ({count}, expected = {expected})")]
    InvalidCardAmount {
        card: Card,
        count: usize,
        expected: usize,
    },
}

struct CountTrack {
    actual: usize,
    expected: usize,
}

impl State {
    pub fn is_valid(&self) -> Result<(), InvalidInitialState> {
        let cards: Vec<_> = self
            .board
            .iter()
            .flat_map(|stack| stack.cards.iter())
            .collect();

        let numbers = (1..=9).map(CardNumber);
        let colors = [CardColor::Teal, CardColor::Red, CardColor::Blue];
        let normal_cards = numbers
            .flat_map(|number| colors.map(|color| Card::Normal(NormalCard { color, number })))
            .map(|card| (card, 1));
        let beast_cards = [Beast::Ship, Beast::Whale, Beast::Tentacle]
            .map(Card::Beast)
            .map(|card| (card, 1));
        let pirate_cards = [Card::Pirate].map(|card| (card, 6));

        let mut counts =
            HashMap::<_, _>::from_iter(normal_cards.chain(beast_cards).chain(pirate_cards).map(
                |(card, expected)| {
                    (
                        card,
                        CountTrack {
                            actual: 0,
                            expected,
                        },
                    )
                },
            ));

        for card in &cards {
            counts.get_mut(card).unwrap().actual += 1;
        }

        for (card, CountTrack { actual, expected }) in counts {
            if expected != actual {
                return Err(InvalidInitialState::InvalidCardAmount {
                    card,
                    count: actual,
                    expected,
                });
            }
        }

        Ok(())
    }
}
