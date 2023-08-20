{ config, pkgs, ... }:

{
  # Home Manager needs a bit of information about you and the paths it should
  # manage.
  home.username = "cwboden";
  home.homeDirectory = "/home/cwboden";

  # This value determines the Home Manager release that your configuration is
  # compatible with. This helps avoid breakage when a new Home Manager release
  # introduces backwards incompatible changes.
  #
  # You should not change this value, even if you update Home Manager. If you do
  # want to update the value, then make sure to first check the Home Manager
  # release notes.
  home.stateVersion = "23.05"; # Please read the comment before changing.

  # The home.packages option allows you to install Nix packages into your
  # environment.
  home.packages = [
    # # It is sometimes useful to fine-tune packages, for example, by applying
    # # overrides. You can do that directly here, just don't forget the
    # # parentheses. Maybe you want to install Nerd Fonts with a limited number of
    # # fonts?
    # (pkgs.nerdfonts.override { fonts = [ "FantasqueSansMono" ]; })

    # # You can also create simple shell scripts directly inside your
    # # configuration. For example, this adds a command 'my-hello' to your
    # # environment:
    # (pkgs.writeShellScriptBin "my-hello" ''
    #   echo "Hello, ${config.home.username}!"
    # '')
  ];

  # Home Manager is pretty good at managing dotfiles. The primary way to manage
  # plain files is through 'home.file'.
  home.file = {
    # # Building this configuration will create a copy of 'dotfiles/screenrc' in
    # # the Nix store. Activating the configuration will then make '~/.screenrc' a
    # # symlink to the Nix store copy.
    # ".screenrc".source = dotfiles/screenrc;

    # # You can also set the file content immediately.
    # ".gradle/gradle.properties".text = ''
    #   org.gradle.console=verbose
    #   org.gradle.daemon.idletimeout=3600000
    # '';
    ".config/nvim/init.vim".source = ~/.dotfiles/nvim/init.vim;

    ".config/fish/conf.d/keychain.fish".text = ''
      if status is-login
          and status is-interactive
          # To add a key, set -Ua SSH_KEYS_TO_AUTOLOAD keypath
          # To remove a key, set -U --erase
          SSH_KEYS_TO_AUTOLOAD[index_of_key]
          keychain --eval $SSH_KEYS_TO_AUTOLOAD | source
      end
    '';
  };

  # You can also manage environment variables but you will have to manually
  # source
  #
  #  ~/.nix-profile/etc/profile.d/hm-session-vars.sh
  #
  # or
  #
  #  /etc/profiles/per-user/cwboden/etc/profile.d/hm-session-vars.sh
  #
  # if you don't want to manage your shell through Home Manager.
  home.sessionVariables = {
    EDITOR = "nvim";
    SHELL = "fish";

    SSH_KEYS_TO_AUTOLOAD = "~/.ssh/id_ed25519";
  };

  # Let Home Manager install and manage itself.
  programs.home-manager.enable = true;

  programs.neovim = {
    enable = true;
    viAlias = true;
    vimAlias = true;
    plugins = with pkgs.vimPlugins; [
      fzf-vim
      gruvbox-nvim
      lightline-gruvbox-vim
      lightline-lsp
      lightline-vim
      nerdcommenter
      nerdtree
      nerdtree-git-plugin
      rust-vim
      vim-argwrap
      vim-devicons
      vim-fugitive
      vim-gitgutter
      vim-polyglot
    ];
  };

  programs.ripgrep.enable = true;

  programs.fish = {
    enable = true;
    plugins = with pkgs.fishPlugins; [
      { name = "tide"; src = pkgs.fishPlugins.tide.src; }
    ];
  };

  programs.direnv.enable = true;
}
