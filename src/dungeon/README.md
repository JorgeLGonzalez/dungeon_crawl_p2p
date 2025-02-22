# Dungeon

The Dungeon module controls the procedural building (spawning) of the [DungeonMap](./dungeon_map/dungeon_map.rs) based on various strategies. It also has systems for revealing the dungeon in response to the `Reveal DungeonEvent` and handling zooming into and out of the local player's map view. Finally, it implements the [Illuminator](./illuminator.rs) helper to illuminate or darken tiles based on the local player's field of view (FOV).

The key construct is the [DungeonMap](./dungeon_map/dungeon_map.rs), which is stored as a Bevy Resource. It contains all the [DungeonTile](./dungeon_map/dungeon_tile.rs)s of different [TileType](./dungeon_map/dungeon_tile.rs)s as well as:

- Item [DungeonPosition](./dungeon_map/dungeon_position.rs)s.
- Monster starting [DungeonPosition](./dungeon_map/dungeon_position.rs)s.
- Player starting [DungeonPosition](./dungeon_map/dungeon_position.rs)s.

The [DungeonPosition](./dungeon_map/dungeon_position.rs) is the x,y coordinates matching the game world coordinates; meaning the origin is at the center and negative y values run towards the bottom of the viewport and each position is a unit length and contains a tile and potentially an item, player and/or monster. The `DungeonPosition` can be converted to/from Vec2 or Vec3.

The [DungeonTile](./dungeon_map/dungeon_tile.rs) consists of a `DungeonPosition` and a `TileType` at that position.

## Random Rooms Builder

The [RandomRoomsBuilder](./dungeon_map/random_rooms_builder.rs) strategy creates a [DungeonMap](./dungeon_map/dungeon_map.rs) with a configured number of rooms of random sizes (within configured limits) in random locations, ensuring rooms do not overlap. It then builds vertical or horizontal corridors between the rooms.

The players are placed in the centers of the first 2 randomly generated rooms.

Items and monsters are placed in randomly chosen floor tiles that are outside the configured player's SAFETY_RADIUS.

## Cell Automata Builder

The [CellAutomataBuilder](./dungeon_map/cell_automata_builder.rs) strategy first randomizes all tiles between Floor and Wall, slightly favoring floor tiles (55%). It then grows floor or wall segments by setting tiles to floor unless they are surrounded by more than 4 walls or no walls at all.

The first player is placed at the outer edge of a randomly chosen corner. The second player is placed in the opposite corner. (Both are placed in the first floor tile spiraling out from their corner.)

Items and monsters are placed in randomly chosen floor tiles that are outside the configured player's SAFETY_RADIUS.
