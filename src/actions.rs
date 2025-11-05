use itertools::{Either, Itertools};

use crate::state::{Card, CardOrBundle, CardStack, NormalCard, Output, PlaceHolder, State};

#[derive(Clone, Copy)]
enum CardLocation<'a> {
    Stack(&'a CardStack),
    PlaceHolder(&'a PlaceHolder),
}

impl State {
    fn pop(&mut self, location: &CardLocation<'_>) -> Option<Card> {
        match location {
            CardLocation::Stack(stack) => {
                let mut stack_from = self.board.take(stack).unwrap();
                let card = stack_from.cards.pop();
                self.board.insert(stack_from);

                card
            }
            CardLocation::PlaceHolder(hole) => {
                let mut hole_from = self.placeholders.holes.take(hole).unwrap();
                let card = match hole_from.0.take() {
                    Some(CardOrBundle::Card(card)) => Some(card),
                    _ => None,
                };
                self.placeholders.holes.insert(hole_from);

                card
            }
        }
    }
}

enum CardSplitOrigin<'a> {
    Stack {
        location: &'a CardStack,
        cards: &'a [Card],
    },
    PlaceHolder {
        location: &'a PlaceHolder,
        card: &'a Card,
    },
}

impl Output {
    fn can_output_card(&self, NormalCard { number, color }: &NormalCard) -> bool {
        &self[color].next() == number
    }
}

impl Card {
    fn can_stack_with(&self, card_to_place: &Card) -> bool {
        self.normal()
            .zip(card_to_place.normal())
            .is_some_and(|(card, card_to_place)| {
                card.number == card_to_place.number.next() && card.color != card_to_place.color
            })
    }
}

impl CardStack {
    fn can_stack_with(&self, card_to_place: &Card) -> bool {
        self.cards
            .last()
            .is_none_or(|card| card.can_stack_with(card_to_place))
    }
}

impl State {
    fn valid_card_outputs(&self) -> impl Iterator<Item = (CardLocation<'_>, &'_ NormalCard)> {
        self.board
            .iter()
            .filter_map(move |stack| {
                stack
                    .cards
                    .last()
                    .and_then(|card| card.normal())
                    .and_then(move |card| {
                        self.output
                            .can_output_card(card)
                            .then_some((CardLocation::Stack(stack), card))
                    })
            })
            .chain(self.placeholders.holes.iter().filter_map(|hole| {
                if let Some(CardOrBundle::Card(Card::Normal(ref card))) = hole.0
                    && self.output.can_output_card(card)
                {
                    Some((CardLocation::PlaceHolder(hole), card))
                } else {
                    None
                }
            }))
    }

