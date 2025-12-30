{
	description = "development environment";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
			in {
				devShells.default = pkgs.mkShell {
					name = "rust-packaging-shell";

					packages = with pkgs; [
						cargo
						rustc
						python3
					];

					shellHook = ''
						cargo install cargo-alias-exec
					'';
				};
			}
		);
}
