pub fn exercise_2 (text: &str) {
    let words: Vec<&str> = text.split(" ").collect();

    let mut text_converted: String = String::new();

    for word in words {
        let word_to_concatenate: String = convert_word(word);
    }
}

fn convert_word (word: &str) -> String {
    let initial_letters: Vec<char> = &word[0..2].chars();

    // ### TODO - Check what rule to apply ###
}

fn convert_with_rule_1 (word: &str) -> &str {

}

fn convert_with_rule_2 (word: &str) -> &str {
    
}

fn convert_with_rule_3 (word: &str) -> &str {
    
}

fn convert_with_rule_4 (word: &str) -> &str {
    
}