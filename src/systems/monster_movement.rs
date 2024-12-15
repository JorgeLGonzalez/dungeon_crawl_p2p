use crate::{
    components::{Monster, WallTile},
    resources::{config, DungeonPosition},
};
use bevy::{
    math::Vec2,
    prelude::{Query, Transform, With, Without},
    utils::hashbrown::HashSet,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

type MonsterQuery<'w, 's, 't> = Query<'w, 's, &'t mut Transform, With<Monster>>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub fn move_monsters(mut monsters: MonsterQuery, wall_tiles: WallQuery) {
    let walls = create_wall_set(&wall_tiles);
    let mut planned = create_current_monster_positions_set(&monsters);
    let mut rng = thread_rng();

    for (mut monster, movement) in monsters
        .iter_mut()
        .filter_map(|m| determine_movement(&mut rng).map(|movement| (m, movement)))
    {
        let pos = DungeonPosition::from_vec2(monster.translation.truncate() + movement);

        if !planned.contains(&pos) && !walls.contains(&pos) {
            planned.remove(&DungeonPosition::from_vec3(monster.translation));
            planned.insert(pos);
            monster.translation = pos.to_vec3(config::MONSTER_Z_LAYER);
        }
    }
}

fn create_wall_set(walls: &WallQuery) -> HashSet<DungeonPosition> {
    HashSet::<DungeonPosition>::from_iter(
        walls
            .iter()
            .map(|w| DungeonPosition::from_vec3(w.translation)),
    )
}

fn create_current_monster_positions_set(monsters: &MonsterQuery) -> HashSet<DungeonPosition> {
    HashSet::<DungeonPosition>::from_iter(
        monsters
            .iter()
            .map(|m| DungeonPosition::from_vec3(m.translation)),
    )
}

fn determine_movement(rng: &mut ThreadRng) -> Option<Vec2> {
    match rng.gen_range(0..50) {
        0 => Some(Vec2::Y),
        1 => Some(Vec2::NEG_Y),
        2 => Some(Vec2::NEG_X),
        3 => Some(Vec2::X),
        _ => None,
    }
}
