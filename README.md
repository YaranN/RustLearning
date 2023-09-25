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
### Database Assignment
## Backend
### Backend Assignment
## Frontend
### Frontend Assignment
## Mini Project
s


