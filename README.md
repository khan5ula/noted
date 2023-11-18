Entries 📝 is a small command line utility for creating and reading quick notes. I wrote Entries because I like to take notes whenever I make system configurations. If (when) something goes wrong, it's easier to rollback when I have record of what I've done.

Entries takes one argument: `new`, `all` or `clear`.

Create a new entry with `new`:

```zsh
➜  ~ entries new
New entry:
I touched a dangerous looking setting. Things could go terribly wrong. 
```

Read all entries with `all`:

```zsh
➜  ~ entries all 
--- Sat Nov 18 14:48:10 2023 ---
I touched a dangerous looking setting. Things could go terribly wrong.

--- Sat Nov 18 14:49:12 2023 ---
Things went better than expected.
```

Clear entries with `clear`:

```zsh
➜  ~ entries clear            
Are you sure you want to clear all entries? [y/N] > y
Done.
```

Entries is written in C.