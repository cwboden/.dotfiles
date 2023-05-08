---
title: "Exploring the *Souls*-like Genre via *Death's Door* and *TUNIC*"
excerpt: "Comparing the small differences in experience, art, and design which can add up to drastically different games in the same genre."
last_modified_at:
categories:
 - Video Games
tags:
  - story
  - simulation
  - philosophy
---

{% include figure
    image_path='/assets/img/deaths-door-and-tunic-splash.jpg'
    alt="The splash art for both Death's Door and TUNIC."
%}

## Overview

I recently finished playing *TUNIC*, which reminded me a lot of *Death's Door*.

| |<em><strong>Death's Door</strong></em>|<em><strong>TUNIC</strong></em>|
|**Release Date:**|20 Jul 2021 <em>(PC)</em>|16 Mar 2022 <em>(PC)</em>|
|**Developer:**|Acid Nerve|Isometricorp Games|
|**Publisher:**|Devolver Digital|Finji|
{: .notice--info }

## What Defines a *Souls*-like Game?
Since we'll be discussing how *Death's Door* and *TUNIC* each follow this
format, we should clarify what it is. The definition has plenty of room for
interpretation, so for the purposes of this article, I will be using ***my***
definition of *Souls*-like.

At a high-level, though, a good starting definition might be:

