pub fn exercise_2 (text: &str) {
    let words: Vec<&str> = text.split(" ").collect();

    let mut text_converted: String = String::new();

    for word in words {
        let word_to_concatenate: String = convert_word(word);
    }
}

fn convert_word (word: &str) -> String {
    let letters: Vec<char> = word.chars().collect();

    // ### TODO - Check what rule to apply ###
}

fn convert_with_rule_1 (characters: Vec<char>) -> String {
    let mut word_compiled: Vec<char> = characters[1..].to_vec();

    word_compiled.push(characters[0]);
    word_compiled.push('a' as char);
    word_compiled.push('y' as char);

    return word_compiled.iter().collect();
}

fn convert_with_rule_2 (characters: Vec<char>) -> String {
    
}

fn convert_with_rule_3 (characters: Vec<char>) -> String {
    
}

fn convert_with_rule_4 (characters: Vec<char>) -> String {
    
}