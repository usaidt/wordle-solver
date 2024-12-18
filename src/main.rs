use wordle_solver::WordData;

fn main() {
    let word_list = vec![
        "zoola".to_string(),
        "boola".to_string(),
        "apple".to_string(),
        "zyxab".to_string(),
        "twrwo".to_string(),
    ];

    let word_data = WordData::new(word_list);

    println!("Word List: {:?}", word_data.word_list);
    println!("zyxwvutsrqponmlkjihgfedcba");
    println!("Bitmask Index:");
    for (key, value) in &word_data.bitmask_index {
        println!("{:026b}: {:?}", key, value);
    }
    println!("Position Index: {:?}", word_data.position_index);
}
