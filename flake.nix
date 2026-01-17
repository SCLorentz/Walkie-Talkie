{
	description = "walkie-talkie cross musl shell";

	inputs.nixpkgs.url = "github:NixOS/nixpkgs";

	outputs = { self, nixpkgs }:
	let
		system = "aarch64-darwin";
		pkgs = import nixpkgs { inherit system; };
		cross = pkgs.pkgsCross.musl64;
		toolchain = fenix.packages.${system}.latest.toolchain;
	in {
		devShells.${system}.default = pkgs.mkShell {
			nativeBuildInputs = [
				pkgs.zig
				cross.stdenv.cc
				cross.pkg-config
				pkgs.python312Packages.toml
				toolchain
			];

			shellHook = ''
				export CC_x86_64_unknown_linux_musl=${cross.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc
				export CXX_x86_64_unknown_linux_musl=${cross.stdenv.cc}/bin/x86_64-unknown-linux-musl-g++
				export AR_x86_64_unknown_linux_musl=${cross.stdenv.cc}/bin/x86_64-unknown-linux-musl-ar
				export PKG_CONFIG_x86_64_unknown_linux_musl=${cross.pkg-config}/bin/pkg-config
			'';
		};
	};
}
