# Dungeon Crawl P2P

Goal is to recreate dungeon crawl in Bevy (using Bevy rendering etc) that works on web and Mac OS and that can be with 2 players. Unclear if it should still be turn-based.

Focusing on generating the map for now. Assume it will become a resource.
But for now, lets generate walls vs floors (just diff color boxes for now).
And lets create the diff room architects. We can add exit and amulet and player.

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
- [ ] monsters now move intentionally, but still not very smart. If they see any players, they will hone in on the closest one and take any valid step that brings them closer to that player, but it is a shortsighted strategy since the distance to the player does not account for any obstacles, so there's probably situations where going to a farther player would be better or where stepping further first is a better path. In other words, there's no path-finding like Dikjstra path. They also have no memory so they will stop chasing a player who escapes their FOV like just around a corner.
- [ ] reorg game project to be feature based. core, player, monster, dungeon, etc...
- [ ] restart game on game over key press
- [ ] other dungeons
  - [ ] spawn_dungeon: should use insert_batch as that is more efficient
- [ ] deploy such that it can be used across devices (ideally over internet)
- [ ] worth looking at [Leafwing input manager](https://github.com/Leafwing-Studios/leafwing-input-manager) for keyboard input handling (and mouse)
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

## Issues

- [x] synctest mode does not work. It spawns the 2 players, but they are not seen in the GgrsSchedule systems (move_players and camera_follow). Solved by starting sync test when entering the MatchMaking state rather than in the Update. See commit 6dfacc59a686f72e3ac49ac957130c72407bb7f0.
- [x] matchbox does not work with bevy 0.15, unless using fork from haihala. Hope to get revised bevy_ggrs and matchbox once [PRs](https://github.com/johanhelsing/matchbox/pull/466) merged
- [ ] Browser tab title

## Player actions

See [Player Actions](./src/systems/player_actions/README.md).

## Rollbacks

GGRS depends on rollbacks. We need to do at least two things:

1. Call `add_rollback` on the spawn command for each entity bundle that may need rollbacks. This adds the Bevy GGRS `Rollback` component.
2. Register specific components for rollback snapshots with a specific strategy (clone or copy) in `main` via the `rollback_component_with_clone` or `rollback_component_with_copy` methods. This tells Bevy GGRS to store a snapshot of these for every frame (discarding them as they lose utility).
3. If the resource or component is critical to the state, add it to the frame checksum via the `checksum_component` or `checksum_resource` App method. This way it will be included in the frame checksum used to detect if clients fall out of sync.

So for `Player` for example (in [spawn_players](./src/systems/spawn_players.rs)) we use `add_rollback` and in main we register `Player` and `Transform` for rollback. We probably will need to add other Components that are added to Player when we add Sprite etc, especially if a Player will be despawned.

See [Extreme Bevy Detecting Desyncs tutorial](https://johanhelsing.studio/posts/extreme-bevy-desync-detection) for more info

Entities Spawned and Rollback Info

- `spawn_players`. With rollback.
- `spawn_camera`. No rollback because it only follows the local player.
- `spawn_dungeon`. Tiles have no rollback as they are only generated at the start of each dungeon level and we rely on the shared random seed to have both players get the same dungeon.

### Time

Note that `bevy_ggrs` replaces the `Time` resource with one that is kept in sync.

### Checksums

The rollback depends on checksums. Transform is a special case where we need to create a custom checksum, which is why we have the `checksum_component` method in `main`. So is `RandomGenerator`. Note that GGRS will not complain about rollback components or resources that cannot be hashed. I guess they will just not be checked to ensure they are in sync. (But I could be wrong...)

## Understanding GGRS

Some key notes:

1. The game must be deterministic. For example, the random number generator must start with the same seed for each player. This way all we need to worry about is sending the player inputs to each client.
2. When the inputs fail to match the GGRS predictions, it will rollback components and resources. These mus be registered in two ways a) the `add_rollback` method when spawning the entity will add a `Rollback` component, which is what `bevy_ggrs` uses to know what needs rolling back. b) the `rollback_component_*` and `rollback_resource_*` methods on the App will create `GgrsSnapshots` resources for each registered Type. This keeps track of snapshots per frame (a short set of frames). The methods indicate the strategy for saving/loading snapshots (clone, copy).
3. Adding a checksum on a component or resource ensures it is part of the overall frame checksum used to ensure state is un sync. So basically, (I believe) the system gathers all entities that have a `Rollback` component and then adds any components on these entities to the state snapshots it keeps for each component/resource. Finally, it any components/resources with a checksum method are included in the overall checksum used to detect desync.
4. `bevy_ggrs` sits on top of `ggrs` and `ggrs` is an implementation of GGPO in Rust.

### Troubleshooting Desyncs

There are two ways to auto-detect desync events:

1. Handle the `GgrsEvent::DesyncDetected`, which is generated when `DesyncDetection` is enabled in the P2P session. Enabling happens in `create_p2p_session` while handling happens in `handle_ggrs_events`. The handling tries to log the snapshots, but see [this issue](https://github.com/gschup/bevy_ggrs/issues/117) I logged.
2. Run the app in `synctest` mode. The problem here is that no event is generated so all you get is the fact that a desync happened when logged out. Not details.
3. Enable Trace and/or Debug level logging (via the bevy App LogPlugin config) as bevy_ggrs logs useful stuff out. (Can do RUST_DEBUG env var, but that gets SUPER noisy).

Despite the above, it took me over a week to figure out why the monster positions (Transforms) were getting out of sync. It was clear it was the monsters, but unclear why. I eventually theorized it was because the Bevy queries do not return entities in the same order every single time, which means one client can iterate through the list of monsters and move monster A while the other client picks monster B. Same move, different monster. (I could see this happening once I started tracking, logging and saving monster moves.)
(At least I HOPE that solved the bug!)

## References

The P2P stuff (among other things) is based on the [Extreme Bevy tutorial](https://johanhelsing.studio/posts/extreme-bevy)
