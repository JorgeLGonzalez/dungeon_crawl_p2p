use crate::components::{FieldOfView, LastAction, Monster, Player, PlayerId, WallTile};
use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

pub type MonsterQuery<'w, 's, 't, 'f, 'a> =
    Query<'w, 's, (&'t Transform, &'f FieldOfView, &'a LastAction, Entity), With<Monster>>;
pub type PlayersQuery<'w, 's, 't, 'p> =
    Query<'w, 's, (&'t Transform, Entity, &'p Player), (With<Player>, Without<Monster>)>;
pub type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub type MonsterPositionSet = HashSet<IVec2>;
pub type PlayerPositionMap = HashMap<IVec2, (Entity, PlayerId)>;
pub type WallPositionSet = HashSet<IVec2>;

pub struct MonsterActionParams {
    pub monsters: MonsterPositionSet,
    pub players: PlayerPositionMap,
    pub walls: WallPositionSet,
}

impl MonsterActionParams {
    pub fn new(monsters: &MonsterQuery, players: &PlayersQuery, walls: &WallQuery) -> Self {
        Self {
            monsters: Self::create_current_monster_positions_set(monsters),
            players: Self::create_player_set(players),
            walls: Self::create_wall_set(walls),
        }
    }

    fn create_current_monster_positions_set(monsters: &MonsterQuery) -> MonsterPositionSet {
        MonsterPositionSet::from_iter(
            monsters
                .iter()
                .map(|(m, ..)| m.translation.truncate().as_ivec2()),
        )
    }

    fn create_player_set(players: &PlayersQuery) -> PlayerPositionMap {
        PlayerPositionMap::from_iter(players.iter().map(|(p, player_entity, player)| {
            (
                p.translation.truncate().as_ivec2(),
                (player_entity, player.id),
            )
        }))
    }

    fn create_wall_set(walls: &WallQuery) -> WallPositionSet {
        WallPositionSet::from_iter(walls.iter().map(|w| w.translation.truncate().as_ivec2()))
    }
}
