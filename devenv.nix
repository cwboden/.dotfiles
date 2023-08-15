{ inputs, pkgs, ... }:

{
  packages = [
    # CLI Tools
    pkgs.tmux
    pkgs.zsh
    pkgs.neovim

    # Python3 package management
    pkgs.poetry

    # Spell check library
    pkgs.hunspell
    pkgs.hunspellDicts.en_US
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
