# Dungeon

The Dungeon module controls the procedural building (spawning) of the [DungeonMap](./dungeon_map/dungeon_map.rs) based on various strategies. It also has systems for revealing the dungeon in response to the `RevealDungeonEvent` and handling zooming into and out of the local player's map view. Finally, it implements the [Illuminator](./illuminator.rs) helper to illuminate or darken tiles based on the local player's field of view (FOV).

The key construct is the [DungeonMap](./dungeon_map/dungeon_map.rs), which is stored as a Bevy Resource. It contains all the [DungeonTile](./dungeon_map/dungeon_tile.rs)s of different [TileType](./dungeon_map/dungeon_tile.rs)s as well as:

- Item [DungeonPosition](./dungeon_map/dungeon_position.rs)s.
- Monster starting [DungeonPosition](./dungeon_map/dungeon_position.rs)s.
- Player starting [DungeonPosition](./dungeon_map/dungeon_position.rs)s.

The [DungeonPosition](./dungeon_map/dungeon_position.rs) is the x,y coordinates matching the game world coordinates; meaning the origin is at the center and negative y values run towards the bottom of the viewport and each position is a unit length and contains a tile and potentially an item, player and/or monster. The `DungeonPosition` can be converted to/from Vec2 or Vec3.

The [DungeonTile](./dungeon_map/dungeon_tile.rs) consists of a `DungeonPosition` and a `TileType` at that position.

Below are the 3 strategies currently in use as well as a "prefab vault" feature that allows for adding hand-crafted areas (vaults) to a random location of the dungeon.

## Cell Automata Builder

The [CellAutomataBuilder](./dungeon_map/cell_automata/cell_automata_builder.rs) strategy first randomizes all tiles between Floor and Wall, slightly favoring floor tiles (55%). It then grows floor or wall segments by setting tiles to floor unless they are surrounded by more than 4 walls or no walls at all.

The first player is placed at the outer edge of a randomly chosen quadrant. The second player is placed in the opposite quadrant. (Both are placed in the first floor tile spiraling out from the corner edge of the quadrant.)

In some dungeons, a quadrant may be cut-off from the bulk of the dungeon. To make sure a player is not in one of those quadrants, we check that a path exists to the center-most dungeon floor tile. If no path is found, we create a simple L-shaped tunnel from the closest tile found by the path-finding algorithm. (See [ReachabilityEnsurer](./dungeon_map/reachability/reachability_ensurer.rs))

Items and monsters are placed in randomly chosen floor tiles that are outside the configured player's SAFETY_RADIUS.

## Drunkards Walk

The [DrunkardsWalkBuilder](./dungeon_map/drunkards_walk/drunkards_walk_builder.rs) strategy first adds the player(s) and then starts creating "drunkard" tunnels from the dungeon center and each player starting position. A drunkard walk is simply taking a step in a random direction until we hit the edge or have staggered enough steps. The process is repeated until there are enough floor tiles in the dungeon. Then we make sure the players can reach the center (tunneling if needed), using the [ReachabilityEnsurer](./dungeon_map/reachability/reachability_ensurer.rs) the same we use it in the `CellAutomataBuilder`. Finally, we add items and monsters as with other strategies.

## Random Rooms Builder

The [RandomRoomsBuilder](./dungeon_map/random_rooms/random_rooms_builder.rs) strategy creates a [DungeonMap](./dungeon_map/dungeon_map.rs) with a configured number of rooms of random sizes (within configured limits) in random locations, ensuring rooms do not overlap. It then builds vertical or horizontal corridors between the rooms.

The players are placed in the centers of the first 2 randomly generated rooms.

Items and monsters are placed in randomly chosen floor tiles that are outside the configured player's SAFETY_RADIUS.

## Prefab Vault

Rather than a procedural dungeon building strategy, this provides a way to define a "vault" (i.e. an area of dungeon) via a blueprint and then building that in a random location of the dungeon, if possible. So we do this for all the strategies via the [PrefabVault](./dungeon_map/prefab/prefab_vault.rs).

Right now there is only one blueprint, the `Fortress` (taken from the book). But it would be easy to add a few more and then randomly choose. The [blueprints](./dungeon_map/prefab/blueprints.rs) are simply a string with character symbols that specify what each tile should have. It would also be easy to have these strings in files to keep adding them, though for now I hardcoded them in the [PrefabBlueprint](./dungeon_map/prefab/prefab_blueprint.rs) smart enum.

The [PrefabVault](./dungeon_map/prefab/prefab_vault.rs) does the following:

1. Uses the [VaultSiteSelector](./dungeon_map/prefab/site_selector.rs) to find a random spot in the dungeon to build the vault. It avoids locations too close to players and excludes the dungeon center.
2. Clears any items or monsters from the construction site.
3. Applies each [BlueprintTile](./dungeon_map/prefab/blueprint_tile.rs) to the vault location in the dungeon.
4. Finally, uses the [ReachabilityEnsurer](./dungeon_map/reachability/reachability_ensurer.rs) to ensure all players can reach the vault.
