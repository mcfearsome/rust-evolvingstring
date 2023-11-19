# EvolvingString Library

The EvolvingString library is a Rust crate that provides functionalities for creating and managing evolving strings, where the evolution of the string is determined by a SHA256 hash function based on an initial string, a secret, and a time interval.

## Features

- Create an instance of EvolvingString with an initial string, secret, and an interval for evolution.
- Calculate the current state of the string based on the elapsed time since the instance was created.
- Predict the state of the string after a specific amount of time has passed.
- Serialize and deserialize EvolvingString instances to and from a base64 representation.

## Usage

To use the EvolvingString library in your Rust project, add the following to your `Cargo.toml` file under `[dependencies]`:

```toml
evolvingstring = 0.1.0
```

You can then create an EvolvingString instance and use its methods as follows:

```rust
use evolvingstring::EvolvingString;

let es = EvolvingString::new("initial value", "secret key", 60);
let current_state = es.current();
let predicted_state = es.predict(120);
let b64_encoded = es.to_base64();
let es_from_b64 = EvolvingString::from_base64(&b64_encoded).unwrap();
```

Replace "initial value" and "secret key" with your own initial string and secret, and choose an appropriate interval in seconds for the evolution of the string.

## Testing

The library comes with an extensive suite of tests to ensure the correctness of the implemented features. You can run the tests using the `cargo test` command.

## Author

EvolvingString is maintained by Jesse McPherson (<jesse@mcfearsome.dev>). For any questions or contributions, please contact the author directly.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
