use crate::leaf_pool::LeafPool;
use crate::leaves::{Leaf, StackOfLeaves};

#[derive(Debug)]
pub(crate) struct Desktop {
    pub(crate) stacks: Vec<StackOfLeaves>,
    pub(crate) finished_leaves: Vec<Leaf>,
}

impl Desktop {
    pub(crate) fn new(pool: &LeafPool, number_of_stacks: u32, stack_height: u32) -> Self {
        let mut stacks = Vec::with_capacity(number_of_stacks as usize);
        for _ in 0..number_of_stacks {
            stacks.push(StackOfLeaves::new(pool, stack_height));
        }

        let finished_leaves = vec![];
        Self {
            stacks,
            finished_leaves,
        }
    }
    
    pub(crate) fn get_saved_fragments(&self) -> usize {
        self.finished_leaves.iter().map(|leaf| leaf.content.len()-leaf.typing_data.number_typing_errors as usize).sum()
    }
    
    pub(crate) fn get_typing_errors(&self) -> usize {
        self.finished_leaves.iter().map(|leaf| leaf.typing_data.number_typing_errors as usize).sum()
    }
    
    pub(crate) fn get_perfect_leaves(&self) -> usize {
        self.finished_leaves.iter().map(|leaf| if leaf.typing_data.number_typing_errors == 0 { 1 } else { 0 }).sum()
    }
    
    pub(crate) fn get_transcribed_leaves(&self) -> usize {
        self.finished_leaves.len()
    }
}
