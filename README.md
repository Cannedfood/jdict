# JDict

This is a japanese dictionary application I wrote, because I couldn't find a dictionary that was
1. Fast
2. Has a good UX (esp. for my use case)

My usual steps are:
1. Look up a word
2. Look up one of the kanji of that word
3. Look up in which words this kanji appears

And this dictionary is optimized for that.

## Data sources
This dictionary uses the following data sources UNDER THEIR RESPECTIVE LICENSES:
- [JMdict](http://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project) (particularily [JMdict_e.gz](http://ftp.edrdg.org/pub/Nihongo/JMdict_e.gz)) for word meanings
- [KANJIDIC2](http://www.edrdg.org/wiki/index.php/KANJIDIC_Project) for kanji information (meanings, num strokes etc.)
- [KanjiVG](https://kanjivg.tagaini.net/) for the stroke order animations and kanji decomposition
