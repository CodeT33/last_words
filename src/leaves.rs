use std::collections::VecDeque;
use crate::leaf_pool::LeafPool;

#[derive(Debug, Default)]
pub(crate) struct TypingData {
    number_typing_errors: u32,
    wpm: u32,
}

#[derive(Default, Debug)]
pub(crate) struct LeafSettings {
    //Maybe important later
}

#[derive(Debug)]
pub(crate) struct Leaf {
    pub(crate) content: String,
    pub(crate) settings: LeafSettings,
    pub(crate) typing_data: TypingData,
}

impl Leaf {
    pub(crate) fn new(content: String) -> Self {
        Self {
            content,
            settings: LeafSettings::default(),
            typing_data: TypingData::default(),
        }
    }

    fn get_value(&self) -> u32 {
        self.content.chars().count() as u32
    }
}

#[derive(Default, Debug)]
pub(crate) struct StackOfLeaves {
    leaves: VecDeque<Leaf>,
}

impl StackOfLeaves {
    pub(crate) fn new(pool: &LeafPool, height: u32) -> Self {
        let mut stack = VecDeque::new();

        for _ in 0..height {
            let Some(leaf) = pool.get_random() else {
                break;
            };
            stack.push_back(leaf);
        }
        Self { leaves: stack }
    }

    pub(crate) fn take_first(&mut self) -> Option<Leaf> {
        self.leaves.pop_front()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.leaves.len()
    }

}

