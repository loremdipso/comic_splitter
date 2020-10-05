# comic_splitter
Let's split up comic book pages for easier e-reading.


Inspired by [Ajira-FR's comics-splitter](https://github.com/Ajira-FR/comics-splitter).


# Performance

### For two pages:
|Script|Rough Time|
|--------------------------------------|--------|
| Original python version              | 1500ms |
| Rust single-threaded                 | 540ms  |
| Rust using rayon for multi-threading | 300ms  |
