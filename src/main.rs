use wordle_solver::WordData;

fn main() {
    let words = vec![
        "apple".to_string(),
        "zoola".to_string(),
        "bytea".to_string(),
        "zycxu".to_string(),
        "jklqa".to_string(),
    ];

    let mut word_data = WordData::new(words);

    println!("Word List:");
    for (id, word) in word_data.word_list.iter().enumerate() {
        println!("ID: {}, Word: {}", id, word);
    }

    println!("\nBitmask Array:");
    for (id, &bitmask) in word_data.bitmask_array.iter().enumerate() {
        println!("ID: {}, Bitmask: {:026b}", id, bitmask);
    }

    println!("\nPresence Index:");
    for (letter, word_ids) in &word_data.presence_index {
        let word_list: Vec<_> = word_ids.iter().map(|&id| &word_data.word_list[id]).collect();
        println!(
            "Letter: {}, Word IDs: {:?}, Words: {:?}",
            letter, word_ids, word_list
        );
    }

    println!("\nPosition Index:");
    for (pos, index) in word_data.position_index.iter().enumerate() {
        println!("Position {}:", pos);
        for (letter, word_ids) in index {
            let word_list: Vec<_> = word_ids.iter().map(|&id| &word_data.word_list[id]).collect();
            println!(
                "  Letter: {}, Word IDs: {:?}, Words: {:?}",
                letter, word_ids, word_list
            );
        }
    }

    let results = word_data
        .contains("apple")
        .doesnt_contain("by")
        .doesnt_contain("y")
        .at_position(0, 'z')
        .not_at_position(1, 'p')
        .results();

    println!("Filtered words: {:?}", results);
}