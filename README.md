<!--
SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
SPDX-License-Identifier: MIT
-->
<div align="center" markdown="1">

![typelerate](https://raw.githubusercontent.com/b1rger/typelerate/main/data/logo.svg)

</div>

`typelerate` is a commandline game. It was inspired by the great
[typespeed](https://typespeed.sourceforge.net/). The idea of the game is to
either type words that fly across the screen before they reach the other side
*or* to guess and type the words that hide *behind* the words (or symbols) that
fly across the screen.

<div align="center" markdown="1">

![screenshot 1](https://raw.githubusercontent.com/b1rger/typelerate/main/data/screenshot_1.png)

</div>

# Wordfiles

The `wordfiles` directory contains a list of wordfiles:

* `python_keywords_stdlib.txt`: a list of Python keywords and terms from the
  Python standard library. This is to exercise typing with typical Python
  terms.

* `gemoji-[0-8].txt`: [Github Emojis](https://github.com/github/gemoji) split
  up in categories. Every line consists of one emoji and the list of shortcodes
  as answer. This way one can exercise Github Emoji Shortcodes - or using the
  `Flags` file one can have *fun with flags*!

The wordfile format is a combination of the
[typespeed](https://typespeed.sourceforge.net/) wordfile format and CSV: The
file is a UTF-8 textfile, the first line is the title of the wordfile. The
remaining lines of the file are comma separated values, with the first value
being the word that is flying across the screen and the remaining values being
the possible answers. If there is only one value in a line, it is what has to
be typed itself.

So the format can look like this:
```
This is the title
Just type me
Whats the answer for everything?,42
Whats that name of that beatles guy?,john,paul,george,ringo
```

## Wordfilegenerators

There are scripts in `contrib` to generate wordfiles:

* `generate-python.py` writes a list of Python keywords and terms from the
  Python stdlib to `python_keywords_stdlib.txt`.

* `generate-emoji.py` takes the list of [Github
  Emojis](https://github.com/github/gemoji) and for every category creates a
  file with one emoji per line and the list of shortcodes as possible answers.
