# BACKEND-API-Rust

This is a reimplementation of [Laravel-based REST API taught by my teacher](https://github.com/muflikhandimasd/BACKEND-API)
in Rust, leveraging the power of [Actix](https://actix.rs) as the web framework and
[Diesel](https://diesel.rs) as the ORM.

This project is my Rust-learning media, so don't expect fully working, ready-to-use code and
I might switch to other frameworks without prior notice.

## How to use

Make sure that you have [Rust](https://rust-lang.org) and MySQL/MariaDB installed.
I personally prefer [MariaDB](https://mariadb.org) since it is faster while
keeping fully open-source.

Clone and `cd` to this repository.
```bash
git clone https://github.com/fathonix/BACKEND-API-Rust.git
cd BACKEND-API-Rust
```

Add `.env` file to the root of this repo and set `DATABASE_URL` accordingly.
If you got MariaDB from XAMPP or Laragon, most likely the account is `root`
and the password is blank. The database name is up to you.
```bash
DATABASE_URL=mysql://root@127.0.0.1/backend_db
```

Install `diesel_cli` with MySQL support enabled. Make sure that you have
added `~/.cargo/bin` to your `PATH`.
```bash
cargo install diesel_cli --no-default-features --features mysql
```

Run `diesel` to run the migrations.
```bash
diesel migration run
```

Finally, build and run the project.
```bash
cargo run
```

## Performance?

My personal benchmark using [Hoppscotch](https://hoppscotch.io), measured in milliseconds (ms)
on a base M1 MBP. YMMV, especially Rust can be as slow as 150ms at the start when `GET`ting one item.

| Actions           | Laravel    | Actix/Diesel |
| ----------------- | ---------- | ------------ |
| `GET` all items   | ~1300-1600 | ~315-360     |
| `GET` one item    | ~20-30     | ~5-7         |
| Creating one item | ~150       | ~15          |
| Updating one item | ~130       | ~10          |

~~^For some reason updating to the database does not work yet, this is taken from the server response.~~ Apparently it worked but the getting part didn't. Fixed

## Todos

* Fix updating items
* Implement token-based authentication (JWT)
* Code cleanup

## License

This project is licensed under MIT. © 2023 Aldo Adirajasa Fathoni