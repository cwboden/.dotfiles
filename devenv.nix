{ inputs, pkgs, ... }:

{
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
      clippy.enable = true;
      hunspell.enable = true;
      markdownlint = {
        enable = true;
      };
      mdsh.enable = true;
      mypy = {
        enable = true;
        excludes = [ "^practice/" ];
      };
      nixpkgs-fmt.enable = true;
      rustfmt.enable = true;
      shellcheck.enable = true;
      shfmt.enable = true;
      yamllint.enable = true;
    };
  };
}
