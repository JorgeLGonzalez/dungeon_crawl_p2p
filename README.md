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
- [ ] May need to allow finer movement adjustments if player is as wide as a tile so they can easily enter corridors
- [ ] GGRS sync stuff
  - [ ] Event handling
  - [ ] Ensure we have rollbacks registered
- [ ] single player mode
- [ ] other dungeons
- [ ] deploy such that it can be used across devices (ideally over internet)

- [ ] spawn_dungeon: should use insert_batch as that is more efficient
- [ ] how to enable trace logging only for my app (or per module)

## Issues

- [ ] synctest mode does not work. It spawns the 2 players, but they are not seen in the GgrsSchedule systems (move_players and camera_follow).
- [ ] matchbox does not work with bevy 0.15, unless using fork from haihala. Hope to get revised bevy_ggrs and matchbox once [PRs](https://github.com/johanhelsing/matchbox/pull/466) merged
- [ ] Browser tab title

## References

The P2P stuff (among other things) is based on the [Extreme Bevy tutorial](https://johanhelsing.studio/posts/extreme-bevy)
