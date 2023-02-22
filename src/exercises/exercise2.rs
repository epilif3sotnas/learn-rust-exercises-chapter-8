const VOWELS: &str = "aeiou";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

pub fn exercise_2(text: &str) {
    println!("############## Exercise 2 ##############");
    let words: Vec<&str> = text.split(" ").collect();

    let mut text_converted: String = String::new();

    for word in words {
        let word_to_concatenate: String = convert_word(word);

        text_converted.push_str(&word_to_concatenate);
        text_converted.push_str(" ");
    }

    println!("Text converted (Pig Latin): {:?}", text_converted);
    println!("########################################");
}

fn convert_word(word: &str) -> String {
    let characters: Vec<char> = word.chars().collect();

    if CONSONANTS.contains(characters[0]) && VOWELS.contains(characters[1]) {
        return convert_with_rule_1(characters);
    } else if CONSONANTS.contains(characters[0]) {
        return convert_with_rule_2(characters);
    } else if VOWELS.contains(characters[0]) {
        return convert_with_rule_3(characters);
    } else {
        return characters.iter().collect();
    }
}

// When started with one consonant
fn convert_with_rule_1(characters: Vec<char>) -> String {
    let mut word_compiled: Vec<char> = characters[1..].to_vec();

    word_compiled.push(characters[0]);
    word_compiled.push('a' as char);
    word_compiled.push('y' as char);

    return word_compiled.iter().collect();
}

// When started with two consonants
fn convert_with_rule_2(characters: Vec<char>) -> String {
    let mut word_compiled: Vec<char> = characters[2..].to_vec();

    word_compiled.push(characters[0]);
    word_compiled.push(characters[1]);
    word_compiled.push('a' as char);
    word_compiled.push('y' as char);

    return word_compiled.iter().collect();
}

// When started with vowel
fn convert_with_rule_3(characters: Vec<char>) -> String {
    let mut word_compiled: Vec<char> = characters[0..].to_vec();

    word_compiled.push('w' as char);
    word_compiled.push('a' as char);
    word_compiled.push('y' as char);

    return word_compiled.iter().collect();
}
