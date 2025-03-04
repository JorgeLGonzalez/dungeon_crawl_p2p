//! Blueprints for the dungeon map.
//! Remember to add a unit test for each new blueprint in [`super::PrefabBlueprint`].

/// Prefabricated blueprints for vaults to randomly place in the dungeon.
/// See [`super::BlueprintTile::new`] for what each character represents.
pub(super) const FORTRESS: &str = "
------------
---######---
---#---S#---
---#-O--#---
-###----###-
--M--X---M--
-###----###-
---#----#---
---#P--I#---
---######---
------------
";
