use ahash::AHashMap;
use rayon::prelude::*;
use std::fmt;
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::Path;


pub struct Word {
    word: String,
    pronunciation: Pronunciation,
}
impl Word {
    fn from_cmu_entry(line: &str) -> Word {
        let (word_str, pronunciation_str) = line.split_once("  ")
            .expect(format!("Failed to split line {line}").as_str());
        // Word ends with a number e.g. (2), representing an additional pronunciation. Strip it off.
        let mut word = word_str.to_string();
        if word.ends_with(")") {
            // There are only single-digit quantities of additional pronunciations in CMU dict, so
            // we can simply truncate the last three characters.
            word.truncate(word.len() - 3)
        }
        Word { word, pronunciation: Pronunciation::from_str(pronunciation_str) }
    }
}

pub struct Pronunciation {
    syllables: Vec<Syllable>
    // Implementation note:
    // For this bot, we only care about stresses, and not that actual pronunciation of any phoneme.
}
impl Pronunciation {
    fn from_str(pronunciation_str: &str) -> Pronunciation {
        let syllables: Vec<Syllable> = pronunciation_str
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|digit| match digit {
                '0' => Syllable::Unstressed,
                '1' => Syllable::PrimaryStress,
                '2' => Syllable::SecondaryStress,
                _   => panic!("Panic! Pronunciation contained unexpected stress digit: {digit}")
            })
            .collect();
        Pronunciation { syllables }
    }
}
impl fmt::Display for Pronunciation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_buf = String::with_capacity(self.syllables.len());
        for syllable in &self.syllables {
            match syllable {
                Syllable::Unstressed => str_buf.push('0'),
                Syllable::PrimaryStress => str_buf.push('1'),
                Syllable::SecondaryStress => str_buf.push('2'),
            }
        }
        write!(f, "{str_buf}")
    }
}

pub enum Syllable {
    PrimaryStress,
    SecondaryStress,
    Unstressed,
}
impl Syllable {
    fn from_char(c: char) -> Syllable {
        match c {
            '0' => Syllable::Unstressed,
            '1' => Syllable::PrimaryStress,
            '2' => Syllable::SecondaryStress,
            _   => panic!("Panic! Pronunciation contained unexpected stress digit: {c}")
        }
    }
}

/// Create a hashmap of words to pronunciations from the CMU pronouncing dictionary.
pub fn cmu_dict_file_to_map(file: &Path) -> AHashMap<String, Vec<Pronunciation>>
{
    // Implementation note:
    //   The whole cmu dict is very small compared to the available memory on the machine I plan to
    //   run this. Given that, I'm just stuffing everything into a big hashmap instead of something
    //   more compact like a specialized automata or ART / radix trie.
    //
    // TODO:
    //   - Allow selection between std hasher, ahash, and FxHash via flag/config & feature toggles
    //   - Serialize hashset to avoid have to re-parsing every time.
    //   - Discard words with > 8 syllables (although I think this is very few?)
    //   - Use Results instead of just panicking whenever something goes wrong.

    read_to_string(file)
        .expect("Panic! Failed to read CMU dict from file")
        .lines()
        // Filter pronunciations of punctuation found in the CMU dict. In sentences containing
        // punctuation marks, we would seldom pronounce the punctuation mark as its name.
        .filter(|line| !line.starts_with(";"))   // | "!" | "\"" | "#" | "%" | "&" | ")" | "(" | "+" | "," | "-" | "." | "/" | ":" | "?" | "{" | "}"
        .map(|line| Word::from_cmu_entry(line))
        .fold(AHashMap::new(), |mut map, word| {
            map.entry(word.word)
                .or_insert(Vec::new())
                .push(word.pronunciation);
            map
        })
}
