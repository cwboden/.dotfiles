{ config, inputs, pkgs, environment, ... }:

packages = [
# Spell check library
pkgs.hunspell
pkgs.hunspellDicts.en_US

# C libraries for Game Dev
pkgs.SDL2
pkgs.SDL2_image
pkgs.SDL2_mixer
pkgs.SDL2_ttf
pkgs.alsa-lib
pkgs.libudev-zero
];

languages.c.enable = true;
languages.cplusplus.enable = true;

languages.python = {
enable = true;
version = "3.11.3";
poetry = {
enable = true;
activate.enable = true;
};
};

languages.ruby.enable = true;

languages.rust.enable = true;
languages.rust.channel = "stable";

pre-commit = {
hooks = {
# Spellcheck. Maybe someday?
# hunspell.enable = true;
isort.enable = true;
mdsh.enable = true;
nixpkgs-fmt.enable = true;
ruff = {
enable = true;
excludes = [ "^practice/" ];
};
rustfmt.enable = true;
shellcheck.enable = true;
shfmt.enable = true;
};
};
