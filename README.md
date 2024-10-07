# Rust Axum server

[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/dfberry/rust-axum-server/badge)](https://scorecard.dev/viewer/?uri=github.com/dfberry/rust-axum-server)

## To run

* `cargo build` or `cargo build --release`
* `bash start-dev-with-env.sh` or `bash start-container-with-env.sh`

## Port 4000

ACA doesn't need 4000 to be exposed in the Dockerfile but local development does. Keep it because it works in both environments.

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

## Build and run container locally

1. Install Visual Studio Code docker extension.
2. Right-click on Dockerfile, and select `Build Image`.
3. Enter image name `source-board-server` at top of screen. Tag will be `latest`.
4. Run with 'bash start-container-with-env.sh`.

## Resources

* https://github.com/hvalfangst/Axum-service-deployed-to-ACI-with-Workflow
* https://github.com/alfredodeza/rust-distroless-azuredi
