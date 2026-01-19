fn main() {
	let mut targets = Vec::new();

	#[cfg(all(target_os = "macos", target_aarch="aarch64"))]
	targets.push(("exit", "src/core/macos/exit.s"));

	#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
	targets.push(("exit", "src/core/linux/exit_a64.s"));

	#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
	targets.push(("exit", "src/core/linux/exit_x64.s"));

	targets.push(("create_socket", "src/core/unix/socket.c"));
	targets.push(("read_socket", "src/core/unix/socket.c"));
	targets.push(("write_socket", "src/core/unix/socket.c"));
	targets.push(("close_socket", "src/core/unix/socket.c"));

	for (name, file) in targets {
		cc::Build::new()
			.file(file)
			.compile(name);
	}
}