    fn valid_card_splits(&self) -> impl Iterator<Item = CardSplitOrigin<'_>> {
        self.board
            .iter()
            .flat_map(|stack| {
                let max_split_size = match stack.cards.len() {
                    0 => 0,
                    _ => {
                        stack
                            .cards
                            .iter()
                            .rev()
                            .skip(1)
                            .zip(stack.cards.iter().rev())
                            .take_while(|(prev, top)| prev.can_stack_with(top))
                            .count()
                            + 1
                    }
                };
                (1..=max_split_size).map(|split_size| CardSplitOrigin::Stack {
                    location: stack,
                    cards: &stack.cards[stack.cards.len() - split_size..],
                })
            })
            .chain(self.placeholders.holes.iter().flat_map(|hole| {
                hole.0.as_ref().and_then(|card_or_bundle| {
                    if let CardOrBundle::Card(card) = card_or_bundle {
                        Some(CardSplitOrigin::PlaceHolder {
                            location: hole,
                            card,
                        })
                    } else {
                        None
                    }
                })
            }))
    }

    fn exposed_beasts(&self) -> impl Iterator<Item = CardLocation<'_>> {
        self.board
            .iter()
            .flat_map(|stack| {
                stack
                    .cards
                    .last()
                    .is_some_and(|card| matches!(card, Card::Beast(_)))
                    .then_some(CardLocation::Stack(stack))
            })
            .chain(self.placeholders.holes.iter().flat_map(|hole| {
                hole.0
                    .as_ref()
                    .is_some_and(|card| matches!(card, CardOrBundle::Card(Card::Beast(_))))
                    .then_some(CardLocation::PlaceHolder(hole))
            }))
    }

    fn exposed_pirates(&self) -> impl Iterator<Item = CardLocation<'_>> {
        self.board
            .iter()
            .flat_map(|stack| {
                stack
                    .cards
                    .last()
                    .is_some_and(|card| matches!(card, Card::Pirate))
                    .then_some(CardLocation::Stack(stack))
            })
            .chain(self.placeholders.holes.iter().flat_map(|hole| {
                hole.0
                    .as_ref()
                    .is_some_and(|card| matches!(card, CardOrBundle::Card(Card::Pirate)))
                    .then_some(CardLocation::PlaceHolder(hole))
            }))
    }

    pub fn get_next_states(&self) -> impl Iterator<Item = State> + '_ {
        let card_output_states = self.valid_card_outputs().map(|(card_location, card)| {
            let mut new_state = self.clone();
            match card_location {
                CardLocation::PlaceHolder(placeholder) => {
                    new_state.placeholders.holes.remove(placeholder);
                }
                CardLocation::Stack(card_stack) => {
                    let mut new_stack = new_state.board.take(card_stack).unwrap();
                    new_stack.cards.pop();
                    new_state.board.insert(new_stack);
                }
            }
            new_state.output[&card.color].next_inplace();
            new_state
        });

        let card_split_states =
            self.valid_card_splits()
                .flat_map(move |split_origin| match split_origin {
                    CardSplitOrigin::PlaceHolder { location, card } => {
                        Either::Left(self.board.iter().flat_map(|stack| {
                            stack.can_stack_with(card).then(|| {
                                let mut new_state = self.clone();

                                // remove from placeholder
                                let mut hole = new_state.placeholders.holes.take(location).unwrap();
                                hole.0 = None;
                                new_state.placeholders.holes.insert(hole);

                                // add to board stack
                                let mut stack = new_state.board.take(stack).unwrap();
                                stack.cards.push(*card);
                                new_state.board.insert(stack);

                                new_state
                            })
                        }))
                    }
                    CardSplitOrigin::Stack { location, cards } => {
                        Either::Right(self.board.iter().flat_map(move |stack| {
                            stack.can_stack_with(cards.first().unwrap()).then(|| {
                                let mut new_state = self.clone();

                                // remove from stack
                                let mut from_stack = new_state.board.take(location).unwrap();
                                let drained_cards = from_stack
                                    .cards
                                    .drain(from_stack.cards.len() - cards.len()..);
                                assert!(drained_cards.as_ref().eq(cards));
                                drop(drained_cards);
                                new_state.board.insert(from_stack);

                                // add to board stack
                                let mut stack = new_state.board.take(stack).unwrap();
                                stack.cards.extend_from_slice(cards);
                                new_state.board.insert(stack);

                                new_state
                            })
                        }))
                    }
                });

        let card_hold_states = self
            .board
            .iter()
            .filter(|stack| !stack.cards.is_empty())
            .flat_map(|stack| {
                self.placeholders
                    .holes
                    .iter()
                    .filter(|hole| hole.0.is_none())
                    .map(move |hole| (stack, hole))
            })
            .map(|(stack, hole)| {
                let mut new_state = self.clone();

                let mut stack_from = new_state.board.take(stack).unwrap();
                let card = stack_from.cards.pop().unwrap();
                new_state.board.insert(stack_from);

                let mut hole_into = new_state.placeholders.holes.take(hole).unwrap();
                hole_into.0 = Some(CardOrBundle::Card(card));
                new_state.placeholders.holes.insert(hole_into);

                new_state
            });

        let free_placeholder = self.placeholders.holes.iter().find(|hole| hole.0.is_none());

        let beast_promotion_states = self
            .exposed_beasts()
            .cartesian_product(
                self.exposed_pirates()
                    .collect_vec()
                    .into_iter()
                    .tuple_combinations(),
            )
            .flat_map(move |(beast_location, (pirate1, pirate2))| {
                match (beast_location, free_placeholder) {
                    (CardLocation::Stack(_stack), Some(free_placeholder)) => {
                        let mut new_state = self.clone();
                        let Card::Beast(beast) = new_state.pop(&beast_location).unwrap() else {
                            unreachable!("should be a beast");
                        };
                        let pirate1 = new_state.pop(&pirate1).unwrap();
                        assert_eq!(pirate1, Card::Pirate);
                        let pirate2 = new_state.pop(&pirate2).unwrap();
                        assert_eq!(pirate2, Card::Pirate);

                        let mut bundle_place =
                            new_state.placeholders.holes.take(free_placeholder).unwrap();
                        bundle_place.0 = Some(CardOrBundle::BeastBundle(beast));
                        new_state.placeholders.holes.insert(bundle_place);

                        Some(new_state)
                    }
                    (CardLocation::PlaceHolder(_hole), _) => {
                        let mut new_state = self.clone();
                        let Card::Beast(beast) = new_state.pop(&beast_location).unwrap() else {
                            unreachable!("should be a beast");
                        };
                        let pirate1 = new_state.pop(&pirate1).unwrap();
                        assert_eq!(pirate1, Card::Pirate);
                        let pirate2 = new_state.pop(&pirate2).unwrap();
                        assert_eq!(pirate2, Card::Pirate);

                        let mut bundle_place = new_state.placeholders.holes.take(&PlaceHolder::default()).unwrap();
                        bundle_place.0 = Some(CardOrBundle::BeastBundle(beast));
                        new_state.placeholders.holes.insert(bundle_place);

                        Some(new_state)
                    }
                    _ => None,
                }
            });

        card_output_states
            .chain(card_split_states)
            .chain(card_hold_states)
            .chain(beast_promotion_states)
    }
}
