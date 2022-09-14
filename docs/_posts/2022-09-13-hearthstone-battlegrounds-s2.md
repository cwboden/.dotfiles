---
title: "Exploring Quest Design in *Hearthstone Battlegrounds* Season 2"
excerpt: "The newly-added quests have added some spice to Battlegrounds. I walk through which parts of the design worked to enhance the game vs. added more of the same."
last_modified_at:
categories:
 - Video Games
tags:
  - strategy
  - collectible-card-game
  - ranked
---

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-board.webp"
    alt="An example playfield of Hearthstone's Battlegrounds mode.">
</figure>

|**Release Date:**|30 Aug 2022|
|**Developer:**|Blizzard Entertainment|
|**Publisher:**|Activision Blizzard|
{: .notice--info}

## Success and Growth of *Battlegrounds*
After almost 3 years in the running (including Beta access), *Hearthstone
Battlegrounds* has become one of the game's cornerstone modes, largely in part
to it's ease of entry.

Unlike the game's *Standard* mode, *Battlegrounds* doesn't require players to
build a deck, but instead form their Warband of minions to fight other players
during the game itself. As a result, there's not the same disparity between
casual and long-term players when it comes to a sizeable collection of cards for
building high-quality decks like there is in *Standard*.

This makes it easy to invite a friend to jump into a game without needing to
unlock cards for a given class or buy packs for the cards they're missing in
their collection. Better yet, players who haven't played in a while won't need
to craft all their old cards into new ones, but can jump right into the same
playing field as everyone else!

### Brief Summary of Gameplay
In *Battlegrounds*, players compete against 7 others to be the last one
standing. Each round consists of two phases.

During the Recruit Phase, players buy minions from Bob's Tavern to add to their
Warband. Minions cost 3 Gold, though can later be sold back to Bob for 1 Gold.
Players start their first turn with 3 Gold, which increases by 1 Gold each turn
until reaching a max of 10 Gold.

In the Combat Phase, players are pitted 1v1 against a different, random opponent
in the game. Minions from each player take turns randomly attacking enemies
until one side has none remaining, at which point the winner attacks the loser.

Players must balance purchasing enough minions to avoid losing in Comabt and
depleting their health with upgrading Bob's Tavern to improve the quantity and
quality of offered minions. Rounds continue, with players fighting different
opponents until only the victor remains.

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-victory.webp"
    alt="The Battlegrounds victory screen">
</figure>

### Previous Expansions
Since the initial release in late 2019, Blizzard has added a variety of factions
to *Battlegrounds*. The new factions enabled the set of cards in the game
(previously a fixed list of the 5 available factions) to vary, changing the meta
each game.

For example, high-health minions become more powerful if Murlocs are not
present, since Murlocs can consistently get the **Poison** buff, which destroys
minions when they take any damage.
{: .notice--info}

The new factions also brought in fresh ideas to the format, since the game's
cards were first based off of existing cards from *Standard Hearthstone*. The
Quilboar faction, for example, added small buff spells called Bloodgems which
give permanent stats. The Naga, on the other hand, provide Spellcrafts, which
are more powerful, yet only buff a minion for one turn.

The development team also explored more powerful buffs with their Darkmoon Faire
expansion. During this special format *(no longer available)*, players would
choose one of three powerful spells every four turns, adding it to their hand.
This format was the standard for *Battlegrounds* for a while and was later
sprinkled into about 25% of matches for *Battlegrounds* Season 1.

<figure class="align-center third">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-might-of-stormwind.webp"
    alt="Darkmoon Prize: Might of Stormwind">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-gruul-rules.webp"
    alt="Darkmoon Prize: Gruul Rules">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-give-a-dog-a-bone.webp"
    alt="Darkmoon Prize: Give a Dog a Bone">
  <figcaption>
    Some examples of Darkmoon Prizes from the Darkmoon Faire expansion.
  </figcaption>
</figure>

### Overview of Quests
For those who haven't played the newest update to *Battlegrounds*, here's a
quick explanation of how Quests work:

