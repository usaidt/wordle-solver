use wordle_solver::WordData;

fn main() -> Result<(), std::io::Error> {

    let _word_file = "word_list.txt";
    let _cache_file = "cache.bin";

    // let mut word_data = WordData::new(_word_file, "words")?;

    let mut word_data = WordData::new(_cache_file, "cache")?;

    // println!("Word List:");
    // for (id, word) in word_data.word_list.iter().enumerate() {
    //     println!("ID: {}, Word: {}", id, word);
    // }

    // println!("\nBitmask Array:");
    // for (id, &bitmask) in word_data.bitmask_array.iter().enumerate() {
    //     println!("ID: {}, Bitmask: {:026b}", id, bitmask);
    // }

    // println!("\nPresence Index:");
    // for (letter, word_ids) in &word_data.presence_index {
    //     let word_list: Vec<_> = word_ids.iter().map(|&id| &word_data.word_list[id]).collect();
    //     println!(
    //         "Letter: {}, Word IDs: {:?}, Words: {:?}",
    //         letter, word_ids, word_list
    //     );
    // }

    // println!("\nPosition Index:");
    // for (pos, index) in word_data.position_index.iter().enumerate() {
    //     println!("Position {}:", pos);
    //     for (letter, word_ids) in index {
    //         let word_list: Vec<_> = word_ids.iter().map(|&id| &word_data.word_list[id]).collect();
    //         println!(
    //             "  Letter: {}, Word IDs: {:?}, Words: {:?}",
    //             letter, word_ids, word_list
    //         );
    //     }
    // }

    // println!("Word at id 488: {}", word_data.word_list[488]);
    // println!("Word at id 314: {}", word_data.word_list[314]);
    // println!("Word at id 705: {}", word_data.word_list[705]);

    let results = word_data
        .contains("apple")
        .doesnt_contain("by")
        .doesnt_contain("ug")
        .at_position(0, 'a')
        .not_at_position(1, 'p')
        .results();

    println!("Filtered words: {:?}", results);

    Ok(())
}