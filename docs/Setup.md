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

See [fix](https://users.rust-lang.org/t/linker-cannot-find-lsqlite3/23230).

```sh
ROCKET_ENV=production
cargo build --release --features iot
cp db_template.sqlite db.sqlite
```

## systemd

<!--- TODO add system -->

## Start

Go to \<IP-adress-of-Pi\>.
Enter the credentials 'admin', 'admin'.
Change the credentials.
