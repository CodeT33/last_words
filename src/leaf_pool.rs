use crate::consts::MAX_WORDS_PER_LEAF;
use crate::leaves::Leaf;
use rand::prelude::IndexedRandom;
use std::fs;
use std::io::BufRead;

#[derive(Debug)]
pub(crate) struct LeafPool {
    pub leaves: Vec<String>,
}

impl LeafPool {
    pub(crate) fn from_path(path: &str) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut pool = Self { leaves: Vec::new() };

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with('#') {
                continue;
            }

            let leaf_content = Self::truncate_line(line, MAX_WORDS_PER_LEAF);
            pool.add(&leaf_content);
        }
        Ok(pool)
    }

    pub(crate) fn add_from_path(&mut self, path: &str) {
        let content = fs::read_to_string(path);
        for line in content.expect("REASON").lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }
            if line.starts_with('#') {
                continue;
            }

            let leaf_content = Self::truncate_line(line, MAX_WORDS_PER_LEAF);
            self.add(&leaf_content);
        }
    }

    fn truncate_line(line: &str, max_words: usize) -> String {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() <= max_words {
            return line.to_owned();
        }

        format!("{}...", words[..max_words].join(" "))
    }

    pub(crate) fn get_random(&self) -> Option<Leaf> {
        let content = self.leaves.choose(&mut rand::rng())?;

        let default = "default".to_string();
        Some(Leaf::new(default, content.clone()))
    }

    fn add(&mut self, content: &str) {
        self.leaves.push(content.to_owned());
    }
}
