fn main()
{
	// build platforms
	println!("cargo::rustc-check-cfg=cfg(android_platform)");
	println!("cargo::rustc-check-cfg=cfg(macos_platform)");
	println!("cargo::rustc-check-cfg=cfg(linux_platform)");
}
