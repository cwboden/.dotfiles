---
title: "Ocient Internship"
header:
  image: /assets/img/portfolio/ocient.png
sidebar:
  - title: "Role"
    text: "Intern"
  - title: "Date"
    text: "May 2019 - Aug 2019"
---

Contributed to the development of an exabyte-scale distributed database.  The
internship was divided into two parts on two different teams to get a feel for
different parts of the company.

On the first team, I worked on designing a testing framework to inject errors
into system calls made by the database.  This allowed the team to simulate
failed hardware and improve fault tolerance.  In addition, I worked on migrating
a complex testing script from its archaic Bash form into Python, allowing easier
upkeep and better integration with Ocient's other Python libraries.  Finally, I
coupled output from testing suites with JIRA tracking to enable teams to quickly
determine how tests perform on nightly builds.

The main project on the second team surrounded optimizing Ocient's
implementation of a distributed consensus protocol.  Key features included
optimizing for new nodes to catch up faster to an existing database, rejecting
nodes that are slowing down the system, and allowing the database to be quickly
rebuilt from an invalid state.  The improvements not only provided faster
database speeds for internal testing but also added powerful features of which
clients can take advantage.
