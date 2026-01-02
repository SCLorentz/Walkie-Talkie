import subprocess, os, platform, sys

version = "0.0.1"

def build():
	print("building project...")
	match system.upper():
		case "LINUX":
			path = "target/x86_64-unknown-linux-musl/release-smaller/wt"
			subprocess.run('cargo build-linux-x64', shell=True, capture_output=True)
			subprocess.run(f'upx -9 {path}', shell=True, capture_output=True)
		case "DARWIN":
			path = "target/aarch64-apple-darwin/release-smaller/wt"
			subprocess.run('cargo build-macos', shell=True, capture_output=True)
			subprocess.run(f'strip {path}', shell=True, capture_output=True)
	if os.path.exists(path):
		print("Done")

def package():
	print("packaging project...")
	match system.upper():
		case "LINUX":
			from tar import create_generic_package
			archive = create_generic_package(version)

			if archive:
				print(f"packaged into: {archive}")
			else:
				print("error packaging")
		case "Darwin":
			from apple import create_dmg
			dmg = create_dmg(version)

if len(sys.argv) == 1:
	system = platform.system()
	print(f"system not defined, fallback to {system}")
else:
	system = sys.argv[1]

build()
package()

# create packaging system:
# https://developer.apple.com/documentation/BundleResources/placing-content-in-a-bundle
# macos: walkie_talkie.app
# https://nix.dev/tutorials/packaging-existing-software.html
# linux: walkie_talkie.deb, walkie_talkie.appimage, walkie_talkie.nix
# windows: wt_installer.exe
