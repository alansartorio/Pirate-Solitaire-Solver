use std::fmt::Display;

use itertools::Itertools;

use crate::state::{Beast, Card, CardColor, CardOrBundle, NormalCard, State};

fn u8_to_digit(num: u8) -> char {
    num.to_string().chars().into_iter().next().unwrap()
}

impl Card {
    fn as_chars(&self) -> [char; 2] {
        match self {
            Card::Pirate => [' ', 'p'],
            Card::Beast(beast) => [
                ' ',
                match beast {
                    Beast::Whale => 'w',
                    Beast::Tentacle => 't',
                    Beast::Ship => 's',
                },
            ],
            Card::Normal(normal_card) => [
                u8_to_digit(normal_card.number.0),
                match normal_card.color {
                    CardColor::Red => 'r',
                    CardColor::Teal => 't',
                    CardColor::Blue => 'b',
                },
            ],
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_stack_height = self
            .board
            .iter()
            .map(|stack| stack.cards.len())
            .max()
            .expect("there's at least one stack");

        let card_width: usize = 2;

        let separation = 2;

        let mut table =
            vec![vec![' '; (card_width + 1) * 6 + 1]; max_stack_height * 2 + separation + 1];

        for (x, placeholder) in self.placeholders.holes.iter().enumerate() {
            if let Some(card) = placeholder.0 {
                let chars = match card {
                    CardOrBundle::Card(card) => card.as_chars(),
                    CardOrBundle::BeastBundle(_beast) => ['#', '#'],
                };
                let start = x * (card_width + 1) + 1;
                table[1][start..start + 2].copy_from_slice(&chars);
            }
        }

        for (x, stack) in self.board.iter().enumerate() {
            for (y, card) in stack.cards.iter().enumerate() {
                let start = x * (card_width + 1) + 1;
                let y = y * 2 + separation + 1;
                table[y][start..start + 2].copy_from_slice(&card.as_chars());
            }
        }

        let colors = [CardColor::Blue, CardColor::Teal, CardColor::Red];
        for (x, (color, highest_number)) in colors
            .into_iter()
            .map(|color| (color, self.output[&color]))
            .enumerate()
        {
            if highest_number.0 > 0 {
                let start = (x + 3) * (card_width + 1) + 1;
                table[1][start..start + 2].copy_from_slice(
                    &Card::Normal(NormalCard {
                        color,
                        number: highest_number,
                    })
                    .as_chars(),
                );
            }
        }

        f.write_str(table.into_iter().map(String::from_iter).join("\n").as_str())
    }
}
