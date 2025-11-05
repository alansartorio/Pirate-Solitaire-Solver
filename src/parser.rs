use std::str::FromStr;

use anyhow::{Context, anyhow};

use crate::state::{Beast, Card, CardColor, CardNumber, CardStack, NormalCard};

impl FromStr for CardColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "t" => Ok(CardColor::Teal),
            "r" => Ok(CardColor::Red),
            "b" => Ok(CardColor::Blue),
            invalid_color => Err(anyhow!("invalid card color: {invalid_color}")),
        }
    }
}

impl FromStr for CardNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.parse()
                .map_err(|_| anyhow!("error parsing card number"))?,
        ))
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "p" => Ok(Card::Pirate),

            "w" => Ok(Card::Beast(Beast::Whale)),
            "s" => Ok(Card::Beast(Beast::Ship)),
            "t" => Ok(Card::Beast(Beast::Tentacle)),

            normal_card => {
                let [number, color]: [char; 2] = normal_card
                    .chars()
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| anyhow!("invalid card: {normal_card}"))?;

                let number = number.to_string().parse()?;
                let color = color.to_string().parse()?;

                Ok(Self::Normal(NormalCard { number, color }))
            }
        }
    }
}

impl FromStr for CardStack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 6] = s
            .split(" ")
            .map(|c| c.parse())
            .take(6)
            .collect::<Result<Vec<_>, _>>()
            .context("parsing stack")?
            .try_into()
            .map_err(|_| anyhow!("stack size must be 6"))?;

        Ok(Self {
            cards: cards.to_vec(),
        })
    }
}
