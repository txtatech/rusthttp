# rusthttp

A very small experimental http server written in Rust with an embedded website in the server.rs file itself.

Once the server is running, you can access it through your web browser by visiting `http://127.0.0.1:8080/`

## Routes:

### GET /

Returns an HTML page with a redirect to the default host.

### POST /echo

Returns the request body as the response.

### GET /hey

Returns a simple message.

## Configuration:

The server is configured to bind to `127.0.0.1:8080`. You can change the binding address in the `run` method of the `Server` struct in the `server.rs` file.

The default folder location that the server serves from is named 'static' which can be changed in the server.rs file.

## Notes:

The server includes an embedded webpage in the root route ("/"). The `hello` function serves as the request handler for the GET request to the root path. It returns an HTTP response with the content type set to "text/html" and a body containing an HTML page.

The HTML page includes a `<meta>` tag with a refresh directive that redirects the user to the specified URL after 5 seconds. The URLs in the `<a>` tags can be modified to match the desired host and port of the server.

When a client sends a GET request to the root path, the server responds with the HTML page as the content. The client's web browser renders the received HTML and executes the meta refresh directive, redirecting the user to the specified URL after the specified delay.

This embedded webpage can be used to provide information, instructions, or links to other parts of your application or website.
