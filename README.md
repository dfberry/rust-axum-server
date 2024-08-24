# Rust Axum server

* `cargo build` or `cargo build --release`
* `cargo run`

## Port 3000

ACA doesn't need 3000 to be exposed in the Dockerfile but local development does. Keep it because it works in both environments.

## Lint

* [Clippy](https://github.com/rust-lang/rust-clippy)

```shell
cargo clippy
```

## Diesel ORM steps

Steps found in [getting started](https://diesel.rs/guides/getting-started.html).

1. Create new migration

    ```shell
    diesel migration generate create_<table_name>
    ```

2. Edit `up.sql` and `down.sql` in migrations subdir.

3. Run migration to run `up.sql`.

    ```shell
    diesel migration run
    ```

4. Run migration again to run both `down.sql` and `up.sql`.

    ```shell
    diesel migration redo
    ```

    This generates the `schema.rs` file. 

## Resources

* https://github.com/hvalfangst/Axum-service-deployed-to-ACI-with-Workflow
* https://github.com/alfredodeza/rust-distroless-azuredi