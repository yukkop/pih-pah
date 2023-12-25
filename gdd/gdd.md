Version of GDD 0.1.0

# Intro
"Shooter" in medieval times with ancient technologies.

# Controls
* Fire - `Left Mouse Button`
* Inscription - `Right Mouse Button`
* Special - `F`
* Actions - `E`
* Move Forward - `W`
* Move Back - `S`
* Move Left - `A`
* Move Right - `D`

# Style
`arcane energy` - primary resource of the game. Player can use this energy to `runic inscription`.
`arcane magic` - magic of form. (bla bla bla, gravity)
`elemental magic` - magic of element.
`runcic inscription` - a way to interact with `arcane energy` to create magic (interact with enviroment, call magic, etc.).
`equilateral triangle grid` - grid for inscriptions. Player can call this grid via `Inscription` button.

## Environment
Some ancient arena with a lot of traps and obstacles. 

Arena consists of cuboid stone blocks. Every static block can be interacted with. For example, it can be destroyed or moved by some special cast of the player. Portion of blocks infused with arcane magic.

The arena features a mirrored structure on the ceiling, but the layout above doesn't necessarily have to be an exact replica of the ground below. While it maintains overall symmetry, there can be unique elements or variations in the placement of obstacles and shelters, providing diversity and tactical depth to the gameplay. The ceiling structure can be used players with `special` ability to invert gravity and walk on the ceiling.

Example of the arena:
<div style="display: flex;">
    <img src="./image/arena-conсept.png" alt="description" height="400"/>
    <div style="width: 15px;"></div>
    Consept image only for explain mirrored consept of the arena.
    Not for the structure of the arena.
</div>
<br/>
Down structures have one color and ceiling structures have another color. This color is the color of different types of arcane energy.
(for example `purple` and `cyan` for different types of arcane energy that affect on gravity or vice versa) 
When the player changes the gravity, the color of the character changes to the color of the ceiling structure. And when the player returns to the ground, the color of the character changes to the color of the ground structure.

In arcanic block, player can infuse element energy [`fire`, `water`, `air`, `earth`] to change the properties of the block. 

```
???
Gravity force direction depends on plane above the player moving.
If we implement the idea, we'll have to cut polygonal meshes and allow the user to tear pieces out of the terrain instead of blocks.
```

## Characters
All the characters are artificial soul that control some cuboid stone golem, `Guard Soul`. 

```
???
If player dies, the soul leaves the golem and the golem becomes a stone block.
Souls can find another vessel to control.
```

Golems have a `tank` of the arcane energy. This energy is used to `runic inscription`. The tank fills up from environment (surrounding block with arcane magic).  
Arcane energy in the tank have color depending on the type of the energy (gravity direction).

```
???
Block with arcane magic can be deplect. In this case, the block becomes a normal stone block.
what next?
```

Player can inscript rune in `equilateral triangle grid`. Every rune is a one command.
Commands:
* `select` - select target
* `move` - move target
* `trow` - trow target
* `infuse` - infuse elemental energy to target

```
???
Posible inscript behavior into the block / object.
```

## Game Modes

* Two team of the players are fighting each other. / Duel 1v1.
* Company
 


# Systems & Mechanics

## Systems

## Mechanics
### Gravity Inversion
[//]: # (TODO `button` or `runic inscription` ?)

The player can toggle gravity inversion as a `special` ability via the `special` button or `runic inscription` after fulfilling certain conditions. This maneuver allows the player to walk on the ceiling, enabling them to evade dangerous situations or ambush opponents unexpectedly.

# Parameters

# Progression

# Interface