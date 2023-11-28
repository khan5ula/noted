#!/bin/bash

# Get the absolute path of the script directory
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

# Check if the user has necessary permissions
if [ ! -w "/usr/local/bin/entries" ]; then
    echo "Error: You don't have the necessary permissions to uninstall Entries."
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
    echo "Error: Something went wrong with the uninstallation script. Please check for any errors above."
    echo "You can manually remove entries by deleting the entries directory and removing the symlink by: $ rm /usr/local/bin/entries"
fi