A *Souls*-like is a subgenre of action role-playing and action-adventure games
known for high levels of difficulty and emphasis on environmental
storytelling
<br>&emsp;-- [Wikipedia](https://en.wikipedia.org/wiki/Soulslike)
{: .notice--info }

Notably, the origin of the name comes from the *Demon's Souls* and *Dark Souls*
series by FromSoftware. For clarity, those *specific* games are instead referred
to as **Soulsborne** games (derived to include the critically acclaimed
*Bloodborne*, also by FromSoftware).

I challenge the notion that high difficulty is a ***requirement*** for
*Souls*-like games and will argue that it is instead an expectation from
contemporary gamers.
{: .notice--warning }

### Redesigned Rest Points
One of the ways that *Souls*-like games are so unique (and classifiable) is the
way in which players must heal their character.

In *Souls*-like games, the player is typically restricted from healing or
resting except at designated sites. However, when the player chooses to do so,
they will ***also*** heal all enemies across the entire gameworld.

This creates a powerful and exciting tension, since players can't kill a few
enemies, then just run back to the nearest checkpoint to heal up. To further
compound the effect, [enemies often take serious investment to
kill](#methodical-combat-using-stamina), meaning players must decide if a rest
will be worth fighting through familiar foes again.

This is one of the ways that difficulty in *Souls*-like have become a
presumption (but, I argue, still not a requirement). Experienced gamers *expect*
high-difficulty enemies in order to foster the kill-or-be-killed tension of
combat.
{: .notice--info }

In other genres, players

|Genre|Recovery Mechanic|Experience Objective|
|-----|-----|---|
|Multiplayer FPS|Hide & Heal|Quickly get player back into the fight, keeping combat snappy and engaging.|
|Platformer|Collect Pickup|Introduce tension between challenge of collecting the item and the risk of failing to do so (e.g. 1-UP).|
|Fighter|Respawn|Incentivize players to fight tooth-and-nail to inflict as much damage before losing a stock.|
|Rhythm|Rebuild Multiplier|Encourage mastery of die-hard players while tolerating higher levels of failure, keeping accessibility high.|

There is a difference between engine-orchestrated game *saves* (which might
track internal events) and player-chosen *rests*, which only serve to replenish
missing resources and respawn all enemies.
{: .notice }

- Rest points where all enemies respawn
- Currency for player upgrades which is lost on death
- Otherwise difficult to heal (Estus Flask)
- Creates tension in combat, rather than players often returning to a rest-point
    to heal after killing a few enemies

### Methodical Combat using Stamina
- Methodical, punishing combat
- Big boss fights

### Environmental Storytelling
- Deep lore, since no exposition dump to the player
- Exploration

## Things in Common
- Isometric *Souls*-like Action-Adventure Games
- Cutesy, animal protagonist
- Multiple endings

Minor Similarities:
- Feature Night/Day cycle (not ongoing, but active, triggered by the player)

### *Souls*-like Influence

### Common Boss Organization
I will postulate that most *Souls*-like games follow this (general) approach to
presenting bosses to the player.

{% mermaid %}
{% raw %}
flowchart LR
    T(Tutorial)
    T --> 1
    1[First]
    1 --> A
    1 --> B
    1 --> C
    style A fill:#8b0000
    style B fill:#013220
    style C fill:#00008b
    A --> P
    B --> P
    C --> P
    P[Penultimate]
    P --> F
    F{{Final}}
{% endraw %}
{% endmermaid %}

Initially, a Tutorial boss is used to demonstrate how this different "type" of
gameplay works. Players are used to dealing with enemies from the standard
gameplay loop, but what changes when fighting a boss?

Often, these bosses become standard enemies in the remainder of the game. (e.g.
Rudeling from *TUNIC*)

Then, the player continues with the game until they are confronted with their
first "real" boss.

Afterwards, they have proved themselves worthy enough to enter any part of the
world. From here, most bosses are able to be tackled in any order.

Once obtaining the necessary
[MacGuffins](https://en.wikipedia.org/wiki/MacGuffin), the player will typically
then "present" these tokens to fight the "gatekeeper" boss. A boss meant to
guard the final levels of the game, where the player will afterwards confront
the FINAL boss.

## Differences Between the Games

|TUNIC|Death's Door|
|:---|:---|
|Flask|Life Seeds|
|Puzzle/Mystery|Narrative|
|Health/Stamina Bars|Retro Pip-based Resources|

## *TUNIC*

The description of *TUNIC* makes it sound like a very similar game to *Death's
Door*. And while that's true at a high-level, we'll see how they differ.

Explore a land filled with lost legends, ancient powers, and ferocious monsters
in *TUNIC*, an isometric action game about a small fox on a big adventure.
{: .notice }

Reaping souls of the dead and punching a clock might get monotonous but it's
honest work for a Crow. The job gets lively when your assigned soul is stolen
and you must track down a desperate thief to a realm untouched by death - where
creatures grow far past their expiry.
{: .notice }

{% mermaid %}
{% raw %}
flowchart TD
    T(East Belltower<br><em><strong>GUARD CAPTAIN</strong></em>)
    T --> 1
    1[West Belltower<br><em><strong>GARDEN KNIGHT</strong></em>]
    1 --> A
    1 --> B
    1 --> C
    A[East Forest<br>Eastern Vault<br><em><strong>SEIGE ENGINE</strong></em>]
    style A fill:#8b0000
    B[Ruined Atoll<br>Frog's Domain<br>Grand Library<br><em><strong>THE LIBRARIAN</strong></em>]
    style B fill:#013220
    C[Quarry<br>Monastery<br>Rooted Ziggurat<br><em><strong>BOSS OF THE SCAVENGERS</strong></em>]
    style C fill:#00008b
    A --> P
    B --> P
    C --> P
    P[Oubliette<br><em><strong>THE HEIR</strong></em>]
    P --> I
    I[Swamp<br>Cathedral<br>Hero's Grave]
    P -.-> F
    I --> F
    F{{"Oubliette<br><strong><em>THE HEIR</em> (again)</strong>"}}
{% endraw %}
{% endmermaid %}

## *Death's Door*

{% mermaid %}
{% raw %}
flowchart TD
    T(Grove of Spirits<br><em><strong>DEMONIC FOREST SPIRIT</strong></em>)
    T --> 1
    1[Lost Cemetery<br><em><strong>GUARDIAN OF THE DOOR</strong></em>]
    1 --> A
    1 --> B
    1 --> C
    A[Ceramic Manor<br>Inner Furnace<br><em><strong>THE URN WITCH</strong></em>]
    style A fill:#8b0000
    B[Mushroom Dungeon<br>Flooded Fortress<br><em><strong>THE FROG KING</strong></em>]
    style B fill:#013220
    C[Castle Lockstone<br>Old Watchtowers<br><em><strong>BETTY</strong></em>]
    style C fill:#00008b
    A --> P
    B --> P
    C --> P
    P[Lost Cemetery<br><em><strong>THE GREY CROW</strong></em>]
    P --> F
    F{{Hall of Doors<br><em><strong>THE LORD OF DOORS</strong></em>}}
{% endraw %}
{% endmermaid %}
