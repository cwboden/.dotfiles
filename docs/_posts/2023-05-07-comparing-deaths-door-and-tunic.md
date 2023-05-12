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
to as **Soulsborne** games (derived to also include FromSoftware's critically
acclaimed *Bloodborne*).

I challenge the notion that high difficulty is a ***requirement*** for
*Souls*-like games and will argue that it is instead an expectation from
contemporary gamers.
{: .notice--warning }

I'll expand on some other common themes that are found in *Souls*-like games,
though there's no official definition.

### Redesigned Rest Points
One of the ways that *Souls*-like games are so unique (and classifiable) is the
way in which players must heal their character.

In *Souls*-like games, the player is typically restricted from healing or
resting except at designated sites. However, when the player chooses to do so,
they will ***also*** heal all enemies across the entire gameworld.

The recovery mechanic creates a powerful and exciting tension, since players
can't kill a few enemies, then just run back to the nearest checkpoint to heal
up. To further compound the effect, [enemies often take serious investment to
kill](#methodical-combat-using-stamina), meaning players must decide if a rest
will be worth fighting through familiar foes again.

This is also one of the ways that difficulty in *Souls*-like have become a
presumption: As genre-experienced gamers grew in experience and more casual
players were pushed out, more difficult enemies were required to continue
fostering the kill-or-be-killed tension of combat.
{: .notice--info }

{% capture tangent-health %}
Most games with a notion of "health" or "lives" have a way of using that
resource to incentivize an experience. In a first-person shooter focused on
energy and excitement, for example, a game might use cover-based combat and
a health bar which quickly recovers when hiding.

These two mechanics together keeps combat snappy, since players need only
take a quick pause and then can jump back into the action. We can see this
done in plenty of AAA titles like *Uncharted*, *Call of Duty*, and
*Battlefield*.

But a designer could instead use spacious levels and remove the ability to
recover health, creating a much slower, tense game. Players will want to
surprise their enemies to avoid losing as much health as they can.

We can see this design used at great success in *Counter-Strike* or even
*Call of Duty* sub-games like *"One in the Chamber"* where players have only
one health and a pistol with a single bullet.

Here are some other ideas of how health mechanics can be used in some other
genres:

|Genre|Recovery Mechanic|Experience Objective|
|-----|-----|---|
|Platformer|Collect Pickup|Introduce tension between challenge of collecting the item and the risk of failing to do so. *(e.g. a difficult to reach 1-UP)*|
|Fighter|Respawn / Lives|Incentivize players to fight tooth-and-nail to inflict as much damage before losing a stock. *(e.g. 2 KO's in one life)*|
|Rhythm|Rebuild Multiplier|Encourage mastery of die-hard players while tolerating higher levels of failure. *(e.g. scores climb quadratically with skill)*|
|MOBA|Return to Base|Healing is always an option, at the cost of not exerting influence around the map. *(e.g. not around to stop your opponent)*|
{% endcapture %}
<details>
  <summary>
    <strong>A tangent on using Health to build an experience</strong>
  </summary>
  {{ tangent-health | markdownify }}
</details>

There is a difference between engine-orchestrated game *saves* (which might
track internal events) and player-chosen *rests*, which only serve to replenish
missing resources and respawn all enemies.
{: .notice }

- Otherwise difficult to heal (Estus Flask)

### Methodical Combat using Stamina
- Methodical, punishing combat
- Big boss fights

### Environmental Storytelling
- Deep lore, since no exposition dump to the player
- Exploration
- Mechanically, tension between the player and the world grows further with
    currency that's lost when you die

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
|Puzzle/Mystery|Narrative (e.g. Splash art has more characters)|
|Health/Stamina Bars|Retro Pip-based Resources|

### How Each Changes the Formula

### Standout Mechanics

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
