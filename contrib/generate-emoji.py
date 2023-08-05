#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
#
# SPDX-License-Identifier: MIT

import urllib.request
import json

SOURCE="https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json"

emoji_categories = dict()

with urllib.request.urlopen(SOURCE) as response:
    data = json.loads(response.read())
    for item in data:
        emoji = item["emoji"]
        aliases = ",".join(item["aliases"])
        emojistring = f"{emoji},{aliases}"
        category = item["category"]
        if category not in emoji_categories:
            emoji_categories[category] = list()
        emoji_categories[category].append(emojistring)

for index, category in enumerate(emoji_categories):
    with open(f"gemoji_{index}.txt", "w") as file:
        file.write(f"{category}\n")
        file.write("\n".join(emoji_categories[category]))
