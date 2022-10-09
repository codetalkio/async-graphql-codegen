# Async GraphQL Codegen: Schema First Approach to GraphQL

A schema generator for [async-graphql](https://github.com/async-graphql/async-graphql) 4.x.

> ⚠️ This is a fork from [linksplatform/gql-gen](https://github.com/linksplatform/gql-gen), which forks [uselessgoddess/codegen-for-async-graphql](https://github.com/uselessgoddess/codegen-for-async-graphql) which forks the original project at [atsuhiro/codegen-for-async-graphql](https://github.com/atsuhiro/codegen-for-async-graphql). All of them seem unmaintained.

- [Async GraphQL Codegen: Schema First Approach to GraphQL](#async-graphql-codegen-schema-first-approach-to-graphql)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Example](#example)

## Installation

In order to install, just run the following command

```bash
$ git clone git@github.com:codetalkio/async-graphql-codegen.git
$ cd async-graphql-codegen
$ cargo build --bin cargo-gql-gen
$ ln -s $(pwd)/target/debug/gql-gen ~/.cargo/bin/gql-gen
```

## Usage

1. Set up your Rust project
2. Create a folder inside your `src/` folder to contain the generated code (e.g. `src/schema/`)
3. Create your GraphQL schema and store it somewhere (e.g. `schema.graphql`)
4. Run `gql-gen --schema schema.graphql --output ./src/schema` from the root of your project

Voila! 🎉 You have now generated Rust code from your schema types.

### Example
Check out the [examples](./examples/) folder, which sets up the necessary structure along with an axum service (adjusted from [async-graphql/examples/axum/starwars](https://github.com/async-graphql/examples/tree/bb0fa782053271096cf8c61eaf6e670b9d08ae15/axum/starwars)):

```bash
.
├── Cargo.toml
├── schema.graphql
└── src
    ├── main.rs
    └── schema
        └── .gitignore
```

We need a `schema` (or similar folder) to output the generated code into, and we of course need the schema itself, which is found in `schema.graphql` for the example case.

With everything set up, we are now ready to generate the code from the schema:

```bash
$ cd examples
$ gql-gen --schema schema.graphql --output ./src/schema
```

You can then check out the generated code in the [examples/src/schema](./examples/src/schema/) folder.
