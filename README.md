Basic Netcat client /server implementation in Rust
==================================================

Implementation Rust to write a **netcat** client and server using the standard library only. You can use it to connect to a server andsend and receive data. It is similar to PuTTY and telnet.

1.  The client will allow user to type messages and send it to any TCP socket server, and display the response from the server, in an endless loop.
    
2.  The server will listen for incoming TCP connections from clients, and display the message from the client, and send a response back to the client.
    
3.  The server manipulate the color of the client message before sending it back to the client.
    

Configure clap to parse command line arguments
----------------------------------------------

This Rust app has a single binary, and depending on the command line arguments, it will behave either as a client or server. We will use the clap crate to parse the command line arguments.

We will configure clap so that the following commands will work:

```rust
cargo run server
cargo run client
```

We want to allow the user to specify the following options and choose their own \`address and port\`. If the user does not specify any options, we will use the default values. The default value for —_address is 127.0.0.1,_ and the default value for _--port is 3000_.

```rust
cargo run server --address 127.0.0.1 --port 8080
cargo run server --address 127.0.0.1
cargo run server --port 8080

cargo run client --address 127.0.0.1 --port 8080
cargo run client --address 127.0.0.1
cargo run client --port 8080
```