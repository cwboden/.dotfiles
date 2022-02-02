# My Dotfiles
The collection of all my dotfiles.

## Installation
```sh
git clone https://github.com/cwboden/.dotfiles.git ~/.dotfiles
cd ~/.dotfiles
pip install -r requirements.txt
python build/bootstrap
```

## Notable Content
I also use this repo for personal projects and tinkering.

### Custom Build System
I had some fun putting together the build system for my repo.  It was also good
practice for learning about the `Protocol` paradigm in Python.

### One-Step Setup
I don't think it's perfect, but ideally this repo should be able to be
installed on any Linux distro and rope in all dependencies. I tried to test
this using GitHub's Workflows, but I haven't tried a fresh install on all
platforms.
