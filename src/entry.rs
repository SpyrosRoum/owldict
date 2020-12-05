use std::fmt;

use colored::*;
use serde::Deserialize;

/// Represents a dictionary entry
#[derive(Deserialize, Debug)]
pub(crate) struct Entry {
    word: String,
    pronunciation: Option<String>,
    definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug)]
struct Definition {
    #[serde(rename(deserialize = "type"))]
    class: String,
    definition: String,
    example: String,
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.class.italic())?;
        writeln!(f, "{}", self.definition)?;
        let example = format!("\"{}\"", self.example);
        write!(f, "{}", example.dimmed())?;

        Ok(())
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word.bold())?;
        let word_len = if let Some(pron) = &self.pronunciation {
            write!(f, " - {}", pron)?;
            self.word.len() + 3 + pron.len()
        } else {
            self.word.len()
        };
        for def in self.definitions.iter() {
            writeln!(f, "\n{}\n", "-".repeat(word_len))?;
            writeln!(f, "{}", def)?;
        }

        Ok(())
    }
}
