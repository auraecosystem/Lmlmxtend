from pathlib import Path
p = Path("/mnt/data/LMLM_File_Intelligence/LmlmFileManager.py")
s = p.read_text(encoding="utf-8")
s = s.replace(
    '    parent = Path.home() / "Documents" / "LMLM-FileIntelligence"\n    label = root.name or "selected-folder"\n',
    '    parent = Path.home() / "Documents" / "LMLM-FileIntelligence"\n    parent.mkdir(parents=True, exist_ok=True)\n    label = root.name or "selected-folder"\n'
)
p.write_text(s, encoding="utf-8")

a = Path("/mnt/data/LMLM_File_Intelligence/LmlmFileManager.applescript")
ap = a.read_text(encoding="utf-8")
ap = ap.replace(
    '        display dialog resultText buttons {"Open Reports Folder", "OK"} default button "OK" with title "LMLM File Intelligence"\n        if button returned of result is "Open Reports Folder" then\n',
    '        set dialogResult to display dialog resultText buttons {"Open Reports Folder", "OK"} default button "OK" with title "LMLM File Intelligence"\n        if button returned of dialogResult is "Open Reports Folder" then\n'
)
a.write_text(ap, encoding="utf-8")

import py_compile, zipfile
py_compile.compile(str(p), doraise=True)
zip_path = Path("/mnt/data/LMLM_File_Intelligence.zip")
with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as z:
    for name in ("LmlmFileManager.py", "LmlmFileManager.applescript", "README.md"):
        z.write(Path("/mnt/data/LMLM_File_Intelligence") / name, arcname=f"LMLM_File_Intelligence/{name}")
print("Fixed report-directory creation and AppleScript dialog handling.")
print("Python syntax check: passed; package rebuilt.")
