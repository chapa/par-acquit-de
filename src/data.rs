use crate::error::Error;
use indexmap::IndexMap;
use rand::Rng;
use rocket::form::{self, Error as FormError};
use serde::Deserialize;
use std::ops::Index;

#[derive(Debug, Deserialize)]
pub struct Word {
    value: String,
    quote: String,
    #[serde(default)]
    keywords: Vec<String>,
}

#[derive(Debug, FromForm)]
pub struct AddWordForm<'a> {
    #[field(validate = validate_value())]
    pub value: &'a str,
    pub quote: &'a str,
    pub keywords: &'a str,
}

pub fn validate_value<'v>(value: &str) -> form::Result<'v, ()> {
    if value.len() < 4 {
        return Err(FormError::validation(
            "L'expression doit faire au moins 4 caractères",
        ))?;
    }

    if !value.starts_with("co")
        || value
            .chars()
            .nth(2)
            .filter(|c| c == &'n' || c == &'m')
            .is_none()
    {
        return Err(FormError::validation(
            "L'expression doit commencer par co et doit avoir comme troisième lettre m ou n",
        ))?;
    }

    if value.chars().nth(2).unwrap() == value.chars().nth(3).unwrap() {
        return Err(FormError::validation(
            "Les deuxième et troisième lettre doivent être différentes",
        ))?;
    }

    Ok(())
}

impl Word {
    pub fn create(value: String, quote: String, keywords: Vec<String>) -> Self {
        Word {
            value,
            quote,
            keywords,
        }
    }

    pub fn from(form: &AddWordForm) -> Self {
        Word {
            value: form.value.to_string(),
            quote: form.quote.to_string(),
            keywords: form.keywords.split(',').map(|s| s.to_string()).collect(),
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    pub fn get_quote(&self) -> &String {
        &self.quote
    }
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

    pub fn get_word(&self, word: &str) -> Result<&Word, Error> {
        self.words
            .get(word.to_lowercase().as_str())
            .ok_or(Error::ThereIsNoWord)
    }
}
