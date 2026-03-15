fn main() {
    println!("cargo:return-if-changed=src/platform/linux/libwayland.c");
    println!("cargo:rustc-link-lib=wayland-client");
    let mut targets = Vec::new();

    // target -> wayland
    #[cfg(all(target_os = "linux"))]
    {
        targets.push(("request_wl_surface", "src/platform/linux/libwayland.c"));
        targets.push(("request_wl_disconnect", "src/platform/linux/libwayland.c"));
        targets.push(("loop_wl_event", "src/platform/linux/libwayland.c"));
    }

    for (name, file) in targets {
        cc::Build::new()
        .file(file)
        .compile(name);
    }
}
