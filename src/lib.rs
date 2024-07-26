use context::Context;
use serde_json::Value;
use std::path::Path;

pub mod builder;
mod config;
pub mod context;
pub mod error;
pub mod parser;
mod reader;

use error::*;

#[macro_export]
macro_rules! context_map {
    // Base case: empty map
    (@single $($x:tt)*) => (());

    // Case for a map with a single key-value pair
    (@count $($rest:expr),*) => (<[()]>::len(&[$(context_map!(@single $rest)),*]));

    // Case for a map with multiple key-value pairs
    ($($key:expr => $value:expr,)+) => { context_map!($($key.to_string() => $value),+) };

    // Recursive case: build a map from multiple key-value pairs
    ($($key:expr => $value:expr),*) => {
        {
            use std::collections::HashMap;

            let _cap = context_map!(@count $($key),*);
            let mut _map = HashMap::with_capacity(_cap);
            $(
                _map.insert($key, context_map!($value));
            )*

            $crate::context::Context::new(_map)
        }
    };

    // Case for a single value (not a map)
    ($value:expr) => {
        serde_json::to_value($value).unwrap()
    };
}

pub struct JsonBuilder;

impl JsonBuilder {
    pub fn render<P>(path: P, context: Context) -> Result<Value>
    where
        P: AsRef<Path>,
    {
        let input = reader::read(path.as_ref().to_path_buf())?;
        let ast = parser::parse(&input)?;
        builder::build(ast, context)
    }

    pub fn render_raw(input: &str, context: Context) -> Result<Value> {
        let ast = parser::parse(input)?;
        builder::build(ast, context)
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::json;

    use super::*;

    #[derive(Serialize)]
    struct User {
        name: String,
        age: u32,
        password: String,
    }

    #[derive(Serialize)]
    struct Note {
        content: String,
        author: User,
        comments: Vec<Comment>,
    }

    #[derive(Serialize)]
    struct Comment {
        content: String,
        author: User,
    }

    fn notes() -> Vec<Note> {
        vec![
            Note {
                content: "Hello, world!".to_string(),
                author: User {
                    name: "John Doe".to_string(),
                    age: 30,
                    password: "password".to_string(),
                },
                comments: vec![
                    Comment {
                        content: "Nice!".to_string(),
                        author: User {
                            name: "Jane Doe".to_string(),
                            age: 25,
                            password: "password".to_string(),
                        },
                    },
                    Comment {
                        content: "Great!".to_string(),
                        author: User {
                            name: "Jack Doe".to_string(),
                            age: 35,
                            password: "password".to_string(),
                        },
                    },
                ],
            },
            Note {
                content: "Goodbye, world!".to_string(),
                author: User {
                    name: "Jane Doe".to_string(),
                    age: 25,
                    password: "password".to_string(),
                },
                comments: vec![
                    Comment {
                        content: "Nice!".to_string(),
                        author: User {
                            name: "John Doe".to_string(),
                            age: 30,
                            password: "password".to_string(),
                        },
                    },
                    Comment {
                        content: "Great!".to_string(),
                        author: User {
                            name: "Jack Doe".to_string(),
                            age: 35,
                            password: "password".to_string(),
                        },
                    },
                ],
            },
        ]
    }

    #[test]
    fn note_template_test() {
        let note = &notes()[0];

        let context = context_map! {
            "note" => note,
        };

        let value = JsonBuilder::render("tests/object_template_test", context).unwrap();

        assert_eq!(
            value,
            serde_json::json!({
                "content": "Hello, world!",
                "author": {
                    "name": "John Doe",
                    "age": 30,
                },
            })
        );
    }

    #[test]
    fn notes_template_test() {
        let context = context_map! {
            "notes" => notes(),
        };

        let value = JsonBuilder::render("tests/array_template_test", context).unwrap();

        assert_eq!(
            value,
            json!({
                "notes": [
                    {
                        "content": "Hello, world!",
                        "author": {
                            "name": "John Doe",
                            "age": 30,
                        },
                        "comments": [
                            {
                                "content": "Nice!",
                                "author": {
                                    "name": "Jane Doe",
                                    "age": 25,
                                },
                            },
                            {
                                "content": "Great!",
                                "author": {
                                    "name": "Jack Doe",
                                    "age": 35,
                                },
                            },
                        ],
                    },
                    {
                        "content": "Goodbye, world!",
                        "author": {
                            "name": "Jane Doe",
                            "age": 25,
                        },
                        "comments": [
                            {
                                "content": "Nice!",
                                "author": {
                                    "name": "John Doe",
                                    "age": 30,
                                },
                            },
                            {
                                "content": "Great!",
                                "author": {
                                    "name": "Jack Doe",
                                    "age": 35,
                                },
                            },
                        ],
                    }
                ]
            })
        );
    }
}
