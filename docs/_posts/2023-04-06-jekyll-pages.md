---
title: "A Guide to Hosting Static Content via Jekyll & GitHub Pages"
excerpt: "How the simple-but-powerful technology behind this site works, and how to make deeper customizations with manual deployments."
last_modified_at: 2023-04-22T07:06:49
categories:
 - Programming
tags:
 - automation
 - jekyll
 - markdown
 - mermaid
 - liquid
---

{% include figure
    image_path='/assets/img/jekyll-site-droste.png'
    alt='A Droste Effect cascading image of my Jekyll site on this page.'
%}

## Overview
It's been over a year since I've migrated my personal site to Jekyll. I had
originally considered using React, as a way to practice a widely used
technology, but quickly learned that there wasn't actually much reactivity that
I needed from my site (and so, not much to practice on).

It was then that I stumbled upon Jekyll and learned the appeal (and different
approach) of static content sites. Here is some of what I've learned:

## Static Content
On the web, there are two main types of content: static and dynamic. The core
difference is in how content changes or responds to the user. In a static
website, content is stable and consistent for every user. Dynamic sites,
however, might pull content on-the-fly based on the specific user.

When the internet was still in its infancy, static content reigned supreme
since processing power was limited and web browsers didn't have consistent
library support. Most users could load HTML, maybe with some custom CSS, but
(at the time) weren't prepared to run code from the client-side.

However, as computers improved, the needle has since shifted nowadays towards
shiny, new, dynamic content, leaving many to forget about the appeal of static
sites.

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

The Jekyll gem is written in Ruby, though, since you don't _need_ to learn Ruby
to use Jekyll (though it can help for some customization), we won't be engaging
with that much. The core simplicity of managing a Jekyll site is its
organization via the file system:

```
docs/
├── assets/
│   └── <images, fonts, diagrams, etc.>
├── _config.yml
├── Gemfile
├── _pages/
│   ├── 404.md
│   └── about.md
└── _posts/
    ├── 2022-07-13-git-basics.md
    ├── 2022-11-20-automating-deployments.md
    └── 2023-04-06-jekyll-pages.md
```

Files in `_pages/` are turned into single, standalone pages needed by users. An
"About" page is a classic, as would be a "FAQ" or even the 404 page that users
find when clicking a malformed link.

Everything in `_posts/` is the blog content. They must follow the format
`YEAR-MONTH-DAY-title.MARKUP` so that Jekyll can sort and organize the posts
accordingly.

