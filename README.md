Entries is a small command line utility for creating and reading quick notes. Entries allows you to write a quick terminal note without having to worry about storing it. Fire and forget... Until you want to read it with `entries r` Entries will keep all your notes in a file and allows you to quickly read them.

Entries is for writing single-line notes that are no longer than 1000 characters.

Entries is especially nice with drop-down terminals such as [yakuake](https://apps.kde.org/yakuake/) or [ddterm](https://github.com/ddterm/gnome-shell-extension-ddterm).

## Instructions

Available arguments:

- new (n)
- all (a)
- first (f)
- first COUNT (f COUNT)
- last (l)
- last COUNT (l COUNT)
- clear (rm)

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

Note! Because Entries uses `soft symlink` to be accessible from all directories with terminal, ***run the installation script again if you move the source directory.***

## Delete Entries
1. Delete the entries directory
2. Delete the symlink: `rm /usr/local/bin/entries`
3. Entries is now removed