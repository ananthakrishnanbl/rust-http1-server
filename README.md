# rust-http1-server

> 🚧 **Work in progress** — this project is under active construction. Expect incomplete features and changing internals.

A minimal HTTP/1.1 server written from scratch in Rust using only the standard library. No frameworks, no async runtime, no external crates — just `std::net`, `std::io` and `std::fs`. The goal is to understand how HTTP works at the wire level by building the request parser, response writer and static file server by hand.

## What it does today

- Listens for TCP connections on `127.0.0.1:3000`
- Parses the HTTP request line (method, target, version), headers and an optional body based on `Content-Length`
- Serves static files from a `./public` directory
  - `/` maps to `./public/index.html`
  - Percent-decodes the request path and strips query strings and fragments
  - Rejects `..` segments, backslashes and null bytes, and canonicalizes paths so requests cannot escape the public root
  - Detects the `Content-Type` from the file extension (HTML, CSS, JS, JSON, images, fonts, audio/video and more)
- Builds and writes HTTP/1.1 responses with `Content-Type`, `Content-Length` and `Connection` headers
- Uses a custom `Error` type and `Result` alias across modules

## Project structure

```
.
├── error.rs          # Custom Error enum, Display impl and Result alias
├── main.rs           # TCP listener, accept loop and per-connection handler
├── request.rs        # Request struct and parsing (request line, headers, body)
├── response.rs       # Response struct and writing the response to the stream
├── static_file
│   ├── file.rs       # MIME type detection and file reading
│   └── path.rs       # Target decoding and safe path resolution
└── static_file.rs    # Static file serving entry point (StaticServe)
```

### Request flow

```
TcpStream → read_request() → static_serve(target) → Response::from(StaticServe) → write_response()
```

## Running

```bash
mkdir -p public
echo "<h1>Hello</h1>" > public/index.html
cargo run
```

Then open <http://127.0.0.1:3000>.

## Roadmap / To do

### Connection handling
- [ ] Persistent connections — keep reading requests on the same stream when `keep-alive` is used
- [ ] Handle multiple clients concurrently (thread per connection or a thread pool)
- [ ] Read and write timeouts for idle or slow clients
- [ ] Graceful shutdown

### Request handling
- [ ] Method handling — respond appropriately to `GET`, `HEAD` and unsupported methods (`405 Method Not Allowed`)
- [ ] Validate the HTTP version
- [ ] Limits on request line, header and body size
- [ ] `Transfer-Encoding: chunked` request bodies
- [ ] Query string parsing

### Responses
- [ ] Map errors to the right status codes (`400`, `403`, `404`, `500`, etc.)
- [ ] Standard headers such as `Date` and `Server`
- [ ] Caching headers (`Last-Modified`, `ETag`, `304 Not Modified`)
- [ ] Range requests (`206 Partial Content`) for media files
- [ ] Custom error pages

### Routing
- [ ] Simple router for dynamic handlers alongside static files
- [ ] Directory index handling for subfolders

### Project & tooling
- [ ] Configurable host, port and public directory (CLI args or env vars)
- [ ] Structured logging instead of debug prints
- [ ] Unit and integration tests
- [ ] Benchmarks
- [ ] Documentation comments

## License

TBD