There are other possible folders, like `_sass/` or `_layouts/`, which can be
used for custom components and theming, but we won't cover that in this article.
I recommend [Jekyll's docs](https://jekyllrb.com/docs/structure/), which are
quite comprehensive.
{: .notice--info}

In order to write posts, though, we need to be familiar with the format that
Jekyll expects. This is where markup comes in:

### Markdown
Markdown is the most ubiquitous markup language today, mainly due to the
readability of its source code. The basic syntax makes it easy to perform common
word-processing formats like `**bold**`, `_italics_`, or \`code\` which Jekyll
transforms into what you see on the page.

The syntax has become commonplace, even outside of Markdown documents, with
platforms like [Reddit](https://www.markdownguide.org/tools/reddit/),
[Slack](https://slack.com/), or [Slab](https://slab.com/) applying Markdown
formatting right inside of a message or article. You can learn more advanced
syntax on [their documentation
page](https://www.markdownguide.org/cheat-sheet/).

Posts are written in Markdown, so they can be stored as code but then processed
by Jekyll to render a richer experience. For example, [here's the source code
for this page](https://github.com/cwboden/.dotfiles/blob/main/docs/_posts/2023-04-06-jekyll-pages.md).
{: .notice--info}

### Liquid
[Liquid](https://github.com/Shopify/liquid) is a lesser known markup language --
also powered by Ruby -- and incorporated into Jekyll's standard build process.
It's used for templating, allowing users to pass in data objects and produce
similarly shaped pages or page elements.

For example, each post in this blog has properties like `title` and `excerpt`
which Jekyll uses, along with the Liquid templates defined by the theme, to
create the feed of recent posts on the home page.
{: .notice--info}

This guide won't walk through customization via Liquid, since there are already
dozens of great examples from existing open source themes. If you like how this
page looks, I use Michael Rose's [Minimal
Mistakes](https://github.com/mmistakes/minimal-mistakes).

### Mermaid
The final markup language I highly recommend is
[Mermaid](https://mermaid.js.org/). It's a lightweight diagramming language
built in JavaScript that uses Markdown-inspired syntax.

I find Mermaid's declarative approach to diagrams much easier to edit than
full-fledged, responsive applications like LucidChart. Though for more complex
diagramming, Mermaid can get difficult to read.

This smaller block of code, though, is fairly easy to parse -- even without
knowing the syntax -- and will create the following diagram:

```
flowchart TD
    A[Free Time] -->B(Write a blog post)
    B --> C{Any new ideas?}
    C --> D[Game Development]
    C --> E[More Neon White]
    C --> |This one!| F[Jekyll Sites]
```

{% mermaid %}
flowchart TD
    A[Free Time] -->B(Write a blog post)
    B --> C{Any new ideas?}
    C -->D[Game Development]
    C -->E[More Neon White]
    C -->|This one!| F[Jekyll Sites]
{% endmermaid %}

Mermaid doesn't work natively with Jekyll, though we can add the
`jekyll-mermaid` gem to the project to add a Liquid template we can use.

I had some issues with the existing `jekyll-mermaid` package, so cut my own fork
that uses the latest version of Mermaid. You can specify a custom fork of the
gem by adding the `github` and `branch` properties
to your Gemfile:
{: .notice--info}

```rb
gem "jekyll-mermaid", :github => 'cwboden/jekyll-mermaid', :branch => 'master'
```

## GitHub Pages
As mentioned above, GitHub Pages is powered by Jekyll. We can combine a few
technologies here to quickly spin up a blog! [Their
walkthrough](https://docs.github.com/en/pages/setting-up-a-github-pages-site-with-jekyll/creating-a-github-pages-site-with-jekyll)
is comprehensive, so I won't rewrite the basic setup in this post, though I'll
call out some additional notes from my experience, below.

### "Safe" Deploy Process
GitHub by default uses a Jekyll "safe" deployment (specified with the `--safe`
flag when building via Jekyll). This makes the setup from their guide much
simpler, since GitHub Actions handles most of the deployment process.

However, as it blocks any plugins not on GitHub's allow list, it prevents
any custom behavior from being loaded. This is a problem if you're hoping to add
specific plugins like `jekyll-mermaid`.

Fortunately, there's a way to deploy via custom GitHub Workflows, meaning we can
get around this problem.
{: .notice--success}

### Deploying via GitHub Workflows
GitHub has a feature called Workflows, enabling repositories to run commands on
cloud resources. This can be great for automating tests, generating build
artifacts, and scripted deployments.

You can read [more about build and deployment automation via GitHub Workflows in
this post]({{ site.url }}/programming/2022/11/20/automating-deployments.html).
Or check out [the source files,
here](https://github.com/cwboden/.dotfiles/tree/main/.github/workflows).
{: .notice--info}

In our case of deploying a Jekyll site, we're fortunate that `jeffreytse` has
already provided [a useful action for deploying the
site](https://github.com/jeffreytse/jekyll-deploy-action). With just one code
block, we can trigger a fresh build and deploy anytime we merge into `main`:

{% raw %}
```yaml
# Use GitHub Deploy Action to build and deploy to Github Page
- uses: jeffreytse/jekyll-deploy-action@master
  with:
    provider: github
    token: ${{ secrets.GITHUB_TOKEN }} # Your Personal Access Token (PAT)
    jekyll_src: ./docs
```
{% endraw %}

And there we have it! By integrating a few technologies we can automate the
translation of simple-to-write Markdown documents into a rich, static site
experience.
{: .notice--success}

### Utterances Integration
As an added bonus, I recently learned about
[Utterances](https://github.com/utterance/utterances) and wanted to share. It's
a comment widget built novelly on top of GitHub Issues.

This was an extremely lightweight way to add comment support to articles, though
it does require any poster to login to a GitHub account. The fact that it uses
the Issues API on the backend, though, means that it can bring discussions about
bugs and blog posts together -- fun, yet wacky!

The widget is down below:
{: .notice--info}
