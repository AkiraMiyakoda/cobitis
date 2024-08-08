# Copyright © 2024 Akira Miyakoda
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

import datetime
import os
import re
import sys
import time

# サービスワーカーに動的なデータを埋め込むスクリプト

EXCLUDES = ["index.html", "sw.js", "robots.txt", "_worker.js"]

build_date = datetime.datetime.now().astimezone().replace(microsecond=0).isoformat()
cache_version = f'"{str(int(time.time() * 1000))}"'

cache_files = ["/", "https://unpkg.com/mvp.css"]
for dir, _, files in os.walk("./dist"):
    for file in files:
        if file in EXCLUDES:
            continue

        cache_files.append(re.sub(r"^\./dist/", "", f"{dir}/{file}"))

cache_files = ", ".join((f'"{file}"' for file in cache_files))

print(f"Modifying ./dist/sw.js", file=sys.stderr)
print(f"__BUILD_DATE__ = {build_date}", file=sys.stderr)
print(f"__CACHE_VERSION__ = {cache_version}", file=sys.stderr)
print(f"__CACHE_FILES__ = {cache_files}", file=sys.stderr)

lines = []
with open("./dist/sw.js", "r") as file:
    for line in file.readlines():
        line = line.replace("__BUILD_DATE__", build_date)
        line = line.replace("__CACHE_VERSION__", cache_version)
        line = line.replace("__CACHE_FILES__", cache_files)
        lines.append(line)

with open("./dist/sw.js", "w") as file:
    print("".join(lines), file=file)
