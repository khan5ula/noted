Entries 📝 is a small command line utility for creating and reading quick notes. I wrote Entries because I like to take notes whenever I make system configurations. If (when) something goes wrong, it's easier to rollback when I have record of what I've done.

Entries is written in C.

## Instructions

Entries takes one argument: `new`, `all` or `clear`.

Create a new entry with `new`:

```zsh
$ entries new
Post a new entry:
$ I wanted to tweak this dangerous looking setting. Things might blow up...
Entry saved 📝
```

Read all entries with `all`:

```zsh
$ entries all
--- Sat Nov 18 22:35:22 2023 ---
I wanted to tweak this dangerous looking setting. Things might blow up...

--- Sat Nov 18 22:36:29 2023 ---
Looks like everything is OK after all.
```

Clear entries with `clear`:

```zsh
$ entries clear            
Are you sure you want to clear all entries? [y/N] > $ y
Done
```

## Install Entries

1. Download the source code and unzip it to the desired location
2. Open the entries directory in terminal
3. Give the installation script execute permissions: `chmod +x install.sh`
4. Run the installation script: `./install.sh`
5. Entries should now work. Try entering `entries` to terminal

Note! Because Entries uses `symlink` to be accessible from all directories with terminal, ***Entries should be installed to the same drive with your root directory.***

## Delete Entries
1. Delete the entries directory
2. Delete the symlink: `rm /usr/local/bin/entries`
3. Entries is now removed