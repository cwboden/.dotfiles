{ pkgs, ... }:

{
  packages = [
    # Python3 package management
    pkgs.poetry
  ];

  languages.python = {
    enable = true;
    version = "3.11.3";
    poetry = {
      enable = true;
      activate.enable = true;
    };
  };
}
