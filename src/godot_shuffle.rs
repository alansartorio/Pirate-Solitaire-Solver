use std::str::FromStr;

use anyhow::Context;
use rand::RngCore;
use rand_pcg::Pcg32;

pub struct Seed(i64);

impl FromStr for Seed {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, mul) = match s.strip_prefix('_') {
            Some(neg_num) => (neg_num, -1),
            None => (s, 1),
        };
        let seed = num
            .parse::<u64>()
            .with_context(|| format!("parsing value: {num}"))? as i64
            * mul;
        Ok(Seed(seed))
    }
}

pub fn shuffle<T>(deck: &mut [T], seed: Seed) {
    let seed: u64 = i64::cast_unsigned(seed.0);
    let mut pcg = Pcg32::new(seed, 1442695040888963407);

    if deck.len() <= 1 {
        return;
    }

    let mut i = deck.len() - 1;
    while i >= 1 {
        let r = pcg.next_u32();
        let j = r as usize % (i + 1);
        deck.swap(i, j);

        i -= 1;
    }
}
