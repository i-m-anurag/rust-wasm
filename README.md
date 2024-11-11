# Rust Encryption WebAssembly Integration

This project demonstrates how to use Rust and WebAssembly (Wasm) to encrypt and decrypt data in a web environment. The encryption is implemented in Rust using AES-128 with CBC mode and PKCS7 padding, then compiled to WebAssembly, allowing it to be called from JavaScript in the browser.

## Project Structure

- **src/lib.rs**: Contains the Rust implementation of the `SecurityService` with `set` and `get` functions for encryption and decryption.
- **index.html**: HTML file with input fields and buttons to test the encryption and decryption functions.
- **JavaScript Module**: Interacts with the WebAssembly module, handling function calls to `set` and `get`.

## Features

- **AES-128 Encryption**: Fixed 16-byte key is used for encryption in CBC mode with PKCS7 padding.
- **Double Base64 Encoding**: Encrypted data is Base64 encoded twice for compatibility with other implementations.
- **WebAssembly Integration**: Allows the encryption and decryption functions to be called from JavaScript, making them accessible from a web interface.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Installation and Setup

1. **Install wasm-pack** (if not already installed):
   ```bash
   cargo install wasm-pack
   ```
2. **Cargo Build Clean**:
   ```bash
   cargo clean
   ```
3. **Cargo Build**:
   ```bash
   cargo build
   ```
4. **Cargo Run**:
   ```bash
   cargo run
   ```
