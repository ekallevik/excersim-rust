use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let word = word.to_lowercase();
    let word_content = count_frequencies(&word);
    let mut result = HashSet::new();

    for &candidate in possible_anagrams {
        let c = candidate.to_lowercase();

        if word != c && word_content == count_frequencies(&c) {
            result.insert(candidate);
        };
    };

    result
}

pub fn count_frequencies(word: &String) -> HashMap<char, u8> {
    word
        .chars()
        .fold(
        HashMap::new(),
        |mut acc, curr| {
            let char_count = acc.entry(curr).or_insert(0);
            *char_count += 1;
            acc
        },
    )
}
