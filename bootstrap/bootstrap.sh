#!/usr/bin/sh

# CI ONLY / HACK:
# Override username / homeDirectory
if [ -n "$CI" ]; then
    # Home Manager expects USER to be `runner` in the CI pipeline
    sed -i 's/cwboden/runner/g' home-manager/home.nix

    # The runner also creates a `.bashrc` which we must remove
    rm ~/.basrc
fi

# Download and install Nix
sudo install -d -m755 -o $(id -u) -g $(id -g) /nix
curl -L https://nixos.org/nix/install | sh

# Activate Nix profile
. ~/.nix-profile/etc/profile.d/nix.sh

# Install Cachix
nix-env -iA cachix -f https://cachix.org/api/v1/install

# Install Devenv
cachix use devenv
nix-env -if https://install.devenv.sh/latest

# Link the Home Manager config to this repository
mkdir -p ~/.config/home-manager/
ln -s $(pwd)/home-manager/home.nix ~/.config/home-manager/home.nix

# Download and install Home Manager
nix-channel --add https://github.com/nix-community/home-manager/archive/master.tar.gz home-manager
nix-channel --update
nix-shell '<home-manager>' -A install

# Source profile to start Home Manager
. ~/.nix-profile/etc/profile.d/nix.sh

# Enter `fish` shell
fish
