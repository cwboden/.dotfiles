---
title: Git basics (From a work presentation)
excerpt:
last_modified_at: 2022-07-14T01:31:13
categories:
 - Programming
tags:
  - git
  - tools
  - education
---

There are plenty of guides to using `git` all around the web. Why should I add
another drop to a vast ocean of resources?

Well, as someone who's professional experience with
[VCS](https://en.wikipedia.org/wiki/Version_control) has only involved
[Mercurial](https://www.mercurial-scm.org/), I wanted to give myself a deeper
understand of a tool I was going to be using daily. Furthermore, [research
shows](https://onlinelibrary.wiley.com/doi/abs/10.1002/acp.3410) that teaching
others improves one's understanding of the taught material.

When I was offered the chance to give a presentation on `git` at work, I took it
to build my own understanding, and, hopefully, my colleagues' as well. While
this won't be the *most* comprehensive guide on `git`, I wanted to highlight my
learnings to myself and to you, dear reader.

This guide will assume you have basic understandings of *why* developers use
`git`, though will still dive into "basic" topics that I find interesting from a
technical or usability standpoint.
{: .notice--info }

## Common Workflows
Most developers use a single, centralized repository that all individuals `push`
and `pull` from. But aside from that, there's rarely a "one size fits all"
solution to `git`. Here are some basic approaches that seemed sane in my
research:

### Gitflow
The Gitflow approach was pioneered in 2010 by [Vincent Driessen at
nvie](https://nvie.com/posts/a-successful-git-branching-model/) and has
wormed its way into enterprise software globally. The idea revolves around two
core branches in your repo: `main` (or `master`) and some `development` branch.

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/git-workflow-gitflow.png"
    alt="">
</figure>

The `development` branch is where *all* branches diverge from and are eventually
re-merged. Then, whenever a new version of the software should be cut, the
`development` branch is merged into `main`.

Effectively, `main` is *only* ever touched by `development`. This prevents any
incomplete code from being checked into `main` and makes the commit history
(ideally) easier to parse, since each commit represents a new version of the
product.

As mentioned in Driessen's 2020 reflection about Gitflow, in the 10 years since,
most software is continuously released, rather than released version-by-version.
Subsequently, a simpler workflow can accomplish the same result.

### Feature Branches *(Preferred)*
Feature branches are, for the most part, identical to the Gitflow approach, with
one major difference: There isn't a `development` branch. By gating merge
commits to `main` with tests, developers can prevent broken code from ever
making it into `main`, which is a huge advantage for continuous integration and
deployment environments.

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/git-workflow-feature-branches.png"
    alt="">
</figure>

Rather than merging from `development` into `main` whenever a release is
necessary, commits in `main` can be [tagged as a
release](https://git-scm.com/book/en/v2/Git-Basics-Tagging). This still provides
the rigid versioning structure necessary for some releases without requiring the
overhead of additional branches and special rules.

### Forking
The forking approach breaks the single-remote mold. Each developer has their
*own* remote that they are able to write to at their leisure (even pushing to
`main`!) This is often used for free and open-source (FOSS) projects, so that
the maintainer doesn't need to meddle around with permissions for contributers.

To understand forking, it's first important to understand how remotes work. This
was something I learned while researching this presentation:

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/git-concept-remotes.png"
    alt="">
</figure>

Remotes are, true to their name, remote repositories to synchronize your changes
with. While most developers use only one, typically hosted on something like
GitHub or BitBucket, multiple remotes can split the management of a codebase
more than just a branch can -- by allowing completely different commit trees
entirely!

With the forking workflow, developers create their own copy of the source repo
to work in. They can always pull in changes from the source repo to their own
copy, that way they'll stay in sync with new features.

When they're ready to commit a change upstream, they make a pull request (PR)
where they can discuss their changes before merging in. In most cases, the
maintainer of the source repo is the only one with write access and is therefore
the "keeper" of features making it into the mainline.

## Other Important Concepts

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/git-concept-branches.png"
    alt="">
</figure>

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/git-concept-merging.png"
    alt="">
</figure>

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/git-merge-vs-squash-commit.png"
    alt="">
</figure>
