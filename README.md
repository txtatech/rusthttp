# rusthttp

A very small experimental http server written in Rust with an embedded website.

Once the server is running, you can access it through your web browser by visiting `http://127.0.0.1:8080/`.

## Routes

### GET /

Returns an HTML page with a redirect to the default host.

### POST /echo

Returns the request body as the response.

### GET /hey

Returns a simple message.

## Configuration

The server is configured to bind to `127.0.0.1:8080`. You can change the binding address in the `run` method of the `Server` struct in the `server.rs` file.

The default folder location that the server servers from is named 'static' which can be changed in the servers.rs file.
