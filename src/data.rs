use crate::error::Error;
use indexmap::IndexMap;
use rand::Rng;
use serde::Deserialize;
use std::ops::Index;

#[derive(Debug, Deserialize)]
pub struct Word {
    value: String,
    quote: String,
    #[serde(default)]
    keywords: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Data {
    words: IndexMap<String, Word>,
    keywords: IndexMap<String, Vec<String>>,
}

impl<const N: usize> From<[Word; N]> for Data {
    fn from(words: [Word; N]) -> Self {
        let mut data: Data = Default::default();

        for word in words {
            let _ = data.add(word);
        }

        data
    }
}

impl Data {
    pub fn from_path(path: &'static str) -> Self {
        let mut data: Data = Default::default();

        if let Ok(mut rdr) = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
        {
            let mut record = csv::StringRecord::new();

            while rdr.read_record(&mut record).is_ok() && !record.is_empty() {
                let word: Word = record.deserialize(None).unwrap();
                let _ = data.add(word);
            }
        }

        data
    }

    pub fn add(&mut self, word: Word) -> Result<(), Error> {
        if self.words.contains_key(&word.value) {
            return Err(Error::WordAlreadyExists);
        }

        for keyword in &word.keywords {
            self.keywords
                .entry(keyword.clone())
                .or_default()
                .push(word.value.clone());
        }

        self.words.insert(word.value.clone(), word);

        Ok(())
    }

    pub fn get_random_word(&self) -> Result<&Word, Error> {
        if self.words.is_empty() {
            return Err(Error::ThereIsNoWord);
        }

        let mut rng = rand::thread_rng();

        Ok(self.words.index(rng.gen_range(0..self.words.len())))
    }
}
