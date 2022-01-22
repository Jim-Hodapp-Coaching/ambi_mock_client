# ambi_mock_client

## Usage

You must have Rust installed to build `ambi_mock_client`.
You can find documentation on installing Rust [here](https://www.rust-lang.org/tools/install).

You will also need a local copy of [Ambi](https://github.com/jhodapp/ambi) running ( default port 4000 ).

### Using cargo run
```BASH
> cargo build
> cargo run

Sending POST request to http://localhost:4000/api/readings/add as JSON: {"tempurature":"19.2","humidity":"87.7","pressure":"1074","dust_concentration":"415","air_purity":"DANGEROUS"}
Response: Ok(
    Response {
        url: Url {
            scheme: "http",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: Some(
                Domain(
                    "localhost",
                ),
            ),
            port: Some(
                4000,
            ),
            path: "/api/readings/add",
            query: None,
            fragment: None,
        },
        status: 200,
        headers: {
            "cache-control": "max-age=0, private, must-revalidate",
            "content-length": "60",
            "content-type": "application/json; charset=utf-8",
            "date": "Sat, 22 Jan 2022 19:25:14 GMT",
            "server": "Cowboy",
            "x-request-id": "FsyuNssWKjhYHbUAAAAj",
        },
    },
)

# Or just

> cargo run
```

### As an executable binary
```BASH
> cargo build
> ./target/debug/ambi_mock_client

Sending POST request to http://localhost:4000/api/readings/add as JSON: {"tempurature":"28.8","humidity":"85.2","pressure":"964","dust_concentration":"930","air_purity":"DANGEROUS"}
Response: Ok(
    Response {
        url: Url {
            scheme: "http",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: Some(
                Domain(
                    "localhost",
                ),
            ),
            port: Some(
                4000,
            ),
            path: "/api/readings/add",
            query: None,
            fragment: None,
        },
        status: 200,
        headers: {
            "cache-control": "max-age=0, private, must-revalidate",
            "content-length": "60",
            "content-type": "application/json; charset=utf-8",
            "date": "Sat, 22 Jan 2022 19:28:08 GMT",
            "server": "Cowboy",
            "x-request-id": "FsyuX1U1NOj9m7IAAABB",
        },
    },
)
```