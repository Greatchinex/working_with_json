use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    name: String,
    release_year: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    nationality: String,
    books: Vec<Book>,
}

// Sample with static return type
fn parse_json(json_payload: &str) -> Author {
    let parsed: Author = serde_json::from_str(json_payload).unwrap();
    parsed
}

// Sample with dynamic(any data type) return type
fn parse_json_dyn<T>(json_payload: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let parsed_data: Result<T, serde_json::Error> = serde_json::from_str(json_payload);
    parsed_data
}

pub fn read_json() {
    let json_data = r#"{
        "name": "Oscar Wilde",
        "nationality": "Ireland",
        "books": [
            { "name": "The Importance of Being Earnest", "release_year": "1895" },
            { "name": "The Picture of Dorian Gray", "release_year": "1890" },
            { "name": "The Canterville Ghost", "release_year": "1887" }
        ]
    }"#;

    let author = parse_json(json_data);
    println!("Author Struct: {:?}", author);

    // Dynamic generic return type sample
    let author_dyn = match parse_json_dyn::<Author>(json_data) {
        Ok(author_dyn) => author_dyn,
        Err(e) => panic!("read_json error: failed to parse json data {}", e),
    };

    println!("GENERICS STRUCT: {:?}", author_dyn)
}
