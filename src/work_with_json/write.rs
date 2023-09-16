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

fn serialize_json(author: &Author) -> String {
    return serde_json::to_string(author).unwrap();
}

pub fn write_json() {
    let author = Author {
        name: String::from("George Orwell"),
        nationality: String::from("Ireland"),
        books: vec![
            Book {
                name: String::from("Nineteen Eighty-Four(1984)"),
                release_year: String::from("1949"),
            },
            Book {
                name: String::from("Animal Farm"),
                release_year: String::from("1945"),
            },
            Book {
                name: String::from("Burmese Days"),
                release_year: String::from("1934"),
            },
        ],
    };

    let json_payload = serialize_json(&author);
    println!("JSON PAYLOAD: {}", json_payload)
}
