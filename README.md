# Dungeon Crawl P2P

Goal is to recreate dungeon crawl in Bevy (using Bevy rendering etc) that works on web and Mac OS and that can be with 2 players. Unclear if it should still be turn-based.

Focusing on generating the map for now. Assume it will become a resource.
But for now, lets generate walls vs floors (just diff color boxes for now).
And lets create the diff room architects. We can add exit and amulet and player.

- [ ] convert DungeonPos to Vec2 or Vec3
- [ ] tunnel between rooms
- [ ] camera should have its own system
- [ ] Create Sprite from DungeonTile
- [ ] how to enable trace logging only for my app (or per module)

## Issues

- [ ] Browser tab title
