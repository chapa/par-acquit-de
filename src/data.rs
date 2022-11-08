use crate::error::Error;
use indexmap::IndexMap;
use rand::Rng;
use std::ops::Index;

#[derive(Debug)]
pub struct Word<'a> {
    value: &'a str,
    quote: &'a str,
    keywords: Vec<&'a str>,
}

impl<'a> Word<'a> {
    pub fn create(value: &'a str, quote: &'a str, keywords: Vec<&'a str>) -> Self {
        Word {
            value,
            quote,
            keywords,
        }
    }
}

#[derive(Debug)]
pub struct Data<'a> {
    words: IndexMap<&'a str, Word<'a>>,
    keywords: IndexMap<&'a str, Vec<&'a str>>,
}

impl<'a, const N: usize> From<[Word<'a>; N]> for Data<'a> {
    fn from(words: [Word<'a>; N]) -> Self {
        let mut data = Data {
            words: Default::default(),
            keywords: Default::default(),
        };

        for word in words {
            let _ = data.add(word);
        }

        data
    }
}

impl<'a> Data<'a> {
    pub fn add(&mut self, word: Word<'a>) -> Result<(), Error> {
        if self.words.contains_key(word.value) {
            return Err(Error::WordAlreadyExists);
        }

        for keyword in &word.keywords {
            self.keywords.entry(keyword).or_default().push(word.value);
        }

        self.words.insert(word.value, word);

        Ok(())
    }

    pub fn get_random_word(&self) -> &Word {
        let mut rng = rand::thread_rng();

        self.words.index(rng.gen_range(0..self.words.len()))
    }
}
