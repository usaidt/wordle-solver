use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WordData {
    pub word_list: Vec<String>,
    pub bitmask_array: Vec<u32>,
    pub position_index: [HashMap<char, Vec<usize>>; 5],
    pub presence_index: HashMap<char, Vec<usize>>,
    pub possible_word_ids: Vec<usize>
}

impl WordData {
    pub fn new(word_list: Vec<String>) -> Self {
        let bitmask_array = Self::build_bitmask_array(&word_list);
        let position_index = Self::build_position_index(&word_list);
        let presence_index = Self::build_presence_index(&bitmask_array);
        let possible_word_ids = (0..word_list.len()).collect();

        WordData {
            word_list,
            bitmask_array,
            position_index,
            presence_index,
            possible_word_ids
        }
    }

    fn build_bitmask_array(word_list: &[String]) -> Vec<u32> {
        word_list.iter().map(|word| Self::calculate_bitmask(word)).collect()
    }

    fn build_position_index(word_list: &[String]) -> [HashMap<char, Vec<usize>>; 5] {
        let mut position_index: [HashMap<char, Vec<usize>>; 5] = Default::default();
        for (word_id, word) in word_list.iter().enumerate() {
            for (i, c) in word.chars().enumerate() {
                position_index[i].entry(c).or_insert_with(Vec::new).push(word_id);
            }
        }
        position_index
    }

    fn build_presence_index(bitmask_array: &[u32]) -> HashMap<char, Vec<usize>> {
        let mut presence_index: HashMap<char, Vec<usize>> = HashMap::new();
        for (word_id, &bitmask) in bitmask_array.iter().enumerate() {
            for c in 'a'..='z' {
                if bitmask & (1 << (c as u32 - 'a' as u32)) != 0 {
                    presence_index.entry(c).or_insert_with(Vec::new).push(word_id);
                }
            }
        }
        presence_index
    }

    fn calculate_bitmask(word: &str) -> u32 {
        word.chars()
            .map(|c| 1 << (c as u32 - 'a' as u32))
            .fold(0, |acc, mask| acc | mask)
    }

    fn containing_letter(&self, letter: char) -> Vec<usize> {
        self.presence_index
            .get(&letter)
            .unwrap_or(&Vec::new())
            .iter()
            .cloned()
            .collect()
    }

    fn containing_letter_at(&self, letter: char, position: usize) -> Vec<usize> {
        self.position_index[position]
            .get(&letter)
            .unwrap_or(&Vec::new())
            .iter()
            .cloned()
            .collect()
    }

    pub fn contains(&mut self, letters: &str) -> &mut Self {
        for letter in letters.chars() {
            let filtered_ids = self.containing_letter(letter);
            self.possible_word_ids.retain(|id| filtered_ids.contains(id));
        }
        println!("Filtered IDs (contains '{}'): {:?}", letters, self.possible_word_ids);
        self
    }

    pub fn doesnt_contain(&mut self, letters: &str) -> &mut Self {
        for letter in letters.chars() {
            let filtered_ids = self.containing_letter(letter);
            self.possible_word_ids.retain(|id| !filtered_ids.contains(id));
        }
        println!(
            "Filtered IDs (doesn't contain '{}'): {:?}",
            letters, self.possible_word_ids
        );
        self
    }

    pub fn at_position(&mut self, position: usize, letter: char) -> &mut Self {
        let filtered_ids = self.containing_letter_at(letter, position);
        self.possible_word_ids.retain(|id| filtered_ids.contains(id));
        println!(
            "Filtered IDs (at position {} with '{}'): {:?}",
            position, letter, self.possible_word_ids
        );
        self
    }

    pub fn not_at_position(&mut self, position: usize, letter: char) -> &mut Self {
        let filtered_ids = self.containing_letter_at(letter, position);
        self.possible_word_ids.retain(|id| !filtered_ids.contains(id));
        println!(
            "Filtered IDs (not at position {} with '{}'): {:?}",
            position, letter, self.possible_word_ids
        );
        self
    }

    pub fn results(&self) -> Vec<String> {
        self.possible_word_ids
            .iter()
            .map(|&id| self.word_list[id].clone())
            .collect()
    }
}