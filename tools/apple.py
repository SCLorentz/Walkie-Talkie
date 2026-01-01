# https://dmglib.readthedocs.io/en/latest/module_doc.html
import dmgbuild, pathlib

def create_apple_package(release):
	with dmg.attachedDiskImage(f'wt_{release}.dmg',
								keyphrase='sample') as mount_points:
		print(mount_points)
