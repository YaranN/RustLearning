# Scalable Web Development In Rust
## Database
### Setup Diesel Database
Object-Relational Mapping (ORM) Tool - Diesel in RUST
#### Install Rust
`curl https://sh.rustup.rs -sSf | sh`
#### Install Homebrew
`/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
`(echo; echo 'eval "$(/opt/homebrew/bin/brew shellenv)"') >> /Users/xnan/.zprofile eval "$(/opt/homebrew/bin/brew shellenv)"`
#### Install Postgresql with brew
`brew install postgresql@15`
`echo 'export PATH="/opt/homebrew/opt/postgresql@15/bin:$PATH"' >> ~/.zshrc`
`export LDFLAGS="-L/opt/homebrew/opt/postgresql@15/lib"`
`export CPPFLAGS="-I/opt/homebrew/opt/postgresql@15/include"`
#### Install backend libraries
Diesel supports SQLite, PostgreSQL, and MySQL as database backends. By default, diesel_cli requires the client library of all three backends to be installed. If one is missing, then cargo install diesel_cli will throw an error. To install diesel_cli without all backends, specify --no-default-features. Use cargo's --features option to specify postgres, sqlite, and/or mysql. For example, to install with sqlite only, run:
`cargo install diesel_cli --no-default-features --features postgres`
1. Create Rust project
`cargo cargo new porject_name --bin`
2. add database connector .env file at the root of project
`touch .env` 
3. Start a server and create database on pgAdmin
4. Edit .env file for Postgresql
`DATABASE_URL=postgresql://xnan:xnan@localhost:5432/diesel_example`
`TEST_DATABASE_URL=postgresql://xnan:xnan@localhost:5432/diesel_test`
5. Creat a product
'diesel migration generate create_products
6. Up.sql and down.sql
'CREATE TABLE products (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  cost DOUBLE PRECISION NOT NULL,
  active BOOLEAN NOT NULL 
)
'drop table products
7. Run migrarion
'diesel migration run
8. Model.rs
### Database Assignment
## Backend
### Backend Assignment
## Frontend
### Frontend Assignment
## Mini Project
s


