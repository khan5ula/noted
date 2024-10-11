#!/bin/bash

DIRECTORY="$HOME/.local/share/noted"

if [ ! -d "$DIRECTORY" ]; then
  mkdir -p "$DIRECTORY"
fi
