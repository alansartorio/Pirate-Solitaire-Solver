use crate::state::{self, CardStack, Output, PlaceHolder, State};

pub struct DenormalizationInformation {
    initial_stacks: Vec<CardStack>,
}

pub struct DisplayState {
    state: State,
    denormalization_information: DenormalizationInformation,
}

#[derive(Debug, Clone, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct PlaceHolders {
    pub holes: Vec<PlaceHolder>,
}

impl Default for PlaceHolders {
    fn default() -> Self {
        Self {
            holes: Vec::from_iter([PlaceHolder::default(); 3]),
        }
    }
}

pub struct DenormalizedState {
    pub placeholders: PlaceHolders,
    pub output: Output,
    pub board: Vec<CardStack>,
}

impl State {
    pub fn denormalize(
        self,
        denormalization_information: &DenormalizationInformation,
    ) -> DenormalizedState {
        let mut board = vec![];
        let mut available_stacks = self.board;
        for initial_stack in &denormalization_information.initial_stacks {
            let matching_stack = available_stacks
                .iter()
                .max_by_key(|stack| {
                    initial_stack
                        .cards
                        .iter()
                        .zip(stack.cards.iter())
                        .take_while(|(initial, current)| initial == current)
                        .count()
                })
                .expect("should never be empty")
                .clone();

            let matching_stack = available_stacks.remove(&matching_stack).unwrap();
            board.push(matching_stack);
        }

        DenormalizedState {
            placeholders: PlaceHolders {
                holes: self.placeholders.holes.into_iter().collect(),
            },
            output: self.output,
            board,
        }
    }
}

impl DenormalizedState {
    pub fn normalize(self) -> (State, DenormalizationInformation) {
        let information = DenormalizationInformation {
            initial_stacks: self.board.clone(),
        };

        let state = State {
            board: self.board.into_iter().collect(),
            output: self.output,
            placeholders: state::PlaceHolders {
                holes: self.placeholders.holes.into_iter().collect(),
            },
        };

        (state, information)
    }
}
