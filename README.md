# JDict

This is a japanese dictionary application I wrote, because I couldn't find a dictionary that was
1. Fast
2. Has a good UX (esp. for my use case)

I often find myself using japanese dictionaries like this:
1. Look up a word
2. Look up one of the kanji of that word
3. Look up in which words this kanji appears

And in this dictionary each of these steps is at most one click.

## Applications / Binaries
- Apps
  - `code/jdict-server`: Server hosting the dictionary as a web app
  - `code/jdict-tauri`: Standalone version of the web app (does not need a server)
  - `code/jdict-egui`: A minimal, self-contained version based on egui.
- Libraries
  - `code/jdict-shared`: Shared code between all of the dictionary versions
  - `code/web`: The code for the web frontend written in (Vue.js)

## Data sources
This dictionary uses the following data sources:
- [JMdict](http://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project) (particularily [JMdict_e.gz](http://ftp.edrdg.org/pub/Nihongo/JMdict_e.gz)) for word meanings
- [KANJIDIC2](http://www.edrdg.org/wiki/index.php/KANJIDIC_Project) for kanji information (meanings, num strokes etc.)
- [KanjiVG](https://kanjivg.tagaini.net/) for the stroke order animations and kanji decomposition

In addition we use Google's Noto Sans CJK Japanese for the egui version
