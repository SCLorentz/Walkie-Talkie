fn main() {
	#[cfg(target_os = "linux")] {
		println!("cargo:return-if-changed=src/platform/linux/libwayland.c");
		println!("cargo:return-if-changed=src/platform/linux/xdg-shell-protocol.c");
		println!("cargo:rustc-link-lib=wayland-client");
	}

    #[cfg(target_os = "linux")]
    cc::Build::new()
        .file("src/platform/linux/libwayland.c")
        .compile("libwayland");

    #[cfg(target_os = "linux")]
    cc::Build::new()
        .file("src/platform/linux/xdg-shell-protocol.c")
        .compile("xdg-shell-protocol");
}
