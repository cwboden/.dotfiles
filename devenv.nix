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

  languages.python.enable = true;
  languages.python.version = "3.11.3";

  languages.rust.enable = true;
  languages.rust.channel = "stable";

  pre-commit = {
    hooks = {
      black = {
        excludes = [ "^practice/" ];
        enable = true;
      };
      hunspell.enable = true;
      mdsh.enable = true;
      mypy = {
        enable = true;
        excludes = [ "^practice/" ];
      };
      nixpkgs-fmt.enable = true;
      rustfmt.enable = true;
      shellcheck.enable = true;
      shfmt.enable = true;
    };
  };
}
