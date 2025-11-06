use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::collection::BTreeMultiSet;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct CardNumber(pub u8);

impl CardNumber {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn next_inplace(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum Beast {
    Whale,
    Tentacle,
    Ship,
}

impl Display for Beast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Beast::Whale => "whale",
            Beast::Tentacle => "tentacle",
            Beast::Ship => "ship",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum CardColor {
    Red,
    Teal,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct NormalCard {
    pub number: CardNumber,
    pub color: CardColor,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum Card {
    Pirate,
    Beast(Beast),
    Normal(NormalCard),
}

impl Card {
    pub fn normal(&self) -> Option<&NormalCard> {
        if let Card::Normal(normal) = self {
            Some(normal)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum CardOrBundle {
    Card(Card),
    BeastBundle(Beast),
}

#[derive(Debug, Clone, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct CardStack {
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Output {
    pub by_color: [CardNumber; 3],
}

impl Default for Output {
    fn default() -> Self {
        Self {
            by_color: [0; 3].map(CardNumber),
        }
    }
}

impl Index<&CardColor> for Output {
    type Output = CardNumber;
    fn index(&self, index: &CardColor) -> &Self::Output {
        &self.by_color[*index as usize]
    }
}

impl IndexMut<&CardColor> for Output {
    fn index_mut(&mut self, index: &CardColor) -> &mut Self::Output {
        &mut self.by_color[*index as usize]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct PlaceHolder(pub Option<CardOrBundle>);

#[derive(Debug, Clone, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct PlaceHolders {
    pub holes: BTreeMultiSet<PlaceHolder>,
}

impl Default for PlaceHolders {
    fn default() -> Self {
        Self {
            holes: BTreeMultiSet::from_iter([PlaceHolder::default(); 3]),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct State {
    pub placeholders: PlaceHolders,
    pub output: Output,
    pub board: BTreeMultiSet<CardStack>,
}
