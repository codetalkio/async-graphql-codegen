# Async GraphQL Codegen: Schema First Approach to GraphQL

A schema generator for [async-graphql](https://github.com/async-graphql/async-graphql) 4.x.

> ⚠️ This is a fork from [linksplatform/gql-gen](https://github.com/linksplatform/gql-gen), which forks [uselessgoddess/codegen-for-async-graphql](https://github.com/uselessgoddess/codegen-for-async-graphql) which forks the original project at [atsuhiro/codegen-for-async-graphql](https://github.com/atsuhiro/codegen-for-async-graphql). All of them seem unmaintained.

## Quick start

### Installation

In order to install, just run the following command

```bash
$ git clone git@github.com:codetalkio/async-graphql-codegen.git
$ cd async-graphql-codegen
$ cargo build --bin cargo-gql-gen
$ ln -s $(pwd)/target/debug/gql-gen ~/.cargo/bin/gql-gen
```

### Usage

**Generate async-graphql schema in 4 easy steps**

1. Create a new empty rust module

```rust
//! main.rs
mod schema;

...
```

2. Put your schema to any folder

```graphql
# example schema
type Book {
    id: ID!
    name: String!
    author: String!
}

input InputBook {
    name: String!
    author: String!
}

type QueryRoot {
    books: [Book!]
}

type MutationRoot {
    createBook(book: InputBook!): Book
}
```

3. Run gql-gen

```shell
# in project/src
$ gql-gen -- --schema schema.graphql --output schema
```

4. Enjoy your generation

```rust
//! book.rs
use async_graphql::*;

#[derive(Debug)]
pub struct Book;

#[Object]
impl Book {
    pub async fn id(&self, ctx: &Context<'_>) -> ID {
        todo!()
    }

    pub async fn name(&self, ctx: &Context<'_>) -> String {
        todo!()
    }

    pub async fn author(&self, ctx: &Context<'_>) -> String {
        todo!()
    }
}
```

```rust
//! input_book.rs
use async_graphql::*;

#[derive(InputObject, Debug)]
pub struct InputBook {
    pub name: String,
    pub author: String,
}
```

```rust
//! query_root.rs
use super::super::Book;
use async_graphql::*;

#[derive(Debug)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn books(&self, ctx: &Context<'_>) -> Option<Vec<Book>> {
        todo!()
    }
}
```

```rust
//! mutation_root.rs
use super::super::{Book, InputBook};
use async_graphql::*;

#[derive(Debug)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_book(&self, ctx: &Context<'_>, book: InputBook) -> Option<Book> {
        todo!()
    }
}
```
