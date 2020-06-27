# comic_splitter
Let's split up comic book pages for easier e-reading


Inspired by [Ajira-FR's comics-splitter](https://github.com/Ajira-FR/comics-splitter).


# Performance

Initial python script: ~1500ms for 2 pages
Rust single-threaded equivalent: ~540ms for same 2 pages
Rust using rayon for multi-threading: ~300ms for same 2 pages
