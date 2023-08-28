{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  pre-commit = {
    hooks = {
      nixpkgs-fmt.enable = true;
      rustfmt.enable = true;
    };
  };
}
