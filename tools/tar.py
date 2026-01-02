# https://realpython.com/ref/stdlib/tarfile/
import tarfile, pathlib, shutil, os

def create_generic_package(release):
	archive_name = pathlib.Path(f"wt_{release}.tar.gz")
	shutil.copyfile("target/x86_64-unknown-linux-musl/release-smaller/wt", "wt")

	with tarfile.open(archive_name, mode="w:gz") as tar_file:
		# for some reason recursive=False does not change a thing
		tar_file.add("wt")
		os.remove("wt")
		return archive_name
