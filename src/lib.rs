use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead};

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedWordData {
    pub word_list: Vec<String>,
    pub bitmask_array: Vec<u32>,
    pub position_index: [HashMap<char, Vec<usize>>; 5],
    pub presence_index: HashMap<char, Vec<usize>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordData {
    pub word_list: Vec<String>,
    pub bitmask_array: Vec<u32>,
    pub position_index: [HashMap<char, Vec<usize>>; 5],
    pub presence_index: HashMap<char, Vec<usize>>,
    pub possible_word_ids: Vec<usize>
}

impl WordData {
    
    pub fn new(file_path: &str, mode: &str) -> Result<Self, std::io::Error> {
        let mut word_data = WordData {
            word_list: Vec::new(),
            bitmask_array: Vec::new(),
            position_index: Default::default(),
            presence_index: HashMap::new(),
            possible_word_ids: Vec::new(),
        };

        match mode {
            "cache" => {
                if !word_data.load_from_cache(file_path) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Cache file not found or invalid.",
                    ));
                }
            }
            "words" => {
                let word_list = Self::create_word_list(file_path)?;
                word_data = WordData::initialize(word_list);
                word_data.save_to_cache("cache.bin");
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid mode. Use 'cache' or 'words'.",
                ));
            }
        }

        Ok(word_data)
    }

    pub fn create_word_list(file_path: &str) -> Result<Vec<String>, std::io::Error> {
        println!("Opening file: {}", file_path);
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let word_list: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        Ok(word_list)
    }

    fn initialize(word_list: Vec<String>) -> Self {
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

    pub fn load_from_cache(&mut self, cache_file: &str) -> bool {
        if let Ok(file) = File::open(cache_file) {
            let reader = BufReader::new(file);
            if let Ok(cached_data) = bincode::deserialize_from::<_, CachedWordData>(reader) {
                self.word_list = cached_data.word_list;
                self.bitmask_array = cached_data.bitmask_array;
                self.position_index = cached_data.position_index;
                self.presence_index = cached_data.presence_index;
                self.possible_word_ids = (0..self.word_list.len()).collect();
                println!("Loaded from cache successfully.");
                return true;
            } else {
                println!("Failed to deserialize cache.");
            }
        }
        println!("Cache file not found or invalid.");
        false
    }

    pub fn save_to_cache(&self, cache_file: &str) {
        if let Ok(file) = File::create(cache_file) {
            let writer = BufWriter::new(file);
            let cached_data = CachedWordData {
                word_list: self.word_list.clone(),
                bitmask_array: self.bitmask_array.clone(),
                position_index: self.position_index.clone(),
                presence_index: self.presence_index.clone(),
            };
            if bincode::serialize_into(writer, &cached_data).is_ok() {
                println!("WordData saved to cache.");
            } else {
                println!("Failed to save cache.");
            }
        }
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