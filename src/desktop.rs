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
}
