fn main()
{
	// vulkan link
	//println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
	//println!("cargo:rustc-link-lib=dylib=MoltenVK");
	//println!("cargo:rustc-link-arg=-Wl,-rpath,/opt/homebrew/lib");
	// build platforms
	println!("cargo::rustc-check-cfg=cfg(android_platform)");
	println!("cargo::rustc-check-cfg=cfg(macos_platform)");
}
