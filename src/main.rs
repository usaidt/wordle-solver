use wordle_solver::WordData;

fn main() -> Result<(), std::io::Error> {

    let _word_file = "word_list.txt";
    let _cache_file = "cache.bin";

    // let mut word_data = WordData::new(_word_file, "words")?;

    let mut word_data = WordData::new(_cache_file, "cache")?;

    let results = word_data
        .contains("apple", true)
        .doesnt_contain("by", true)
        .doesnt_contain("ug", true)
        .at_position(0, "a", true)
        .not_at_position(1, "p", true)
        .results();

    println!("\nFiltered words: {:?}", results);

    Ok(())
}