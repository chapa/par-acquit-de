use crate::error::Error;
use indexmap::IndexMap;
use rand::Rng;
use rocket::form::{self, Error as FormError};
use serde::Deserialize;
use std::ops::Index;
use std::sync::RwLock;

#[derive(Clone, Debug, Deserialize)]
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
struct InnerData {
    words: IndexMap<String, Word>,
    keywords: IndexMap<String, Vec<String>>,
}

#[derive(Debug, Default)]
pub struct Data(RwLock<InnerData>);

impl<const N: usize> From<[Word; N]> for Data {
    fn from(words: [Word; N]) -> Self {
        let data: Data = Default::default();

        for word in words {
            let _ = data.add(word);
        }

        data
    }
}

impl Data {
    pub fn from_path(path: &'static str) -> Self {
        let data: Data = Default::default();

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

    pub fn add(&self, word: Word) -> Result<(), Error> {
        match self.0.write() {
            Ok(mut data) => {
                if data.words.contains_key(&word.value) {
                    return Err(Error::WordAlreadyExists);
                }

                for keyword in &word.keywords {
                    data.keywords
                        .entry(keyword.clone())
                        .or_default()
                        .push(word.value.clone());
                }

                data.words.insert(word.value.clone(), word);

                Ok(())
            }
            Err(_) => Err(Error::LockPoisoned),
        }
    }

    pub fn get_random_word(&self) -> Result<Word, Error> {
        match self.0.read() {
            Ok(data) => {
                if data.words.is_empty() {
                    return Err(Error::ThereIsNoWord);
                }

                let mut rng = rand::thread_rng();

                Ok(data.words.index(rng.gen_range(0..data.words.len())).clone())
            }
            Err(_) => Err(Error::LockPoisoned),
        }
    }

    pub fn get_word(&self, word: &str) -> Result<Word, Error> {
        match self.0.read() {
            Ok(data) => data
                .words
                .get(word.to_lowercase().as_str())
                .map(Clone::clone)
                .ok_or(Error::ThereIsNoWord),
            Err(_) => Err(Error::LockPoisoned),
        }
    }
}
