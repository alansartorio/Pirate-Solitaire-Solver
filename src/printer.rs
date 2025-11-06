use std::fmt::Display;

use itertools::Itertools;

use crate::{
    ansi,
    denormalized::DenormalizedState,
    state::{Beast, Card, CardColor, CardOrBundle, NormalCard, State},
};

fn u8_to_digit(num: u8) -> char {
    num.to_string().chars().into_iter().next().unwrap()
}

impl Card {
    fn as_chars(&self) -> (ansi::Color, [char; 2]) {
        match self {
            Card::Pirate => (ansi::Color::White, [' ', 'p']),
            Card::Beast(beast) => (
                ansi::Color::White,
                [
                    ' ',
                    match beast {
                        Beast::Whale => 'w',
                        Beast::Tentacle => 't',
                        Beast::Ship => 's',
                    },
                ],
            ),
            Card::Normal(normal_card) => (
                match normal_card.color {
                    CardColor::Red => ansi::Color::Red,
                    CardColor::Teal => ansi::Color::Cyan,
                    CardColor::Blue => ansi::Color::Blue,
                },
                [
                    u8_to_digit(normal_card.number.0),
                    match normal_card.color {
                        CardColor::Red => 'r',
                        CardColor::Teal => 't',
                        CardColor::Blue => 'b',
                    },
                ],
            ),
        }
    }
}

impl Display for DenormalizedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_stack_height = self
            .board
            .iter()
            .map(|stack| stack.cards.len())
            .max()
            .expect("there's at least one stack");

        let card_width: usize = 2;

        let separation = 2;

        let mut table = ansi::Matrix::with_size(
            max_stack_height * 2 + separation + 1 + 1,
            (card_width + 1) * 6 + 1,
        );

        for (x, placeholder) in self.placeholders.holes.iter().enumerate() {
            if let Some(card) = placeholder.0 {
                let (color, chars) = match card {
                    CardOrBundle::Card(card) => card.as_chars(),
                    CardOrBundle::BeastBundle(_beast) => (ansi::Color::White, ['#', '#']),
                };
                let start = x * (card_width + 1) + 1;
                table.arr[1][start..start + 2].copy_from_slice(&chars.map(|c| (color, c)));
            }
        }

        for (x, stack) in self.board.iter().enumerate() {
            for (y, card) in stack.cards.iter().enumerate() {
                let start = x * (card_width + 1) + 1;
                let y = y * 2 + separation + 1 + 1;
                let (color, chars) = card.as_chars();
                table.arr[y][start..start + 2].copy_from_slice(&chars.map(|c| (color, c)));
            }
        }

        let colors = [CardColor::Blue, CardColor::Red, CardColor::Teal];
        for (x, (color, highest_number)) in colors
            .into_iter()
            .map(|color| (color, self.output[&color]))
            .enumerate()
        {
            if highest_number.0 > 0 {
                let start = (x + 3) * (card_width + 1) + 1;

                let (color, chars) = Card::Normal(NormalCard {
                    color,
                    number: highest_number,
                })
                .as_chars();
                table.arr[1][start..start + 2].copy_from_slice(&chars.map(|c| (color, c)));
            }
        }

        table.fmt(f)
    }
}
