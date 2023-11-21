#!/bin/bash

# Get the absolute path of the script directory
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

if [ -e "$SCRIPT_DIR/src/Makefile" ]; then
    make -C "$SCRIPT_DIR/src"

    if [ $? -eq 0 ]; then
        # Initialize empty entries.txt if not present
        if [ ! -e "$SCRIPT_DIR/entries.txt" ]; then
            touch "$SCRIPT_DIR/./entries.txt"
        fi

        MAIN_PATH=$(readlink -f "$SCRIPT_DIR/src/main")

        # Create a symlink to the main executable in a directory in the PATH
        # Updates the existing link if present
        ln -sf "$MAIN_PATH" "/usr/local/bin/entries"

        if [ $? -eq 0 ]; then
            echo "Entries 📝 is now installed. You can now run 'entries'."
        else
            echo "Error: Failed to create symlink in /usr/local/bin/. Please check for any errors above."
        fi
    else
        echo "Error: Compilation failed. Please check for any errors above."
    fi
else
    echo "Error: Makefile not found in the src folder. Please make sure you are in the correct directory."
fi