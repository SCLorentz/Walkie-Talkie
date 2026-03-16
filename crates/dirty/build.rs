#![allow(missing_docs, clippy::panic, clippy::expect_used)]
use std::env;

fn main() {
	let Ok(target) = env::var("TARGET") else { panic!("TARGET not defined!") };
	let mut build = cc::Build::new();

	#[cfg(all(target_os = "macos", target_aarch="aarch64"))]
	targets.push(("exit", "src/core/macos/exit.s"));

	#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
	targets.push(("exit", "src/core/linux/exit_a64.s"));

	#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
	targets.push(("exit", "src/core/linux/exit_x64.s"));

	targets.push(("create_thread", "src/core/unix/thread.c"));
	targets.push(("kill_thread", "src/core/unix/thread.c"));

	for (name, file) in targets {
		cc::Build::new()
			.file(file)
			.compile(name);
	}

	let _ = build
		.file("src/core/unix/socket.c")
		.file("src/core/unix/thread.c")
		.file("src/core/unix/getenv.c");

	build.compile("dirty-core");
}
