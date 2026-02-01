#![allow(missing_docs)]
use std::env;

fn main() {
	let target = env::var("TARGET").expect("TARGET não definido");
	let mut build = cc::Build::new();

	match target.as_str() {
		"aarch64-apple-darwin" => {
			let _ = build.file("src/core/macos/exit.s");
		}
		"aarch64-unknown-linux-gnu" | "aarch64-unknown-linux-musl" => {
			let _ = build.file("src/core/linux/exit_a64.s");
		}
		"x86_64-unknown-linux-gnu" | "x86_64-unknown-linux-musl" => {
			let _ = build.file("src/core/linux/exit_x64.s");
		}
		_ => {
			panic!("target não suportado para exit syscall: {target}");
		}
	}

	let _ = build
		.file("src/core/unix/socket.c")
		.file("src/core/unix/thread.c")
		.file("src/core/unix/getenv.c");

	build.compile("dirty-core");
}
