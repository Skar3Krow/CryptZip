# CryptZip

This documentation provides detailed information about the HTTP server implemented in Rust that supports simple GET/POST commands and gzip compression. The server is designed to handle basic key-value storage operations with efficient data transfer using gzip compression.

## Features

1. GET Command: Retrieve the value associated with a specific key.
2. POST Command: Store a value with a specific key.
3. Gzip Compression: Support for gzip compression to reduce data transfer size.

## Requirements

1. Rust (version 1.50 or later)
2. cargo (version 1.7 or later)
3. std crate for TCP Server, Buffer(Reader/Writer) and Threads
4. flate2 crate for gzip compression

## Installation

1. Clone the repository:

```sh
git clone https://github.com/Skar3Krow/CryptZip.git
cd CryptZip
```

2. Ensure Rust is installed:

```sh
rustc --version
```

3. Build the project:

```sh
cargo build --release
```

## Usage

### Starting the Server
To start the server, run the following command in the project directory:

```sh
cargo run --release
./your_server.sh
```
The server will start listening on the default port 4221. You can customize the port by modifying the PORT constant in the main.rs file.

## Supported Endpoints
### GET /get

Retrieve the value associated with a specific key.

- URL: /get

- Method: GET

- Query Parameters:

  - key: The key for which the value is to be retrieved.

- Response:
  - 200 OK: If the key exists.
  - 404 Not Found: If the key does not exist.

#### Example:

1. echo
```sh
curl -v http://localhost:4221/echo/orange
```

2. Reading a header
```sh
curl -v http://localhost:4221/user-agent -H "User-Agent: pineapple/grape-orange"
```

4. Returning a file :
   - Prints file content if file exists
   - Shows 404 Not Found if file doesnt exist
```sh
curl -v http://localhost:4221/files/banana_banana_raspberry_grape
```
### POST /post

Store a value with a specific key.

- URL: /post

- Method: POST

- Request Body :

  - key: The key to be stored.
  - value: The value to be stored.
- Response:

  - 200 OK: If the value is successfully stored.
    
#### Example:

- Read request body
```sh
curl -v -X POST http://localhost:4221/files/mango_banana_strawberry_pear -H "Content-Length: 64" -H "Content-Type: application/octet-stream" -d 'pineapple pear raspberry apple blueberry strawberry orange mango'
```

### Gzip Compression
The server supports gzip compression to reduce the size of responses. To request gzip compression, include the Accept-Encoding: gzip header in your request.

#### Example:
```sh
curl -v http://localhost:4221/echo/pear -H "Accept-Encoding: gzip"
```
