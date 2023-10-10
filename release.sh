#!/bin/bash

set -e
# Ask for RELEASE_NAME
read -p "Enter RELEASE_NAME: " RELEASE_NAME

# Create releases directory if it doesn't exist
mkdir -p "releases/${RELEASE_NAME}"

# Read targets from .targets file and build
if [ -f .targets ]; then
    cat .targets | xargs -I {} cargo zigbuild --target {} --release -Z unstable-options --out-dir="./releases/${RELEASE_NAME}/{}" || true
    echo "Build completed. Output files are in releases/${RELEASE_NAME}/ directory."

    # Create tar.gz archive excluding files specified in .gitignore
    git archive --format=tar.gz --output="releases/${RELEASE_NAME}/src.tar.gz" HEAD

    # Create zip archive excluding files specified in .gitignore
    git archive --format=zip --output="releases/${RELEASE_NAME}/src.zip" HEAD

    echo "Tar.gz and Zip archives created in releases/${RELEASE_NAME}/ directory."
else
    echo ".targets file not found."
fi
