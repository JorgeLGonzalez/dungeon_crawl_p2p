# Dungeon Crawl P2P

Goal is to recreate dungeon crawl in Bevy (using Bevy rendering etc) that works on web and Mac OS and that can be with 2 players. Unclear if it should still be turn-based.

Focusing on generating the map for now. Assume it will become a resource.
But for now, lets generate walls vs floors (just diff color boxes for now).
And lets create the diff room architects. We can add exit and amulet and player.

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
  - [ ] adapt to multi player
- [ ] monster random moves (w/o stepping on other monster)
- [ ] player attack on monster
- [ ] monster attack player
- [ ] monster follows player
- [ ] monster FOV
- [ ] player FOV
- [ ] other dungeons
- [ ] deploy such that it can be used across devices (ideally over internet)

- [ ] worth looking at [Leafwing input manager](https://github.com/Leafwing-Studios/leafwing-input-manager) for keyboard input handling (and mouse)
- [ ] spawn_dungeon: should use insert_batch as that is more efficient
- [ ] how to enable trace logging only for my app (or per module)

## Issues

- [x] synctest mode does not work. It spawns the 2 players, but they are not seen in the GgrsSchedule systems (move_players and camera_follow). Solved by starting sync test when entering the MatchMaking state rather than in the Update. See commit 6dfacc59a686f72e3ac49ac957130c72407bb7f0.
- [ ] matchbox does not work with bevy 0.15, unless using fork from haihala. Hope to get revised bevy_ggrs and matchbox once [PRs](https://github.com/johanhelsing/matchbox/pull/466) merged
- [ ] Browser tab title

## Rollbacks

GGRS depends on rollbacks, which I don't fully understand. We need to do at least two things:

1. Call `add_rollback` on the spawn command for each entity bundle that may need rollbacks. This adds the Bevy GGRS Rollback component.
2. Register specific components for rollback with a specific strategy (clone or copy) in `main` via the `rollback_component_with_clone` or `rollback_component_with_copy` methods.

So for `Player` for example (in [spawn_players](./src/systems/spawn_players.rs)) we use `add_rollback` and in main we register `Player` and `Transform` for rollback. We probably will need to add other Components that are added to Player when we add Sprite etc, especially if a Player will be despawned. `PlayerMovement`, for example, might need registration. But basic testing does not reveal that.

See [Extreme Bevy Detecting Desyncs tutorial](https://johanhelsing.studio/posts/extreme-bevy-desync-detection) for more info

Entities Spawned and Rollback Info

- `spawn_players`. With rollback.
- `spawn_camera`. No rollback because it only follows the local player.
- `spawn_dungeon`. Tiles have no rollback as they are only generated at the start of each dungeon level and we rely on the shared random seed to have both players get the same dungeon.

Resources do not need rollback???

### Checksums

The rollback depends on checksums. Transform is a special case where we need to create a custom checksum, which is why we have the `checksum_component` method in `main`.

## References

The P2P stuff (among other things) is based on the [Extreme Bevy tutorial](https://johanhelsing.studio/posts/extreme-bevy)
