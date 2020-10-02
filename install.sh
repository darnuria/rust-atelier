#!/bin/bash

# Doc : https://rustup.rs/
echo "Installation de l'ecosystème Rust avec Rustup"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
echo "$HOME/.cargo.env" >> ~/.bashrc

# Installtion d'un editeur vscodium
release_url="https://github.com/VSCodium/vscodium/releases/download/1.49.2/"
vscodium_file="codium_1.49.2-1600994273_amd64.deb"
vscodium_sha="codium_1.49.2-1600994273_amd64.deb.sha256"
wget "$release_url$vscodium_file" && wget "$release_url$vscodium_sha"
echo "$release_url$vscodium_file"
# Check telechargement correct
(sha256sum --check --status < $vscodium_sha)  && sudo dpkg -i $vscodium_file


sudo apt install build-essential valgrind

# Installation rustlings par défaut dossier courant.
curl -L https://git.io/rustlings | bash

echo "Run :\n $ source $HOME/.cargo/env"
