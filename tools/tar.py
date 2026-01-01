# https://realpython.com/ref/stdlib/tarfile/
import tarfile, pathlib

def create_generic_package(release):
	archive_name =
		pathlib.Path(f"wt_{release}.tar.gz")

	if not archive:
		return
	with tarfile.open(archive_name, mode="w:gz") as tar_file:
		tar_file
			.add("target/x86_64-unknown-linux-musl/release-smaller/wt")
		return archive_name
