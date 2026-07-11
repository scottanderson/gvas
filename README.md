# gvas

A Rust library for reading and writing Unreal Engine save game files.

This library provides Rust representations of the Unreal Engine types used to
serialize and deserialize `USaveGame` objects. Version-dependent serialization
behavior is detected automatically from header fields.

## Installation

```shell
cargo add gvas
```

Optional features can be enabled as needed:

```shell
cargo add gvas --features palworld,serde
```

| Feature    | Description                                                             |
| ---------- | ----------------------------------------------------------------------- |
| `serde`    | Implements Serde serialization and deserialization for supported types. |
| `palworld` | Adds support for Palworld compressed save-game wrappers.                |

## Usage

A gvas save file is a binary file format used by the Unreal Engine 4+ game
engine to store persistent data such as player progress, game settings, and
other game-related information.

```rust
use binrw::BinRead;
use gvas::types::USaveGame;
use std::fs::File;

let mut file = File::open("save.sav")?;
let save_game = USaveGame::read(&mut file)?;

println!("{:#?}", save_game);
```

### Auto-detecting file type

If the `palworld` feature is enabled, `AutoDetectFile` can read either a plain
GVAS save or a Palworld-wrapped one without knowing in advance which it is:

```rust
use binrw::BinRead;
use gvas::detect::AutoDetectFile;
use std::fs::File;

let mut file = File::open("save.sav")?;
match AutoDetectFile::read(&mut file)? {
    AutoDetectFile::GVAS(save_game) => println!("{:#?}", save_game),
    AutoDetectFile::Palworld(palworld_save) => println!("{:#?}", palworld_save.content),
}
```

For more examples, see [docs.rs/gvas](https://docs.rs/gvas/) or [lib.rs](src/lib.rs).

## Format compatibility

Some older Unreal Engine save files do not contain enough type information to
determine every nested property type. See the crate documentation for details
about these limitations and how unknown properties are represented.

## Contributing

Please see the [CONTRIBUTING](CONTRIBUTING.md) document for guidelines on how
to contribute to this project.

## License

This library is distributed under the terms of the MIT license. See the
[LICENSE](LICENSE) file for details.