On your fourth turn, players are asked to choose from one of three quests. Each
quest has a goal, like "Lose or tie 3 combats" or "Spend 25 gold" which, once
completed, gives the player a reward, such as "You only need 2 copies of a
minion to make it Golden" or "After you Discover a card, get an extra copy of
it".

<figure class="align-center">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-quest-choices.jpg"
    alt="The example of the Quest selection options in Battlegrounds.">
  <figcaption>
    On Turn 4, players must choose a Quest to help them scale later in the game.
  </figcaption>
</figure>

The goals scale slightly based on the relative power of rewards. So, for
example, "Summon X minions" might be 20 for a lower-impact reward but 30 for a
higher-impact one. This can incentivize players to choose "worse" rewards if it
means they'll get it earlier and can power through the midgame, which often
times can net a low-placing win.

Remember that getting 4th place or better is typically considered a "Win" (and
gives a raise in MMR) since there are eight players in each game, so 4th place
is in the top 50%.
{: .notice--info}

While Quests have no doubt shaken up the meta, I think it's important to
consider if the quests are expanding the game by changing the way players build
their Warbands or if they are instead amplifying existing strategies.

The answer is both! But let's explore some of the different Quests to see why
that's the case.

## Expanding Player Interaction
These Quests change how players interact with the game. Players will need to
adapt their strategy to handle new bonuses (and obstacles) that each Quest
provides.

<figure class="align-center third">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-alter-ego.png"
    alt="Battlegrounds Reward: Alter Ego">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-devils-in-the-details.png"
    alt="Battlegrounds Reward: Devils in the Details">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-teal-tiger-sapphire.png"
    alt="Battlegrounds Reward: Teal Tiger Sapphire">
</figure>

### Alter Ego
I like Alter Ego because of the interaction with Freeze. If you Freeze a
buffed minion, it'll lose the buff next turn, due to the swapping mechanic. But
similarly, freezing an un-buffed minion will cause it to be buffed the following
turn.

In this way, players may reconsider which minions they're Freezing. Perhaps a
player wants to buy a buffed minion to empower their board and wait for a
triple, but if they'll have to Freeze it for next turn, when it won't have the
buff, it's not worth it.

### Devils in the Details
Initially, this reward feels like a straight buff that you can stack onto the
right and left minions. It's possibly good for hard-to-buff cleave minions or
long-term buff minions that can't buff themselves.

However, once players begin Freezing important minions they want to buy next
turn, they'll realize the devil really *is* in the details: Those Frozen minions
each have a chance of being consumed!

Similar to **Alter Ego**, players will rethink how they'll be Freezing minions.
Is the Freeze worth it if the minion could be consumed? Is it better to re-roll
until there's a good set of stats in the Tavern to absorb instead? The Quest
changes how player's deal with the Tavern if they want to maximize their chance
for a win.

### Teal Tiger Sapphire
Many players can erroneously get caught in the cycle of spending all of their
money each turn Refreshing for a specific minion. But with this Quest, players
are incentivized to!

In Season 1 and earlier of *Battlegrounds*, committing to rolling for an entire
turn was a bad move, unless you had multiple pairs you were hoping to triple.
But now, players can commit to that strategy with a bigger payoff. That perfect
minion might not always be found, but this can give players an extra edge
towards piecing together a sweet combo.

## Intensifying Existing Strategies
Some of the other Quests in the set are powerful, though they serve more to
empower existing strategies and combos rather than create new player
experiences.

This isn't a bad thing; too many complicated Quests can lower the barrier to
entry that made Battlegrounds so successful. But I think its important for
designers to consider what future Quests will bring to the game, since expanding
the experience is what keeps the game fresh.

<figure class="align-center third">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-staff-of-origination.png"
    alt="Battlegrounds Reward: Staff of Origination">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-friends-along-the-way.png"
    alt="Battlegrounds Reward: The Friends Along the Way">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-theotars-parasol.png"
    alt="Battlegrounds Reward: Theotar's Parasol">
