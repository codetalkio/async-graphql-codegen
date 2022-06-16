# aqlgen

A schema generator for [async-graphql](https://github.com/async-graphql/async-graphql) 4.x

## Quick start
### Installation 
In order to install, just run the following command
```shell
cargo install aqlgen
```
### Usage
**Generate async-graphql 4.x schema in 4 easy steps**
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
3. Run aqlgen
```shell
# in project/src
cargo aqlgen --schema schema.graphql --output schema
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
use super::super::Book;
use super::super::InputBook;
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
