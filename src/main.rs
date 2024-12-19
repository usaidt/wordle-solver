mod guesser;

use guesser::Guesser;

fn main() {
    let words = vec![
        "apple".to_string(),
        "zoola".to_string(),
        "bytea".to_string(),
        "zycxu".to_string(),
        "jklqa".to_string(),
    ];

    let mut guesser = Guesser::new(words);
    guesser.print_debug_info();
    guesser.apply_filters();
}
