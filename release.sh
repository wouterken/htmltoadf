#!/bin/bash

set -e
# Ask for RELEASE_NAME
read -p "Enter RELEASE_NAME: " RELEASE_NAME

# Create releases directory if it doesn't exist
mkdir -p "releases/${RELEASE_NAME}"

# Read targets from .targets file and build
if [ -f .targets ]; then
    while IFS= read -r target; do
        rustup target add "${target}" --toolchain nightly
    done < .targets

    while IFS= read -r target; do
        if [ "${target}" = "x86_64-pc-windows-msvc" ]; then
            cargo xwin build --target "${target}" --release
            mkdir -p "releases/${RELEASE_NAME}/${target}"
            cp target/${target}/release/html2adf.exe "releases/${RELEASE_NAME}/${target}/"
            cp target/${target}/release/htmltoadf.dll "releases/${RELEASE_NAME}/${target}/"
            cp target/${target}/release/htmltoadf.dll.lib "releases/${RELEASE_NAME}/${target}/"
            cp target/${target}/release/libhtmltoadf.rlib "releases/${RELEASE_NAME}/${target}/"
        else
            cargo +nightly zigbuild --target "${target}" --release -Z unstable-options --out-dir="./releases/${RELEASE_NAME}/${target}"
        fi
    done < .targets
    echo "Build completed. Output files are in releases/${RELEASE_NAME}/ directory."

    # Create tar.gz archive excluding files specified in .gitignore
    git archive --format=tar.gz --output="releases/${RELEASE_NAME}/src.tar.gz" HEAD

    # Create zip archive excluding files specified in .gitignore
    git archive --format=zip --output="releases/${RELEASE_NAME}/src.zip" HEAD

    echo "Tar.gz and Zip archives created in releases/${RELEASE_NAME}/ directory."
else
    echo ".targets file not found."
fi
