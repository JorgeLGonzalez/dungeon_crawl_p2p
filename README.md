# Dungeon Crawl **P2P**

Goal is to recreate dungeon crawl in Bevy (using Bevy rendering etc) that works on web and Mac OS and that can be with 2 players. Unclear if it should still be turn-based.

Focusing on generating the map for now. Assume it will become a resource.
But for now, lets generate walls vs floors (just diff color boxes for now).
And lets create the diff room architects. We can add exit and amulet and player.

- [ ] healing potions
  - [x] toggle visibility. It just worked! Nice!
- [ ] inventory and item usage
- [ ] data driven dungeon monsters and items
- [ ] weapons
- [ ] deploy such that it can be used across devices (ideally over internet)
- [ ] cellular automata dungeon generation
  - [ ] spawn_dungeon: should use insert_batch as that is more efficient
- [ ] drunkard walk dungeon generation
- [ ] prefab dungeon sections
- [ ] stairs and dungeon levels
  - [ ] show level in HUD
- [ ] player sprites
- [ ] monster sprites
- [ ] tile sprites
  - [ ] note wall tiles are currently always hidden. will need to adjust `recalculate_fov`
- [ ] improve lighting simulation
- [ ] amulet. In original it is placed farthest from player. But we now have 2 randomly placed players.
  - [ ] Game won state and screen
