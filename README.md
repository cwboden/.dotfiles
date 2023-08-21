# My Dotfiles
[![Submission Tests](https://github.com/cwboden/.dotfiles/actions/workflows/submission-tests.yml/badge.svg)](https://github.com/cwboden/.dotfiles/actions/workflows/submission-tests.yml)

The collection of all my dotfiles.

I also use this repo for personal projects and tinkering. Having all of my projects in one place
makes it easy to share code between projects and ensure tests are run when pushing code back to the
repo.

## One-Step Setup
It's not perfect, but ideally this repo should be able to be installed on any Linux distro and rope
in all dependencies. I tried to test this using GitHub's Workflows, but I haven't tried a fresh
install on all platforms.

You should be able to get things started on Ubuntu using the following:

```sh
git clone https://github.com/cwboden/.dotfiles.git ~/.dotfiles
cd ~/.dotfiles
./bootstrap/bootstrap.sh
```

## Running Jekyll Locally
After bootstrapping, the necessary dependencies should be installed to run the Jekyll server which
contains the portfolio website and blog. You can start the server by running:

```sh
cd ~/.dotfiles/docs/
bundle exec jekyll serve
```

## Custom Build System
I had some fun putting together the build system for my repo.  It was also good practice for
learning about the `Protocol` paradigm in Python.

The main goal is to link together some `Predicate` that will inform the build system whether an
action needs to be taken (e.g. if a file is not present, a build artifact doesn't exist, etc.) with
an `Action` which is what the build system will actually do when the `Predicate` isn't met. There
may be further improvements to this idea, but I felt like it was a good starting point for generic,
conditional building.

## Argparse
An attempt at recreating the `argparse` library from Python in Rust. The implementation and
featureset is still primitive, but can be expanded for any additional features required by future
projects.

## Path-Finding
I recreated a path-finding project from an old class (EECS 281) in Rust. I decided not to follow
through with all of the I/O required for the class's autograder and used integration tests instead.

Try it out with:

```sh
cargo run --package path_finding -- --help
```

## Bevy Engine
Despite it's relative infancy in the game dev scene, I've enjoyed prototyping games and systems in
[Bevy Engine](https://bevyengine.org/). Most of the projects are incomplete, but you may find some
nuggets of information buried within.

### Gaia Project
An in-progress port of Jens Drögemüller and Helge Ostertag's [Gaia
Project](https://boardgamegeek.com/boardgame/220308/gaia-project). The UI is still extremely
primitive, but I've been able to interlace systems like:
 - The power cycle, including charging, spending, reserving, and paying for power actions.
 - Claiming federation tokens, with resource rewards being automatically awarded.
 - Paying for various actions like terraforming, buildings, and research.

Try it out with:

```sh
cargo run --package gaia_project
```

### Cards
Decks and Piles of arbitrary "Card" objects can be created, shuffled, and reset to initial
positions. The idea is to make it easier to prototype simple card games without needing to create
the deck logic every time.

An in-progress example using a standard 52-card deck might someday be completed.

### Pocket Ops
Another board game port of Brandon Beran's [Pocket
Ops](https://boardgamegeek.com/boardgame/216234/pocket-ops), a variant of tic-tac-toe where guessing
your opponent's move means they won't be able to play a piece!

In it's current state, it's just tic-tac-toe, since I can't think of a good way to obscure your
moves from an opponent's if playing on the same screen / keyboard, but it has been a fun project to
practice using Bevy.

Try it out with:

```sh
cargo run --example pocket-ops
```
