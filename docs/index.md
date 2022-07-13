---
layout: splash
author_profile: true
header:
  overlay_color: "#000"
  overlay_filter: "0.3"
  overlay_image: /assets/img/plum-blossoms.jpg
  actions:
    - label: All Posts
      url: /by-year/
    - label: Resume
      url: /assets/pdf/resume.pdf
excerpt: Welcome to my portfolio! I'm a programmer-for-good, lover of games, and write random blog posts in my spare time.
feature_row:
  - image_path: /assets/img/spiritfarer-splash.jpg
    alt: "Latest Review: Spiritfarer"
    title: "Video Games"
    excerpt: A dive into mechanics, player experience, what "worked", and what didn't.
    url: /categories/video-games
    btn_label: "Read More"
    btn_class: "btn--primary"
  - image_path: /assets/img/code-snippet.png
    title: "Programming"
    excerpt: Experimentation, abandoned projects, and maybe some nuggets to learn from.
    url: /categories/programming
    btn_label: "Read More"
    btn_class: "btn--primary"
  - image_path: /assets/img/old-books.jpg
    image_caption: "Image courtesy of [Wikimedia](https://commons.wikimedia.org/wiki/File:Old_books_by_bionicteaching.jpg)"
    alt: "View all categories"
    title: "All Categories"
    excerpt: Explore to your heart's content. See if you can find something interesting.
    url: /categories/
    btn_label: "Choose"
    btn_class: "btn--inverse"
---

{% include feature_row id="intro" type="center" %}

{% include feature_row %}
