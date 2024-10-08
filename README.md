# Noted

A terminal based application for creating, managing and reading quick notes.

## Usage

| Command                    | Option      | Description                                                                                |
| -------------------------- | ----------- | ------------------------------------------------------------------------------------------ |
| **New (n)**                |             | Creates a new note from the command line arguments.<br>Eg. `noted n buy more coffee soon!` |
| **New (n)**                | `--file -f` | Creates a new note from a given text file                                                  |
| **New (n)**                | `--gui -g`  | Creates a new note with a YAD-Based graphical editor.<br>Required for multi-line notes.    |
| **All (a)**                |             | View all notes                                                                             |
| **Last (l)**               |             | View the newest note                                                                       |
| **Last (l)**               | `$NUMBER`   | View `$NUMBER` of newest notes                                                             |
| **First (f)**              |             | View the oldest note                                                                       |
| **First (f)**              | `$NUMBER`   | View `$NUMBER` of oldest notes                                                             |
| **Delete (d, rm, remove)** | `$ID`       | Remove notes with a matching ID or start of an ID                                          |
| **Search (s)**             | `$STRING`   | View notes that contain the given `$STRING`                                                |

## Dependencies

- yad
- gtk3
- webkit2gtk-4.1
- gtksourceview3
- gspell