</figure>

### Staff of Origination
This was the most egregiously powerful-yet-boring card in the new expansion. In
fact, it was so powerful it got nerfed from giving minions +15/+15 to +12/+12.

At face value, there's rarely a reason *not* to choose this card: it gives the
most amount of stats out of any quest. While, in theory, it falls off compared
to other Quests once everyone's minions have been buffed enough, the constant
+12/+12 is enough to get most any build through the midgame, which normally
means ending up in the top 50%, with a win.

While "Vanilla" cards like this are important for introducing players to the
power of Quests, I find **Staff of Origination** to be extremely
one-dimensional. Players aren't doing much different once this Reward is active.

### The Friends Along the Way
Getting locked into a specific faction is a blessing and a curse with this
Quest. Most players will likely already be building towards the corresponding
faction in the Reward, but that could change based on the Quest Goal.

Regardless, though, the Quest offers more of the same, existing *Battlegrounds*
experience. The only change in playstyle is the comfort of getting two
guaranteed cards from a faction each turn (though if you're *not* playing to
that faction, you could treat it as two coins, instead).

Don't get me wrong, this Quest is still fun to play! And, unlike the first
iteration of **Staff of Origination**, it isn't unbalanced, since players might
not get the specific faction cards they need for their build. But the only
change in playstyle is forcing players to complete their Quest Goal to get this
reward.

### Theotar's Parasol
Another extremely fun idea that mainly serves to empower a few existing cards
like **Baron Rivendare**, **Soul Juggler**, or any card that doesn't ever want
to be attacked.

<figure class="align-center third">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-baron-rivendare.png"
    alt="Battlegrounds Minion: Baron Rivendare">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-soul-juggler.png"
    alt="Battlegrounds Minion: Soul Juggler">
  <img
    src="{{ site.url }}{{ site.baseurl }}/assets/img/hearthstone-bg-dread-admiral-eliza.png"
    alt="Battlegrounds Minion: Dread Admiral Eliza">
</figure>

In particular, I've found this Quest to amplify the Beasts + **Baron Rivendare**
build to powerful levels. Though it's nice that a buffed **Zapp Slywick** can
still cut through the buff.

While the Quest Reward improves the viability of some builds, it doesn't force
the player to change how they're building the Warband. The Quest itself is fun
to use, but it doesn't expand the *Battlegrounds* experience but rather
highlights existing strengths.

## Touching on Quest Goals
Since the Quest Goals themselves don't make players more powerful, they aren't
as interesting of a topic to explore on a card-by-card basis.

As a whole, though, I appreciate that the Goals are balanced around the Rewards
to encourage players to think about how difficult it will be to complete a Goal,
even if the reward is extremely juicy.

The Goals serve to hinder the player in some ways: Often if you take too long to
complete a Quest, you'll quickly fall behind, no matter how good the Reward is.
(This happened to me in several games.) In this way, the new Quest mechanic
requires players to commit to their choice, since winning *without*
incorporating your Quest is extremely difficult.

## The Future of Quests
I imagine that the current state of Quests is only the beginning. The developers
likely have more ideas on Goals and Rewards that will continue to diversify the
relatively small pool of options, but are seeing how the first wave of Quests
play out.

Some ways that Quests could change mechanically include:
 - Granting a new / additional Hero Power
 - Awarding special, unique minions
 - Debuffing opponents at the start of Combat
 - Repeatable Goals / Rewards (There's currently one Quest that does this
   already!)

For Rewards, to keep the system consistent, there will likely be new additions
that won't stray far from the script. The Goals need to be completable by all
heroes (though that could change) which means they can't rely too much on a
specific build.

As the Season progresses, I'll be excited to see if new ideas are introduced or
if they'll unveil new Quests in a future Season. In any case, the meta still
feels fluid thanks to Quests. I'd love to test out the few remaining Quests I
haven't seen as well as experiment more with Quests I've already tried. Who
knows what will come out on top? (Though **Poison** still feels very dominant.)
