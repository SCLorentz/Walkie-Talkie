import subprocess, os, platform, sys, toml, time

with open('Cargo.toml', 'r') as f:
	config = toml.load(f)
	version = config['workspace.package']['version']
	print(f"current version: {version}")

def build():
	print("building project...")
	start = time.time()
	match system:
		case "LINUX":
			path = "target/x86_64-unknown-linux-musl/release-smaller/wt"
			subprocess.run('cargo +nightly build-linux-x64', shell=True, capture_output=True)
			subprocess.run(f'upx -9 {path}', shell=True, capture_output=True)
		case "DARWIN":
			path = "target/aarch64-apple-darwin/release-smaller/wt"
			subprocess.run('cargo +nightly build-macos', shell=True, capture_output=True)
			subprocess.run(f'strip {path}', shell=True, capture_output=True)
		case _:
			print(f"{system} not supported or not configured")
			sys.exit(1)
	end = time.time()
	if os.path.exists(path):
		print(f"Done ({end - start:.4f}s)")

def package():
	print("packaging project...")
	start = time.time()
	match system:
		case "LINUX":
			from tar import create_generic_package
			package = create_generic_package(version)
		case "DARWIN":
			print("todo")
			sys.exit(1)
			#from apple import create_dmg
			#dmg = create_dmg(version)
		case _:
			print(f"{system} not supported or not configured")
			sys.exit(1)
	end = time.time()
	if package:
		print(f"Done ({end - start:.4f}s)")
		print(f"Packaged into: {package}")
	else:
		print("error packaging")
		sys.exit(1)

def cleanup():
	r = input("Do you wish to clean the target directory? y/N\t")
	if r.lower() == "y":
		start = time.time()
		subprocess.run('cargo +nightly clean', shell=True, capture_output=True)
		end = time.time()
		print(f"Done ({end - start:.4f}s)")
	else:
		print("skipping...")
		print("Done")

if len(sys.argv) == 1:
	system = platform.system().upper()
else:
	system = sys.argv[1].upper()

print(f"packaging to {system}")
subprocess.run('cargo +nightly check', shell=True)
build()
package()
cleanup()


# create packaging system:
# https://developer.apple.com/documentation/BundleResources/placing-content-in-a-bundle
# macos: walkie_talkie.app
# https://nix.dev/tutorials/packaging-existing-software.html
# linux: walkie_talkie.deb, walkie_talkie.appimage, walkie_talkie.nix
# windows: wt_installer.exe
