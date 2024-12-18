use wordle_solver::WordData;
use wordle_solver::WordQuery;

fn main() {
    let words = vec![
        "apple".to_string(),
        "zoola".to_string(),
        "bytea".to_string(),
        "zycxu".to_string(),
        "jklqa".to_string(),
    ];

    let word_data = WordData::new(words);

    println!("Word List:");
    for (id, word) in word_data.word_list.iter().enumerate() {
        println!("ID: {}, Word: {}", id, word);
    }

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

    // let mut find_letter = 'a';
    // println!("\n Words containing '{}':", find_letter);
    // let words_with_letter = word_data.containing_letter(find_letter);
    // println!("Words: {:?}", words_with_letter);

    // let at_position = 1;
    // find_letter = 'p';
    // println!(
    //     "\nWords with '{}' in position {}:",
    //     find_letter, at_position
    // );

    // let words_at_position = word_data.containing_letter_at(find_letter, at_position);
    // println!("Words: {:?}", words_at_position);

    let results = WordQuery::new(&word_data)
        .contains('a')
        .doesnt_contain('b')
        .doesnt_contain('y')
        .at_position(0, 'z')
        .not_at_position(1, 'p')
        .results();

    println!("Results: {:?}", results);
}
