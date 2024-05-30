#!/bin/bash

# Iterate over each first level subdirectory
for dir in */; do
    # Remove .git directory if exists
    if [ -d "${dir}.git" ]; then
        echo "Removing .git directory from ${dir}"
        rm -rf "${dir}.git"
    fi

    # Remove .gitignore file if exists
    if [ -f "${dir}.gitignore" ]; then
        echo "Removing .gitignore file from ${dir}"
        rm "${dir}.gitignore"
    fi
done