- [ ] monsters now move intentionally, but still not very smart. If they see any players, they will hone in on the closest one and take any valid step that brings them closer to that player, but it is a shortsighted strategy since the distance to the player does not account for any obstacles, so there's probably situations where going to a farther player would be better or where stepping further first is a better path. In other words, there's no path-finding like Dikjstra path. They also have no memory so they will stop chasing a player who escapes their FOV like just around a corner. See [Dijkstra Maps](https://www.roguebasin.com/index.php/The_Incredible_Power_of_Dijkstra_Maps) for an implementation of the algorithm in the context of a rogue-like dungeon crawler. I think this is what the Hands-On Rust book used.
- [ ] restart game on game over key press. And actual game over systems and display
- [ ] dungeon themes
- [ ] score points for defeating monsters, picking up coins ?
- [ ] monster patrol strategies (explore, guard, rest)
- [ ] worth looking at [Leafwing input manager](https://github.com/Leafwing-Studios/leafwing-input-manager) for keyboard input handling (and mouse)

## Issues

- [ ] Browser tab title

## Main Modules

- `common`: Functionality shared among multiple other modules:
  - `fov`: Field of view calculations
  - `health`: Health, healing, damage
  - `random_generator`: Random number generation
  - `events`: Events not clearly associated with any other module (e.g. DesyncEvent, SnapshotStateEvent)
- `dungeon`: Dungeon generation and map
- `game_states`: The GameState enum and the game_over system
- [HUD](./src/hud/README.md): Heads-up display, including health bar and tooltips
- `monsters`: Monsters and their actions
- [Player](./src/player/README.md).
- [startup](./src/startup/README.md), including GGRS for multiplayer

### System Sequencing

The `GameState::InGame` is by far the most complex and is handled differently in `GameMode::SinglePlayer` vs `GameMode::P2P` (or `GameMode::SyncTest`). In single player mode, the systems run in the `Update` schedule while in P2P mode, they run in the `GgrsSchedule` schedule (plus a special ReadInputs schedule for handling inputs from both the local and remote players).

The sequencing is also non-trivial and diagrammed below:

```mermaid
flowchart TD
  Health-- before -->Player
  Player-- before -->Monsters
  Monsters-- before -->HUD
  Monsters-- before -->FOV
```

- Health controls the healing of players and monsters so we calculate before any combat.
- Player systems run before monsters to give them a theoretical advantage.
- Monster systems run before HUD and FOV.
- HUD and FOV run last can can run in parallel, but they must follow monster, player and health systems. FOV is affected by player and monster movements. HUD is affected by player health and actions.

- Dungeon systems just control the dungeon reveal and map zoom level so they can run in parallel with everything else.

## Archived TODO

- [x] simplify player inputs
- [x] re-enable snapshots
- [x] should we add/remove PlayerMovement component instead? More generally, the throttle is weird as it is only reset when there is a move. it should reset also when key is released?
- [x] simplify intersects on player moves. check against monster moves
- [x] set game mode to p2p for wasm by default and sp otherwise
- [x] monster attack player. For now, it just transitions to GameOver which simply logs out game over. What should happen in p2p mode? Respawn dead player?
- [x] Maintain single list of systems for player/monster actions. Not sure how best to do this. Something like defining an array/tuple of the set of systems that are the same between GgrsSchedule and Update. Quick dialog w/ copilot suggests a macro. But it is ugly. Perhaps combine a single do_player_action which internally branches based on GAME_MODE? This way the tuple is the same and can be defined as a var that can be used in both places, if the borrow checker allows? Or maybe have an add_systems that selects the GgrsSchedule vs Update based on GAME_MODE?
- [x] prevent players from moving onto each other (no attacks yet)
- [x] Health component and give player 10
  - [x] Elapsed time healing
  - [x] synctest.
- [x] health bar
- [x] player FOV
- [ ] monster follows player
  - [x] FOV contains hash set of positions (or hashmap of position to tile entity)
  - [x] monster FOV
  - [x] MonsterActionDeterminer.plan_move. move towards any player it can see, otherwise move randomly
    - [x] Slow down the monster move to attack
    - [x] chase move needs to avoid invalid moves
- [x] deal monster-based damage to players
- [x] heal monsters
- [x] hide unexplored dungeon
- [x] hide monsters outside FOV
- [x] how to enable trace logging only for my app (or per module)
- [x] tunnel between rooms
- [x] zoom in/out in dungeon or scroll (or both)
- [x] camera should have its own system
- [x] set up p2p
- [x] spawn player
  - [x] players can spawn off map. Probably rooms can be off map
- [x] player input
  - [x] collision detection
- [x] camera follows player
- [x] May need to allow finer movement adjustments if player is as wide as a tile so they can easily enter corridors
- [x] GGRS sync stuff
  - [x] Event handling
  - [x] Ensure we have rollbacks registered
- [x] single player mode
- [x] spawn monsters
- [x] monster random moves (w/o stepping on other monster)
  - [x] adapt to multi player
- [x] player attack on monster
  - [x] adapt to multi player
  - [x] getting desyncs. Note there is a bug in desync detection, but I am pretty sure I have my own bugs because I see visually things going out of sync. It happens even if i do not despawn monsters. Even registering Time does not solve it. Hmm...
- [x] text on health bar
  - [x] health to left
  - [x] health points x/10 to right
  - [x] update health points text
  - [x] use custom font on both web and mac
  - [x] re-arrange to have parent container with text and bars as children
  - [x] center the whole thing
  - [x] experiment using UI instead of sprites/transforms
- [x] tool tips to show monster name and health
  - [x] only show when monster is in player's FOV
  - [x] Do we want to show current health on monster or max? Showing max for now
- [x] map revealer debug key
- [x] zoom in/out
  - [x] HUD should not change. Does this mean a different camera or?
- [x] only show other player when in FOV
- [x] reorg game project to be feature based. core, player, monster, dungeon, etc...
  - [x] Look into plugins. Probably each top module should be a plugin that adds systems, events etc
- [x] reorg cleanup
  - [x] events plugin for all relevant modules in events mod
  - [x] system sets x-ref to each other
  - [x] Helper for core_systems

### Archived Issues

- [x] synctest mode does not work. It spawns the 2 players, but they are not seen in the GgrsSchedule systems (move_players and camera_follow). Solved by starting sync test when entering the MatchMaking state rather than in the Update. See commit 6dfacc59a686f72e3ac49ac957130c72407bb7f0.
- [x] matchbox does not work with bevy 0.15, unless using fork from haihala. Hope to get revised bevy_ggrs and matchbox once [PRs](https://github.com/johanhelsing/matchbox/pull/466) merged
