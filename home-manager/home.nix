{ config, pkgs, ... }:

{
  # Home Manager needs a bit of information about you and the paths it should
  # manage.
  home.username = "cwboden";
  home.homeDirectory = "/home/cwboden";

  home.stateVersion = "23.05"; # Refer to documentation on `stateVersion` before changing.

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
    pkgs.tmux
    pkgs.keychain
  ];

  home.file = {
    ".bashrc".text = ''
      . ~/.nix-profile/etc/profile.d/nix.sh
      home-manager switch
      # Embed fish shell to avoid needing to `chsh`
      fish; exit
    '';

    ".config/nvim/init.vim".source = ../nvim/init.vim;

    ".config/fish/conf.d/keychain.fish".text = ''
      if status is-login
              and status is-interactive
          # To add a key, set -Ua SSH_KEYS_TO_AUTOLOAD keypath
          # To remove a key, set -U --erase SSH_KEYS_TO_AUTOLOAD[index_of_key]
          keychain --eval $SSH_KEYS_TO_AUTOLOAD | source
      end
    '';

    ".gitconfig.local".source = ../git/gitconfig.local;
    ".gitconfig".source = ../git/gitconfig;
    ".gitexcludes".source = ../git/gitexcludes;
    ".gitmessage.txt".source = ../git/gitmessage.txt;

    ".tmux.conf".source = ../tmux/tmux.conf;
  };

  home.sessionVariables = {
    EDITOR = "nvim";
    SHELL = "fish";

    JAVA_HOME = "/usr/lib/jvm/java-17-openjdk-amd64/bin/java";

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

  programs.git = {
    enable = true;
    userName = "Carson Boden";
    userEmail = "carson.boden@gmail.com";
  };
}
