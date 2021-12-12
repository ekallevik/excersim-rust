// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_content = count_frequencies(magazine);
    let note_content = count_frequencies(note);

   for (word, count) in note_content {
       let magazine_count = match magazine_content.get(word) {
           None => 0,
           Some(&value) => value
       };
       if magazine_count < count {
           return false
       }
   }
    true
}

pub fn count_frequencies<'a>(words: &'a[&str]) -> HashMap<&'a str, i32> {
    words.iter().fold(
        HashMap::new(),
        |mut acc, &curr| {
            let word_count = acc.entry(curr).or_insert(0);
            *word_count += 1;
            acc
        }
    )
}
