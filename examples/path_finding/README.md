# Path Finding Algorithm Practice
Adapted from the University of Michigan EECS281 path-finding project.

The executable should read in a maze in either List or Map format and find a
path to the exit. There are a variety of tiles that each influence how the
solver can move throughout the maze.

## Rules
The following characters can be encountered as part of the maze.

|Map Item|Character|Rule|
|---|---|---|
|Start Position|S|Where the algorithm should start searching for a path|
|Exit|E|Where the solver is trying to reach|
|Empty Cell|.|An empty grid cell within the maze|
|Wall|#|Blocks the solver from moving through it|
|Teleporter|[0-9]|Teleports the solver to the same grid location on the corresponding floor. (e.g. `4` would teleport to floor 4)|

## Maze Input
The mazes are read in via `stdin` and can be input in either `Map` or `List`
format.

Regardless, each file should contain the following on the corresponding lines:
1. Either an `L` or an `M` to specify the input mode.
2. An integer between 1 and 10 indicating the number of rooms in the maze
3. An integer between 1 and `u32::MAX` indicating the height and width of each
   room. (Each room is `NxN` and all rooms are the same size)

Comment lines are allowed and begin with `//`.

### List Format
Valid list input will contain a list of coordinates for _at least_ all walls
and teleporters in the room. The coordinates can appear in any order, though
are more readable when done room-by-room.

A coordinate is specified in the following form: `(room,row,column,character)`.
By default, all unspecified coordinates are of type `.`, but it's still valid
to specify them redundantly.

A sample map is as follows:

```
L
2
4
// sample M-mode input file with 2 4x4 rooms
// room 0
(0,0,1,C)
(0,2,3,S)
(0,3,0,#)
(0,3,2,1)
(0,3,3,#)
// room 1
(1,1,0,#)
(1,2,1,#)
(1,3,0,#)
```

### Map Format
Valid map input will contain a map of each room, in order, using the characters
specified above. A sample map is as follows (same maze as above):

```
M
2
4
// sample M-mode input file with 2 4x4 rooms
// room 0
.C..
....
...S
#.1#
// room 1
....
#...
.#..
#...
```
