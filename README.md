Entries 📝 is a small command line utility for creating and reading quick notes. I wrote Entries because I like to take notes whenever I make system configurations. If (when) something goes wrong, it's easier to rollback when I have record of what I've done.

Entries takes one argument, either `write` or `read`.

Create a new entry with `write`:

```zsh
➜  ~ entries write
New entry:
I touched a dangerous looking setting. Things could go terribly wrong. 
```

Read all entries with `read`:

```zsh
➜  ~ entries read 
--- Sat Nov 18 14:48:10 2023 ---
I touched a dangerous looking setting. Things could go terribly wrong.

--- Sat Nov 18 14:49:12 2023 ---
Things went better than expected.
```

Entries is written in C.