# Rapid CDS Parser
[![ci](https://github.com/rapid-d9t/rapid-cds-parser/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/rapid-d9t/rapid-cds-parser/actions/workflows/ci.yml)
[![Hits-of-Code](https://hitsofcode.com/github/rapid-d9t/rapid-cds-parser?branch=main)](https://hitsofcode.com/github/rapid-d9t/rapid-cds-parser/view?branch=main)

npm module for rapid parsing of cds written in rust as component of [rapidcds extention](https://github.com/rapid-d9t/rapidcds)

## Installing 

Installing of rapid-cds-parser requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

## Exploring rapid-cds-parser

After building rapid-cds-parser, you can explore its exports at the Node REPL:

```sh
$ npm install
$ node
> require('.').transpile()
```

## Available Scripts

In the project directory, you can run:

### `npm install`

Installs the project, including running `npm run build`.

### `npm build`

Builds the Node addon (`index.node`) from source.

### `npm test`

Runs the unit tests by calling `cargo test`. You can learn more about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from the [Rust book](https://doc.rust-lang.org/book/).

## Project Layout

The directory structure of this project is:

```
rapid-cds-parser/
├── Cargo.toml
├── README.md
├── index.node
├── package.json
├── src/
|   └── lib.rs
|   └── cds.lalrpop
|   └── ast/
└── target/
```

### Cargo.toml

The Cargo [manifest file](https://doc.rust-lang.org/cargo/reference/manifest.html), which informs the `cargo` command.

### README.md

This file.

### index.node

The Node addon—i.e., a binary Node module—generated by building the project. This is the main module for this package, as dictated by the `"main"` key in `package.json`.

Under the hood, a [Node addon](https://nodejs.org/api/addons.html) is a [dynamically-linked shared object](https://en.wikipedia.org/wiki/Library_(computing)#Shared_libraries). The `"build"` script produces this file by copying it from within the `target/` directory, which is where the Rust build produces the shared object.

### package.json

The npm [manifest file](https://docs.npmjs.com/cli/v7/configuring-npm/package-json), which informs the `npm` command.

### src/

The directory tree containing the Rust source code for the project.

### src/cds.lalrpop

Definitions of cds syntax.

### src/ast

Implementations of cds terms.

### src/lib.rs

The Rust library's main module.

### target/

Binary artifacts generated by the Rust build.
