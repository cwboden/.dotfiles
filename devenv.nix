{ inputs, pkgs, ... }:

{
  packages = [
    # CLI Tools
    pkgs.tmux
    pkgs.zsh
    pkgs.neovim
    pkgs.ripgrep

    # Python3 package management
    pkgs.poetry

    # Spell check library
    pkgs.hunspell
    pkgs.hunspellDicts.en_US

    # C libraries
    # -> game dev
    pkgs.SDL2
    pkgs.SDL2_image
    pkgs.SDL2_mixer
    pkgs.SDL2_ttf
    pkgs.alsaLib
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
}
