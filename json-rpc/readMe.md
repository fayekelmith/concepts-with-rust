# JSON-RPC (Remote Procedure Call)

JSON-RPC is a lightweight remote procedure call protocol. It is transport agnostic in that the concepts can be used within the same process, over sockets, over HTTP, or in many various message passing environments. It uses JSON (RFC 4627) as data format.

JSON-RPC allows for notifications (data sent to the server that does not require a response) and for multiple calls to be sent to the server which may be answered out of order.

A great use case is in a microservices architecture where you have many services that need to communicate with each other. JSON-RPC is a great way to do this.

Another use case is in LSP implementations. LSP (Language Server Protocol) is a protocol used between an editor and a language server to provide language features like auto complete, go to definition, etc. JSON-RPC is used as the underlying protocol for LSP.

## Specification

The JSON-RPC 2.0 specification can be found [here](https://www.jsonrpc.org/specification).

## Implementations

Run the following rust code to start the server:

```bash
cargo run
```

Then run the following curl command to send a request to the server:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "say_hello","id":2}' http://localhost:3000
```

The server will respond with a `hello` message.

```bash
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "add", "params": {"a": 1, "b": 2}, "id": 3}' http://localhost:3000
```

## Notes:

- Every request must have a `jsonrpc` field with the value `2.0`.
- The `id` field is used to correlate requests and responses. If the request is a notification, the `id` field can be omitted.
- The `method` field is used to specify the method to be called.
- The `params` field is used to pass parameters to the method.
- It is recommended to use the `id` field to correlate requests and responses. If the `id` field is omitted, the request is considered a notification and the server will not send a response.
- The server must respond with a JSON object with the `jsonrpc` field set to `2.0`.
- If the request is a notification, the server must not send a response.
- If the request is a batch request, the server must respond with an array of responses in the same order as the requests.
- If the request is a batch request and one of the requests is a notification, the server must not send a response for that request.
- ALL requests are POST requests.
- The `Content-Type` header must be set to `application/json`.
