use ahash::AHashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt;
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::Path;


/// A vector of WordEntries comprising a sentence or phrase.
pub struct Sentence {
    words: Vec<WordEntry>,
}
impl Sentence {
    /// Check each combinations of stresses for each word in the sentence and return True if
    /// one is found matching trochaic tetrameter that is also exactly 8 syllables.
    /// E.g. you can sing it to the TMNT themesong.
    fn turtle_trochaic_tetrameter(&self) -> bool {

        false
    }

    fn can_be_eight_syllables(&self) -> bool {
        let mut can_be_eight = false;

        for word in self.words.iter() {
            for word_len in word.get_syllable_lengths() {

            }
        }

        let mut word_queue = self.words.clone();
        while !word_queue.is_empty() {
            let word = word_queue.pop();


        }

        can_be_eight
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A single Word with all its known Pronunciations.
pub struct WordEntry {
    word: Word,
    pronunciations: Vec<Pronunciation>,
}
impl WordEntry {
    // TODO: figure out how to make these functions into properties that are lazily evaluated only once
    fn get_syllables(&self) -> &Vec<Vec<Syllable>> {
        self.pronunciations.iter()
            .map(|p| p.syllables)
            .collect()
    }
    fn get_syllable_lengths(&self) -> &Vec<u8> {
        self.pronunciations.iter()
            .map(|p| p.len())
            .collect()
    }
    fn get_unique_syllable_lengths(&self) -> &Vec<u8> {
        self.pronunciations.iter()
            .map(|p| p.len())
            .collect()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A single word with its string representations and one possible pronunciation.
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
    fn len(&self) -> u8 {
        self.pronunciation.len() as u8
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Syllables that represent how a word is pronounced.
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
    fn len(&self) -> u8 {
        self.syllables.len() as u8
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
