#!/bin/bash

# Get the current directory
CURRENT_DIR=$(pwd)

# Run the Makefile in the current directory
make

# Create a bash/zsh alias for the 'entries' command
echo "alias entries='$CURRENT_DIR/entries/src/./main'" >> ~/.bashrc  # for Bash
echo "alias entries='$CURRENT_DIR/entries/src/./main'" >> ~/.zshrc   # for Zsh

# Source the appropriate shell profile to make the alias immediately available
source ~/.bashrc  # for Bash
source ~/.zshrc   # for Zsh

echo "Installation complete. You can now use the 'entries' command."
