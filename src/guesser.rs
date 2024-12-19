use wordle_solver::WordData;

pub struct Guesser {
    pub word_data: WordData,
}

impl Guesser {
    pub fn new(words: Vec<String>) -> Self {
        let word_data = WordData::new(words);
        Guesser { word_data }
    }

    pub fn print_debug_info(&self) {
        println!("Word List:");
        for (id, word) in self.word_data.word_list.iter().enumerate() {
            println!("ID: {}, Word: {}", id, word);
        }

        println!("\nBitmask Array:");
        for (id, &bitmask) in self.word_data.bitmask_array.iter().enumerate() {
            println!("ID: {}, Bitmask: {:026b}", id, bitmask);
        }

        println!("\nPresence Index:");
        for (letter, word_ids) in &self.word_data.presence_index {
            let word_list: Vec<_> = word_ids
                .iter()
                .map(|&id| &self.word_data.word_list[id])
                .collect();
            println!(
                "Letter: {}, Word IDs: {:?}, Words: {:?}",
                letter, word_ids, word_list
            );
        }

        println!("\nPosition Index:");
        for (pos, index) in self.word_data.position_index.iter().enumerate() {
            println!("Position {}:", pos);
            for (letter, word_ids) in index {
                let word_list: Vec<_> = word_ids
                    .iter()
                    .map(|&id| &self.word_data.word_list[id])
                    .collect();
                println!(
                    "  Letter: {}, Word IDs: {:?}, Words: {:?}",
                    letter, word_ids, word_list
                );
            }
        }
    }

    pub fn apply_filters(&mut self) {
        let results = self
            .word_data
            .contains("apple")
            .doesnt_contain("by")
            .doesnt_contain("y")
            .at_position(0, 'a')
            .not_at_position(1, 'l')
            .results();

        println!("Filtered words: {:?}", results);
    }
}
