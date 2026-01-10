fn main()
{
	// build platforms
	println!("cargo::rustc-check-cfg=cfg(android_platform)");
	println!("cargo::rustc-check-cfg=cfg(macos_platform)");
	println!("cargo::rustc-check-cfg=cfg(linux_platform)");
	//
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rustc-link-arg=-Wl,--gc-sections");
	println!("cargo:rustc-link-arg=-Wl,--strip-all");
	//
	#[cfg(target_os = "macos")]
	{
		println!("cargo:rerun-if-changed=linker/a64.ld");
		println!("cargo:rustc-link-arg=-Tlinker/a64.ld");
	}
}
