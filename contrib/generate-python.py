#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
#
# SPDX-License-Identifier: MIT

import sys
import keyword

res = set()

for word in sys.stdlib_module_names:
    if not word.startswith("_"):
        res.add(word)

res = res | set(keyword.kwlist)

with open("python_keywords_stdlib.txt", "w") as file:
    file.write("Python keywords & stdlib\n")
    file.write("\n".join(sorted(res)))
