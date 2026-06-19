# gvas-binrw

`gvas-binrw` is a library for reading and writing Unreal Engine save game
files. This file format changes between engine major version releases,
requiring programs to account for version-specific behavior. This library
encapsulates those differences so applications can focus on working with
save-game data, rather than Unreal Engine's evolving serialization details.

## Goals

- Provide a Rust implementation for every UE4.18+ struct used in GVAS files
- Derive `binrw::BinRead` and `binrw::BinWrite` whenever possible

## Status

Reading is supported across the legacy and newer ("complete type name")
property tag formats. Writing is implemented for most property types, with full
coverage for the newer format still in progress.

## Coming soon

- Serde

## License

This library is distributed under the terms of the MIT license. See the
[LICENSE](LICENSE) file for details.
