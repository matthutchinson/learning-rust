// Convert strings to pig latin. The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_one(word: &str) -> String {
    // grab mutable iterator for word chars
    let mut chars = word.chars();
    // grab first char, handle None, since next() could return nothing from iter
    let first_ch = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };
    // check if word starts with a vowel or not
    // chars.as_str() returns rest of iterable chars to a string (i.e. not including the first ch)
    match first_ch.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_ch),
    }
}

fn pig_latin(sentence: &str) -> String {
    let mut pig_words = Vec::new();

    // split sentence into words
    for word in sentence.split_whitespace() {
        // push new word into string (also contains space)
        pig_words.push(pig_one(&word));
    }

    // return new sentence, by joining words
    pig_words.join(" ")
}

// a map and fold approach, build up string adding a space after each current word
// https://codereview.stackexchange.com/questions/175906/convert-string-to-pig-latin-in-rust
fn folder(mut current: String, next: String) -> String {
    if !current.is_empty() {
        current.push(' ');
    }
    current.push_str(&next);
    current
}

fn pig_latin_map_n_fold(sentence: &str) -> String {
    sentence.split_whitespace()
        .map(pig_one)
        .fold(String::new(), folder)
}

fn main() {
    // these solutions will not work with punctuation characters in input sentence
    let sentence = String::from("First the Apple does not fall far from the tree sir");

    println!("{}", sentence);
    println!("--");
    println!("{}", pig_latin(&sentence));
    println!("--");
    println!("{}", pig_latin_map_n_fold(&sentence));
}
