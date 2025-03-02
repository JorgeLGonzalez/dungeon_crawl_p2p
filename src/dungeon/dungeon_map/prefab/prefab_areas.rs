#[derive(Debug)]
pub enum PrefabBlueprint {
    Fortress,
}

impl PrefabBlueprint {
    pub fn blueprint(&self) -> &'static str {
        match self {
            PrefabBlueprint::Fortress => &FORTRESS,
        }
    }
}

/*
LEGEND
======
- Floor
# Wall
I Item
M Random Monster
O Ogre
S Huge sword
P Magic Map
X Key marker location that must be reachable by players
*/

const FORTRESS: &str = "
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
