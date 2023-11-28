# Entries
Entries is a small command line utility for creating and reading quick notes. Entries allows you to write a quick terminal note without having to worry about storing it. Fire and forget... Until you want to read it. Entries will keep all your notes in a file and allows you to quickly read them.

Entries is designed for creating single-line notes with a maximum length of 1000 characters. Keep in mind that entries are confirmed by pressing `enter`, so entries with newlines are not supported.

Entries is especially nice with drop-down terminals such as [yakuake](https://apps.kde.org/yakuake/) or [ddterm](https://github.com/ddterm/gnome-shell-extension-ddterm).
<br /><br />

# Instructions


| Argument         | Short version     | Modifier | Description |
|------------------|-------------------|----------|-------------|
| new              | n                 |          | Post a new entry
| all              | a                 |          | Read all entries
| first            | f                 | number   | Read the first entry, or number of entries from the start
| last             | l                 | number   | Read the last entry, or number of entries from the end
| clear            | rm                |          | Delete all entries

<br />

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

Read the first entry with `first`:
```zsh
$ entries first
--- Tue Nov 21 15:06:45 2023 ---
Something I've posted a while back...
```

Read the two latest entries with `last 2`:

```zsh
$ entries last 2
--- Tue Nov 28 10:36:40 2023 ---
Buy tomato sauce

--- Tue Nov 28 11:04:57 2023 ---
Make that two cans actually
```

Nuke all entries with `clear`:

```zsh
$ entries clear            
Are you sure you want to clear all entries? [y/N] > $ y
Done
```

<br />

# Installation

1. Download the source and unzip it to the desired location
2. Open the entries directory in terminal
3. Run the script:
```bash
# First installation method. Give permissions and execute:
chmod +x ./install.sh
./install.sh
```

```bash
# Second installation method. Execute directly:
bash install.sh
```

Note! Because Entries uses `soft symlink` to be accessible from all directories with terminal, ***run the installation script again if you move the source directory.***

Running the installation script again will not delete your entries.

<br />

# How to delete

Open the installation directory in terminal and run the uninstallation script:

```bash
# First method. Give permissions and execute:
chmod +x ./uninstall.sh
./uninstall.sh
```

```bash
# Second method. Execute directly:
bash uninstall.sh
```

If there's a problem with the uninstall script, manual uninstall is simple:

1. Delete the entries directory
2. Delete the symlink: `rm /usr/local/bin/entries`
3. Entries is now removed