{ inputs, pkgs, environment, ... }:

{
  packages = [
    # CLI Tools
    pkgs.ripgrep
    pkgs.tmux

    # Shell & Plugins
    pkgs.fish
    pkgs.fishPlugins.tide

    # Spell check library
    pkgs.hunspell
    pkgs.hunspellDicts.en_US

    # C libraries for Game Dev
    pkgs.SDL2
    pkgs.SDL2_image
    pkgs.SDL2_mixer
    pkgs.SDL2_ttf
    pkgs.alsa-lib

    # Vim & Plugins
    pkgs.neovim
    pkgs.vimPlugins.fzf-vim
    pkgs.vimPlugins.gruvbox-nvim
    pkgs.vimPlugins.lightline-gruvbox-vim
    pkgs.vimPlugins.lightline-lsp
    pkgs.vimPlugins.lightline-vim
    pkgs.vimPlugins.nerdcommenter
    pkgs.vimPlugins.nerdtree
    pkgs.vimPlugins.nerdtree-git-plugin
    pkgs.vimPlugins.rust-vim
    pkgs.vimPlugins.vim-argwrap
    pkgs.vimPlugins.vim-devicons
    pkgs.vimPlugins.vim-fugitive
    pkgs.vimPlugins.vim-gitgutter
    pkgs.vimPlugins.vim-polyglot
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
