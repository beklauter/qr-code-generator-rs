# QR Code Generator in Rust

A simple command-line tool that generates QR codes from user input and saves them as PNG images with a timestamped filename.

## Features

- Reads a link or any text input from the console.
- Generates a QR code encoding the input text.
- Saves the QR code image as a PNG file in the `output/` directory.
- Adds a timestamp (`YYYYMMDD_HHMMSS`) to the filename for uniqueness.

## Requirements

- Rust (with Cargo) installed via [rustup](https://rustup.rs/)
- Dependencies are specified in `Cargo.toml`:
  - `qrcode` for QR code generation
  - `image` for image creation and saving
  - `chrono` for generating timestamps

## Usage

1. Clone the repository or copy the code.
2. Run the program using Cargo:

```
cargo run
```

3. When prompted, enter the URL or text to encode as a QR code.
4. The generated QR code image will be saved in the `output/` folder, with the timestamped filename.

## Example

```
$ cargo run
Enter the link: https://example.com
QR code saved at "output/qrcode_20250918_192300.png"
```

## License

This project is licensed under the MIT License.