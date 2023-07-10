---
title: "Qumulo"
header:
  overlay_color: "#000"
  overlay_filter: "0.4"
  overlay_image: /assets/img/portfolio/qumulo.jpg
  teaser: /assets/img/portfolio/qumulo-thumbnail.jpg
  actions:
    - label: Website
      url: "https://qumulo.com/"
    - label: Patent
      url: "https://uspto.report/patent/grant/11,354,273"
    - label: GitHub
      url: "https://github.com/Qumulo"
excerpt: "Member of Technical Staff, <br/> April 2020 - May 2022"
---

Served on a variety of teams helping build out an enterprise-scale filesystem
enabling customers to manage and curate petabytes of data. Qumulo is Seattle's
latest unicorn, hitting a $1.2 Billion valuation in 2020.

On my first team, I helped design and develop a now-patented max-flow algorithm
for storage layout, enabling customers to cluster nodes regardless of their
underlying hardware. This key feature allowed customers with old hardware to
expand existing clusters without needing to replace their existing nodes.

My second team was more focused on Site-Reliability. Aside from putting out
fires, I productized a variety of customer-facing scripts, allowing our customer
success team to more efficiently address issues in the field. I also spearheaded
a continuous integration and deployment pipeline for externally-facing scripts
to ensure they work on customer clusters when either the product or the script
is changed.

The third team I was on was tasked with redesigning our RPC server to enable
Kerberos support for NFSv4.1. We split functional layers for more effective
testing and "oxidized" the code from C to Rust, leading to an easy-to-find
memory leak from the previous server.

Off the team, I led engineering onboarding and drove improvements to materials
and processes to enable new hires to comfortably adjust to their team. To
promote community in an increasingly remote work environment, I introduced a
process checklist to ensure new hires are invited to meetings, given an active
"buddy" on their team, attend key tech talks, and join a cohort of engineers
that started around a similar time.

Additionally, I was a founding member of the Take-Home Interview initiative,
changing up the interview format to more properly reflect the working experience
of the software industry as well as reduce the stress associated with whiteboard
interviews. The asynchronous review process enabled the take-home team to
interview more candidates than a standard tech screen.
