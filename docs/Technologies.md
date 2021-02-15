# Technologies

## Language

The project is written in [Rust](https://www.rust-lang.org/) ([Documentation](https://doc.rust-lang.org/book/)) as the performance footprint is low and it guarantees a relatively high security if it's done properly.

### GUI

The GUI is written in HTML with [Tera](https://tera.netlify.app) as an templating engine. The styling is done via SCSS.

## Framework

The server-side webframework used in this projet is [Rocket](https://rocket.rs/) ([Guide](https://rocket.rs/v0.4/guide/), [Documentation](https://api.rocket.rs/v0.4/rocket/)).

## Important Libraries

### [Rusqlite](https://crates.io/crates/rusqlite)

Used to communicate with the SQLite database and works greatly with Rocket together.
[Documentation](https://docs.rs/rusqlite/latest/rusqlite/)

### [BLAKE2](https://crates.io/crates/blake2)

Used for hashing the passwords as of this is a very modern hashing algorithm.
[Documentation](https://docs.rs/blake2/latest/blake2/)

### [rumqttc](https://crates.io/crates/rumqttc)

A MQTT client in order to communicate with the flats.
[Documentation](https://docs.rs/rumqttc/latest/rumqttc/)

### [rust_gpiozero](https://crates.io/crates/rust_gpiozero)

Used for interacting with the physical world.
[Documentation](https://docs.rs/rust_gpiozero/latest/rust_gpiozero/)
