# Noted

A (mostly) terminal based application for creating, reading and managing quick notes. Hit `noted n` and store your note quickly and get back to it whenever you want. Alternatively, provide the note with a YAD-based graphical editor using the `-g` option.

## Usage

| Command                    | Option       | Description                                                                                |
| -------------------------- | ------------ | ------------------------------------------------------------------------------------------ |
| **New (n)**                |              | Creates a new note from the command line arguments.<br>Eg. `noted n buy more coffee soon!` |
| **New (n)**                | `--file -f`  | Creates a new note from a given text file                                                  |
| **New (n)**                | `--gui -g`   | Creates a new note with a YAD-Based graphical editor.<br>Required for multi-line notes.    |
| **All (a)**                |              | View all notes                                                                             |
| **Last (l)**               |              | View the newest note                                                                       |
| **Last (l)**               | `$NUMBER`    | View `$NUMBER` of newest notes                                                             |
| **First (f)**              |              | View the oldest note                                                                       |
| **First (f)**              | `$NUMBER`    | View `$NUMBER` of oldest notes                                                             |
| **Delete (d, rm, remove)** | `$ID`        | Remove notes with a matching ID or start of an ID                                          |
| **Delete (d, rm, remove)** | `--all` `-a` | Remove all notes                                                                           |
| **Search (s)**             | `$STRING`    | View notes that contain the given `$STRING`                                                |
| **Edit (e)**               | `$ID`        | Edit note with the given ID. Requires GUI.                                                 |

## Dependencies

- yad
- gtk3
- webkit2gtk-4.1
- gtksourceview3
- gspell
