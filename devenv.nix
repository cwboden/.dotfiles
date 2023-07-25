{ pkgs, ... }:

{
  languages.python.enable = true;
  languages.python.version = "3.11.3";

  languages.rust.enable = true;
  languages.rust.version = "stable";
}
