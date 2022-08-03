# Cargo

## Starting a new project

You can start a new Rust project using cargo by running:

```
cargo new project-path
```

The cargo new command allows you to specify options of the new project being
created like

1. `--registry` Allows you to specify the package registry to use. Defaults to
   `crates.io`
2. `--vcs` Allows you to specify the version control system. Defaults to `git`
3. `--name` Allowss you to specify the package name of the project being
   created. Defaults to the `project-path`

You can use `cargo new --help` to see different options that an be passed to
the `cargo new` command

## Adding dependencies

You can add dependencies to your Rust project by either directly adding the
dependency to the dependencies section of the `Cargo.toml` file or
alternatively, by using the `cargo add pkg-name` command.

## Building your project

You can build your project using the `cargo build` command. By default, this
will build a `debug` version of your project. To build a release version pass  
the `--release`or `-r` option to the command.

```
cargo build --release
```

## Running your project

You can run your project using the `cargo run` command. As with the build
command, it will by default run a debug version of your code. Pass the
`--release` option to run the optimized version.
