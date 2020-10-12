# Setup

<!--- TODO validate -->

## Motion

<!--- TODO add motion -->

## Compile

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose nightly.
Clone this repository, cd into it.

<!--- TODO add fix for libsqlite.so -->

```sh
ROCKET_ENV=production
cargo build --release --features iot
cp db_template.sqlite db.sqlite
```

## systemd

<!--- TODO add system -->
