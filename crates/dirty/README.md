# Dirty

this is hell

## Want to use a String?

here:

```rust
let foo = String::from("my string");
bar(foo.as_str());
```

## Want to use void ptr?

here:

```rust
let foo = void::to_handle(core::ptr::null_mut());
```

## Want to print a value?

here:

```rust
write!("Hello World");
```

## Exit the program?

here:

```rust
exit(1);
```

## Unix Sockets

sockets are useful to comunicate with other process within the same UNIX based OS

```rust
let socket = dirty::Socket::new();
socket.write_socket(b"hello socket");

let buffer: &[u8] = &[];
match socket.read_socket(buffer) {
	Some(result) => debug!("{:?}", result),
	None => warn!("no message recived"),
};
socket.close_socket();
```
