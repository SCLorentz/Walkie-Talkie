# https://dmglib.readthedocs.io/en/latest/module_doc.html
import dmgbuild, pathlib

def create_dmg(release):
	path = r'wt.app/MacOS'
	if not os.path.exists(path):
		os.makedirs(path)
	with dmg.attachedDiskImage(f'wt_{release}.dmg',
								keyphrase='sample') as mount_points:
		print(mount_points)
