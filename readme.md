# Serialize/Deserialize Macro

A custom Rust procedural macro library for binary serialization and deserialization of structs with numeric fields and strings.

## Overview

This project provides derive macros that automatically implement binary serialization and deserialization for structs containing numeric types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`) and `String` fields.

## Project Structure

- `serialize_macro/` - Contains the procedural macros
- `serialize_macro_traits/` - Defines the `Serialize` and `Deserialize` traits
- `app/` - Example application demonstrating usage

## Features

- **Binary Serialization**: Converts struct data to binary format using big-endian byte order
- **Binary Deserialization**: Reconstructs structs from binary data
- **String Support**: Handles variable-length strings with length prefixes
- **Numeric Types**: Supports all standard integer types
- **Derive Macros**: Simple `#[derive()]` syntax for automatic implementation

## Usage

### Add Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
serialize_macro = { path = "serialize_macro" }
serialize_macro_traits = { path = "serialize_macro_traits" }
```

### Basic Example

```rust
use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct, Debug)]
struct Person {
    age: u32,
    height: u16,
    name: String,
}

fn main() {
    let person = Person {
        age: 25,
        height: 175,
        name: "Alice".to_string(),
    };

    // Serialize to binary
    let serialized = person.serialize();
    println!("Serialized data: {:?}", serialized);

    // Deserialize from binary
    let deserialized = Person::deserialize(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

## Supported Types

- `u8`, `u16`, `u32`, `u64` - Unsigned integers
- `i8`, `i16`, `i32`, `i64` - Signed integers
- `String` - Variable-length UTF-8 strings

## Binary Format

- **Numeric fields**: Serialized as big-endian bytes
- **String fields**: Length (as `u32`) followed by UTF-8 bytes

## Limitations

- Only supports structs with named fields
- Only supports the listed numeric types and `String`
- No support for nested structs, enums, or collections
- Fixed big-endian byte order

## Building and Running

```bash
# Build the entire workspace
cargo build

# Run the example
cargo run --bin app
```

## Error Handling

Deserialization returns `Result<T, Error>` and will fail if:

- Input data is too short
- String data contains invalid UTF-8
- Array conversion fails
