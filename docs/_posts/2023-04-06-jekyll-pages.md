---
title: "A Guide to Hosting Static Content via Jekyll & GitHub Pages"
excerpt: "How it works, deeper customizations with manual deployments"
last_modified_at:
categories:
 - Programming
tags:
 - automation
 - jekyll
 - markdown
 - mermaid
 - liquid
---

## Overview


## Static Content
On the web, there are two main types of content: static and dynamic. The core
difference is in how content changes or responds to the user. In a static
website, content is stable and consistent for every user. Dynamic sites,
however, might pull content on-the-fly based on the specific user.

When the internet was still in its infancy, static content reigned surpreme
since processing power was limited and web browsers didn't have consistent
library support. Most users could load HTML, maybe with some custom CSS, but
(at the time) weren't prepared to run code from the client-side.

However, as computers improved, the needle has since shifted nowadays towards
dynamic content, leaving many to forget about the appeal of static sites.

{% mermaid %}
sequenceDiagram
    Client (User)->>Server: I want to access `https://carsonboden.com`
    Server->>Client (User): Here is the HTML for that page, have a nice day.
{% endmermaid %}
<figcaption>
  The interaction between client and server for a static website is
  straightforward; since the content doesn't change, the server returns the same
  HTML each time.
</figcaption>

### Compared to Dynamic Content
Dynamic content is the standard for most websites now that phones, computers,
and tablets are efficient enough to handle updates from the client side. Common
frameworks like React are designed around updating content to respond to user
interaction.

This user-specific interaction is especially important when it comes to custom
content, like a feed of customer-controlled subscriptions or the organization of
posts based on an engagement algorithm. Typically, updates are handled via
scripting languages, such as JavaScript, to modify the HTML and CSS as needed.

{% mermaid %}
sequenceDiagram
    Client (User)->>Server: I want to access `https://linkedin.com`
    Server->>Client (User): Here is the HTML and associated scripts for that page.
    loop As the page is used:
        note right of Client (User): Run scripts as user logs in, clicks elements, etc.
        Client (User)-->>Server: Scripts call server endpoints to fetch additional data
        Server-->>Client (User): Server talks with DB / blob storage to serve client requests
    end
{% endmermaid %}
<figcaption>
  Dynamic sites also include scripts which can change the page's content,
  responding to the user and their input, though also requiring more processing
  on the client-side.
</figcaption>

### When to Use Each
Static pages are best for content with infrequent modifications, like a blog,
portfolio, or documentation site. They are often easier to manage due to a lack
of database or client-server integration. In addition, load times are quick
because caching is simpler to manage with pages that won't change.

Search engine optimizations (SEOs) often heavily weights page load speeds,
meaning that static sites can boost content higher up in the search rankings.
{: .notice--info}

By contrast, though, users can't engage much with a static site. If you're
building a web application or platform meant for user engagement, dynamic sites
are probably what you want.

The ability to interact comes at a cost, though. Dynamic sites must have
external resources, like databases or blob storage in order to track the state
of various users, since pages change responsively.

## Using Jekyll for Static Sites
[Jekyll](https://jekyllrb.com/) is a simple, static site generator that can turn
common markup languages like Markdown or Liquid _(discussed more below)_ into a
complete website, ready to be hosted by most web servers. It's what powers
GitHub pages, which is also used to create this blog site.

The Jekyll gem is written in Ruby, though we won't be engaging with that much,
since you don't _need_ to learn Ruby to use Jekyll (though it can help for some
customization). The core organization of a Jekyll site is via the file system:

```
docs/
├── assets
│   └── <content files like images, fonts, etc.>
├── _config.yml
├── Gemfile
├── _pages
│   ├── 404.md
│   └── about.md
└── _posts
    ├── 2022-07-13-git-basics.md
    ├── 2022-11-20-automating-deployments.md
    └── 2023-04-06-jekyll-pages.md
```

Files in `_pages/` are turned into single, standalone pages needed by users. An
"About" page is a classic, as would be a "FAQ" or even a 404 page which users
will find if clicking a malformed link.

Everything in `_posts/` is the "most-dynamic" content. They must follow the
format `YEAR-MONTH-DAY-title.MARKUP` so that Jekyll can sort and organize the
posts accordingly.

There are other possible folders, like `_sass/` or `_layouts/`, which can be
used for custom components and theming, but we won't cover that in this article.
I recommend [Jekyll's docs](https://jekyllrb.com/docs/structure/), which are
quite comprehensive.
{: .notice--info}

In order to write posts, though, we need to be familiar with the format that
Jekyll expects. This is where markup languages come in:

### Markdown
Markdown is the most ubiquitous markup language today, mainly due to the
readability of its source code. The basic syntax makes it easy to perform common
word-processing formats like **bold**, _italics_, or `code`, which Jekyll
transforms into the forms you see on the page.

The syntax has become commonplace, even outside of Markdown, with platforms like
[Reddit](https://www.markdownguide.org/tools/reddit/),
[Slack](https://slack.com/), or [Slab](https://slab.com/) applying Markdown
formatting right inside of a message or article. You can learn more advanced
syntax on [their documentation
page](https://www.markdownguide.org/cheat-sheet/).

### Liquid
#### What is it?

### Mermaid
#### What is it?

## GitHub Pages
### "Safe" Deploy Process
### Deploying manually via GitHub Action
### Utterances Integration
