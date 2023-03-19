---
title:
excerpt:
last_modified_at:
categories:
 - Video Games
tags:
  - story
  - simulation
  - philosophy
image_directory: "/assets/img/neon-white/"
---

|**Release Date:**| |
|**Developer:**| |
|**Publisher:**| |
{: .notice--info}

## Overview
Neon White's levels are designed to be explored and discovered, even though the
default routes are still challenging and zippy. I wanted to take a deep dive
into one of the levels to highlight where Neon White's level design shines.

We'll be looking at *Covenant: Hanging Gardens*, the first level of the sixth
mission.

## Neon White Brief
To quickly give some context on the game itself:
- FPS focused on speedrunning
- Levels are short, must kill all Demons and reach the finish line
- Weapons can be discarded for movement abilities

## The First Pass

<figure class="align-center">
  <img
    src="/assets/vector/neon-white-hanging-gardens-first-path.png"
    alt="">
</figure>

1. `Katana` kills Laser Trap, destroying Door

Immediately, first-time player are informed of new mechanic they will be
learning about for over the course of the level. at first, we see that they 1HP
and can destroy breakable objects.

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-first-room.jpg"
    alt="The first room in the level">
</figure>

1. Collect `Godspeed`
1. `Godspeed` kills both Laser Traps
1. Discard `Godspeed` through Door

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-first-enemy.jpg"
    alt="">
</figure>

1. `Katana` kills Laser Trap, killing Demon
1. `Katana` kill Laser Traps on Water platforms, killing `Godspeed` Demon

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-second-enemy.jpg"
    alt="">
</figure>

1. Collect and discard `Godspeed` through Door, collecting `Elevate`
1. Shoot Laser Trap chain, destroying Door
1. Discard `Elevate` to reach destroyed Door

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-elevate-room.jpg"
    alt="">
</figure>

1. `Katana` kills Laser Trap, killing Demon
1. `Katana` kills Laser Trap, killing `Stomp` Demon
1. Discard `Stomp`, killing `Stomp` Demon
1. Collect 2 `Purify`s

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-ramp-before-drop.jpg"
    alt="">
</figure>

1. Discard `Purify`, destroying Door
1. Discard `Purify`, killing Head Demon, `Stomp` Demon, and Laser Trap chain,
   destoying door.

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-head-room-1.jpg"
    alt="">
</figure>

1. Discard `Stomp`, killing Head Demon, `Stomp` Demon, and Laser Trap chain
1. Run off platform and discard `Stomp`, destroying all demons below and
   reaching the Finish

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-final-stomp.jpg"
    alt="">
</figure>

## My Optimized Route

<figure class="align-center">
  <img
    src="/assets/vector/neon-white-hanging-gardens-fast-path.png"
    alt="">
</figure>

1. `Katana` kills Laser Trap, destroying Door
1. Collect `Godspeed`

### Through the Looking Glass
1. `Godspeed` kills both Laser Traps

Note that each `Godspeed` card has four bullets, so the player is challenged to
shoot each of the three Laser Traps with accuracy, since they'll need `Godspeed`
to cross the gap, later.

1. Jump out window, `Godspeed` kills Laser Trap, killing Demon

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-sniper-shortcut-1.jpg"
    alt="">
</figure>

1. `Katana` kill Laser Traps on Water platforms, killing `Godspeed` Demon

### Bypassing the `Elevate` Room

1. Collect `Godspeed` and kill Laser Trap chain

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-sniper-shortcut-2.jpg"
    alt="">
</figure>

1. Discard `Godspeed` to bypass `Elevate` room
1. `Katana` kills Laser Trap, killing Demon
1. `Katana` kills Laser Trap, killing `Stomp` Demon
1. Discard `Stomp`, killing `Stomp` Demon

### Clearing Your Heads
This is the game-hinted optimization, noted by the Hand in the diagram as well.
In good form, the hint shows you *where* to go but not how to get there. In this
case, touching the hint shows the player they can immediately drop onto the goal
to complete the level.

1. Collect 2 `Purify`s

<figure class="align-center">
  <img
    src="{{ page.image_directory }}/hanging-gardens-ramp-before-drop.jpg"
    alt="">
</figure>

1. Discard both `Purify`s, killing Head Demons, `Stomp` Demons, and Laser Trap
   chains.
1. Run off platform and discard `Stomp`, destroying all demons below and
   reaching the Finish

### Decision Tree

```mermaid!
flowchart TD
    1[Shoot Laser Trap<br>Collect Godspeed<br>Shoot Laser Traps]
    1-->2a[Godspeed through door<br>Katana Laser Trap, killing Demon]
    1-->2b[Jump out window<br>Shoot Laser Trap, killing Demon]
    2a-->3
    2b-->3[Katana Laser Traps, killing Godspeed Demon]
    3-->4a[Godspeed through door<br>Collect Elevate<br>Shoot Laser Trap chain<br>Discard Elevate to reach platform]
    3-->4b[Collect Godspeed<br>Shoot Laser Trap chain<br>Discard Godspeeds to reach platform]
    4a-->5
    4b-->5[Katana Laser Traps, killing Demon and Stomp Demon<br>Discard Stomp, killing Stomp Demon<br>Collect 2 Purifys]
    5-->6a[Discard Purify through door<br>Discard Purify, killing first room of Demons<br>Discard Stomp, killing second room of Demons]
    5-->6b[Discard Purify, killing first room of Demons<br>Discard Purify, killing second room of Demons]
    6a-->7
    6b-->7[Discard Stomp, killing Demons below<br>Finish!]
```
