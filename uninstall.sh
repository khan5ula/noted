#!/bin/bash

# ANSI color codes
GREEN='\033[0;32m' # Green color
RED='\033[0;31m' # Red color
NC='\033[0m' # No color

# Get the absolute path of the script directory
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

# Check if the user has necessary permissions
if [ ! -w "/usr/local/bin/entries" ]; then
    echo -e "${RED}Error: ${NC}You don't have the necessary permissions to uninstall Entries."
    echo "Please run this script with appropriate permissions."
    exit 1
fi

# Remove the symlink in /usr/local/bin
rm "/usr/local/bin/entries"

# Remove the script directory
rm -rf "$SCRIPT_DIR"

if [ $? -eq 0 ]; then
    echo "Entries is now removed."
else
    echo -e "${RED}Error: ${NC}Something went wrong with the uninstallation script. Please check for any errors above."
    echo "You can manually remove entries by deleting the entries directory and removing the symlink by: $ rm /usr/local/bin/entries"
fi
