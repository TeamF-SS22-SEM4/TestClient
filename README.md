# TestClient

Test client for the REST Backend, of the Tomify Musicshop.

Requires [cargo](https://doc.rust-lang.org/cargo) to be installed.

# Start

    cargo run

or 

    cargo build --release
    ./target/release/test-client.exe

This requires the API to be running on localhost:8080. Add the argument <code>remote</code> to the start command to connect to the deployed version.
