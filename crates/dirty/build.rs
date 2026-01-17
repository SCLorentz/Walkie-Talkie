use std::{env, path::PathBuf};

fn main() {
	let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let target = env::var("TARGET").unwrap();

	let mut targets = Vec::new();

	if target.contains("aarch64") && target.contains("apple-darwin") {
		targets.push(("exit", root.join("src/core/macos/exit.s")));
	}

	if target.contains("aarch64") && target.contains("linux") {
		targets.push(("exit", root.join("src/core/linux/exit_a64.s")));
	}

	if target.contains("x86_64") && target.contains("linux") {
		targets.push(("exit", root.join("src/core/linux/exit_x64.s")));
	}

	targets.push(("create_socket", root.join("src/core/unix/socket.c")));

	for (name, file) in targets {
		cc::Build::new()
			.file(file)
			.compile(name);
	}
}
