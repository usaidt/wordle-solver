use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WordData {
    pub word_list: Vec<String>,
    pub bitmask_index: HashMap<u32, Vec<String>>,
    pub position_index: [HashMap<char, Vec<String>>; 5],
}

impl WordData {
    pub fn new(word_list: Vec<String>) -> Self {
        let bitmask_index = Self::build_bitmask_index(&word_list);
        let position_index = Self::build_position_index(&word_list);
        WordData {
            word_list,
            bitmask_index,
            position_index,
        }
    }

    fn build_bitmask_index(word_list: &[String]) -> HashMap<u32, Vec<String>> {
        let mut index = HashMap::new();
        for word in word_list {
            let mask = Self::calculate_bitmask(word);
            index.entry(mask).or_insert_with(Vec::new).push(word.clone());
        }
        index
    }

    fn build_position_index(word_list: &[String]) -> [HashMap<char, Vec<String>>; 5] {
        let mut position_index: [HashMap<char, Vec<String>>; 5] = Default::default();
        for word in word_list {
            for (i, c) in word.chars().enumerate() {
                position_index[i].entry(c).or_insert_with(Vec::new).push(word.clone());
            }
        }
        position_index
    }

    fn calculate_bitmask(word: &str) -> u32 {
        word.chars()
            .map(|c| 1 << (c as u32 - 'a' as u32))
            .fold(0, |acc, mask| acc | mask)
    }
}
