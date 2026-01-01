import subprocess

print("building project...")
subprocess.run('cargo build --profile release-smaller', shell=True)
subprocess.run('strip target/release-smaller/wt', shell=True)
print("packaging project...")

# create packaging system:
# https://developer.apple.com/documentation/xcode/building-and-running-an-app
# macos: walkie_talkie.app
# https://nix.dev/tutorials/packaging-existing-software.html
# linux: walkie_talkie.deb, walkie_talkie.appimage, walkie_talkie.nix
# windows: wt_installer.exe
