use crate::leaf_pool::LeafPool;
use crate::leaves::{Leaf, StackOfLeaves};
use rand::RngExt;

#[derive(Debug)]
pub(crate) struct Desktop {
    pub(crate) stacks: Vec<StackOfLeaves>,
    pub(crate) finished_leaves: Vec<Leaf>,
}

impl Desktop {
    pub(crate) fn new(pool: &LeafPool, number_of_stacks: u32, max_stack_height: u32) -> Self {
        let mut stacks = Vec::with_capacity(number_of_stacks as usize);
        for _ in 0..number_of_stacks {
            stacks.push(StackOfLeaves::new(
                pool,
                rand::rng().random_range(1..max_stack_height),
            ));
        }

        let finished_leaves = vec![];
        Self {
            stacks,
            finished_leaves,
        }
    }

    pub(crate) fn get_total_points(&self) -> i32 {
        let mut total_points: f32 = 0.0;
        total_points += self.get_saved_chars() as f32;
        total_points -= self.get_typing_errors() as f32 * 5.0;
        total_points *= 0.2 * self.get_transcribed_leaves() as f32 + 1.0;
        total_points *= 0.5 * self.get_perfect_leaves() as f32 + 1.0;
        total_points as i32
    }

    pub(crate) fn get_saved_chars(&self) -> usize {
        self.finished_leaves
            .iter()
            .map(|leaf| leaf.content.len() - leaf.typing_data.number_typing_errors as usize)
            .sum()
    }

    pub(crate) fn get_typing_errors(&self) -> usize {
        self.finished_leaves
            .iter()
            .map(|leaf| leaf.typing_data.number_typing_errors as usize)
            .sum()
    }

    pub(crate) fn get_perfect_leaves(&self) -> usize {
        self.finished_leaves
            .iter()
            .map(|leaf| {
                if leaf.typing_data.number_typing_errors == 0 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    pub(crate) fn get_transcribed_leaves(&self) -> usize {
        self.finished_leaves.len()
    }

    pub(crate) fn get_transcribed_chars(&self) -> usize {
        self.finished_leaves
            .iter()
            .map(|leaf| leaf.content.len())
            .sum()
    }

    pub(crate) fn get_correct_transcribing_in_percent(&self) -> usize {
        let total = self.get_transcribed_chars();
        let saved = self.get_saved_chars();
        if total == 0 {
            return 0;
        }
        (saved as f32 / total as f32 * 100.0) as usize
    }

    pub(crate) fn get_average_wpm(&self) -> usize {
        if self.finished_leaves.is_empty() {
            return 0;
        }
        self.finished_leaves
            .iter()
            .map(|leaf| leaf.typing_data.wpm as usize)
            .sum::<usize>()
            / self.finished_leaves.len()
    }
}
