# ambi_mock_client

## Usage

You must have Rust installed to build `ambi_mock_client`.
You can find documentation on installing Rust [here](https://www.rust-lang.org/tools/install).

### Using cargo run
```Rust
> cargo build
> cargo run
{"tempurature":"23.7","humidity":"63.8","pressure":"945","dust_concentration":"999","air_purity":"DANGEROUS"}

# Or just

> cargo run
{"tempurature":"33.5","humidity":"39.6","pressure":"1100","dust_concentration":"8","air_purity":"FRESH_AIR"}
```

### As an executable binary
```Rust
> cargo build
> ./target/debug/ambi_mock_client
{"tempurature":"25.9","humidity":"43.4","pressure":"1076","dust_concentration":"322","air_purity":"DANGEROUS"}
```