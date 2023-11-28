#!/bin/bash

# ANSI color codes
GREEN='\033[0;32m' # Green color
RED='\033[0;31m' # Red color
NC='\033[0m' # No color

# Get the absolute path of the script directory
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

if [ -e "$SCRIPT_DIR/src/Makefile" ]; then
    echo "Makefile found"
    make -C "$SCRIPT_DIR/src"

    if [ $? -eq 0 ]; then
        # Initialize empty entries.txt if not present
        if [ ! -e "$SCRIPT_DIR/entries.txt" ]; then
            "Initializing a new file for entries"
            touch "$SCRIPT_DIR/./entries.txt"
        fi

        MAIN_PATH=$(readlink -f "$SCRIPT_DIR/src/main")

        # Create a symlink to the main executable in a directory in the PATH
        # Updates the existing link if present
        ln -sf "$MAIN_PATH" "/usr/local/bin/entries"
        echo "Creating a symlink for entries"

        if [ $? -eq 0 ]; then
            echo -e "${GREEN}Install successful 🔥${NC}"
            echo "If you decide to move the entries directory, run this installation script again"
            echo "Running this script again will not delete your old entries"
            echo
            echo -e "Get started by writing ${GREEN}entries${NC}"
        else
            echo -e "${RED}Error: ${NC}Failed to create symlink in /usr/local/bin/. Please check for any errors above."
        fi
    else
        echo -e "${RED}Error: ${NC}Compilation failed. Please check for any errors above."
    fi
else
    echo -e "${RED}Error: ${NC}Makefile not found in the src folder. Please make sure you are in the entries directory."
fi
