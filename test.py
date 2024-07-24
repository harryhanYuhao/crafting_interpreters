import glob
import subprocess


for filename in glob.iglob("./test/" + '**/*.lox', recursive=True): 
    if "error" in filename:
        continue
    if subprocess.run(["cargo", "run", filename]).returncode == 1 : 
        print(filename + " error!")
        break

