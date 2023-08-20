{ pkgs, ... }:

{
  languages.python = {
    enable = true;
    version = "3.11.3";
    poetry = {
      enable = true;
      activate.enable = true;
    };
  };

  pre-commit = {
    hooks = {
      isort.enable = true;
      ruff.enable = true;
    };
  };
}
