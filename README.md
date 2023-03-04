# JDict

This is a japanese dictionary application I wrote, because I couldn't find a dictionary that was
1. Fast
2. Has a good UX (esp. for my use case)

I often find myself using japanese dictionaries like this:
1. Look up a word
2. Look up one of the kanji of that word
3. Look up in which words this kanji appears

And in this dictionary each of these steps is at most one click and extremely responsive.

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

## Building
Prerequisites / required programs:
- rust, cargo and rustup
- NodeJS, NPM (jdict-server and jdict-tauri only)

Building
- `jdict-tauri`:
  ```bash
  (cd code/web && npm run build) # Build frontend

  # Set up rust
  rustup default nightly
  cargo install cargo-tauri

  # Tauri
  cargo tauri build
  ```

- `jdict-server`:
  ```bash
  (cd code/web && npm run build) # Build frontend

  # Set up rust
  rustup default nightly
  cargo build --release --bin jdict-server
  # Build Output: target/release/jdict-server
  ```

- `jdict-egui`
  ```bash
  rustup default nightly
  cargo build --release --bin jdict-egui
  # Build Output: target/release/jdict-egui
  ```

## TODO:
- `jdict-egui`:
  - [ ] Highlight search term
  - [ ] Kanji decomposition
- `jdict-server`, `jdict-tauri`
  - [ ] Better Kanji Input
    - [ ] Use radicals from other kanji
    - [ ] Make the kanji search better
  - [ ] Useful auto-complete
