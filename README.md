Entries 📝 is a small command line utility for creating and reading quick notes. I wrote Entries because I like to take notes whenever I make system configurations. If (when) something goes wrong, it's easier to rollback when I have record of what I've done.

Entries takes one argument: `new`, `all` or `clear`.

Create a new entry with `new`:

```zsh
➜  ~ entries new
Post a new entry:
I wanted to tweak this dangerous looking setting. Things might blow up...
Entry saved 📝
 
```

Read all entries with `all`:

```zsh
➜  ~ entries all
--- Sat Nov 18 22:35:22 2023 ---
I wanted to tweak this dangerous looking setting. Things might blow up...

--- Sat Nov 18 22:36:29 2023 ---
Looks like everything is OK after all.
```

Clear entries with `clear`:

```zsh
➜  ~ entries clear            
Are you sure you want to clear all entries? [y/N] > y
Done
```

Entries is written in C.