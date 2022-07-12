---
title: Easy to Use Tools for Larger Codebases
---

I've recently started using tools to aid the development of the game I've been
working on, and I believe CMake, GoogleTest, and Git Hooks are not only
incredibly powerful, but also easy to setup and use. I have only just started
learning them, but I imagine they will be continuously useful.

CMake was a tool that I've heard of many a time, but I was so used to manually
creating my own Makefiles that I never took the time to learn how to use CMake.
While I still don't fully understand the intricacies of the platform, I respect
the ability for CMake to deploy a build solution regardless of the operating
system and compiler. Setting it up required me to read through the [basic
tutorial](https://cmake.org/cmake-tutorial/) which mainly introduces concepts,
but it got me on the right track, and I hope to learn more.

I have always been a proponent of quality testing of code. Not only does it
ensure that your code works, but more importantly, it shows you where it breaks.
GoogleTest is a framework that takes tests and presents them in an
easy-to-visualize way, making problems visible at a glance. Like with CMake, I
have only scratched the surface with GoogleTest via their
[README](https://github.com/google/googletest/blob/master/googletest/README.md)
but it was enough to get me started and interested.

I have a bit more experience with Git Hooks, given that I used them last
semester, but I still believe I have much to learn. The idea behind Git Hooks is
to integrate a series of scripts that are called whenever a corresponding action
is called via a contributer (push, commit, etc.). The scripts have a variety of
uses, but I mainly used them to ensure unit tests are created and working before
allowing a commit. There are limitless possibilities, though, that can be read
about on the [Git
Wiki](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).

Developers have spent tons of time creating frameworks that can provide utility
to projects large and small. Finding the best of these tools can take work, and
learning to use them can take more.  But the earlier you start learning, the
sooner you can learn to use them.
