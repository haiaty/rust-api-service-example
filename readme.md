
This is an example of an API server/ Backend written in Rust (using actix framework). In order to construct the response some api calls to other web apis are made (using hyper).




How to get it running:

## Manual installation

* install rust: https://www.rust-lang.org/tools/install


*  clone the repository from:

`
git clone https://github.com/haiaty/rust-api-service-example.git
`

*  run:

`
cargo run
`

* run tests:

NOTE: since we have test only in the Pokemons module we filter it

`
cargo test -p Pokemons
`



### references: 

Some concepts used are part of the "Lucid Architecture"
Lucid Architecture: https://lucid-architecture.gitbook.io/docs/