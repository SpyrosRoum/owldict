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
