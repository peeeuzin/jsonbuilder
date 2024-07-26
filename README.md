# Json builder
A json builder used to create JSON structures using a simple DSL (based on [Jbuilder](https://github.com/rails/jbuilder)). It is a simple and easy to use tool to create JSON structures in Rust.

## Example
```jb
notes @notes |note| do
    content @note.content
    author do
        name @note.author.name
        age @note.author.age
    end
    comments @note.comments |comment| do
        content @comment.content
        author do
            name @comment.author.name
            age @comment.author.age
        end
    end
end
```

Built with the above DSL, the following JSON structure will be created:
```json
{
    "notes": [
        {
            "content": "Note content",
            "author": {
                "name": "Author name",
                "age": 20
            },
            "comments": [
                {
                    "content": "Comment content",
                    "author": {
                        "name": "Comment author name",
                        "age": 25
                    }
                }
            ]
        }
    ]
}
```

# Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
jsonbuilder = "0.1.0"
```

# Usage
```rust
use jsonbuilder::{JsonBuilder, context_map};

#[derive(Serialize)]
struct Note {
    content: String,
}

fn main() {
    let context = context_map!({
        "note" => Note::new(),
    })

    let json = JsonBuilder::render("data", context)
}
```
JsonBuilder searches for a file that ends with `json.jb` in `templates` directory.

or you can render with raw string:
```rust
use jsonbuilder::JsonBuilder;

#[derive(Serialize)]
struct Note {
    content: String,
}

fn main() {
    let context = context_map!({
        "note" => Note::new(),
    })

    let input = "
    note @note.content

    ...
    "
    let json = JsonBuilder::render_raw(input, context)
}
```

# Settings
you can change the settings by creating a `jsonbuilder.toml` file in the root of your project.
```toml
template_path = "another/path/to/templates"
```

# Syntax
The syntax is based on [Jbuilder](https://github.com/rails/jbuilder) and is very similar to it. The following is a list of the supported syntax:

## Variables
```jb
name @variable
```

## Array mapping
```jb
names @names |item| do
    name @item
end
```

## Object mapping
```jb
person do
    name @person.name
    age @person.age
    _ @person.abilities # _ is a special character that allows you to access the entire object
end
```

# Roadmap
- [x] Basic DSL
- [x] Array mapping
- [x] Object mapping
- [ ] Extend with another templates
- [ ] Conditional statements
- [ ] Merge objects

# Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

See the [contribution guidelines](CONTRIBUTING.md) for more information.

# License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.