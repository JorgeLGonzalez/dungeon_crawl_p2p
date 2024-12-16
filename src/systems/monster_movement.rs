use crate::{
    components::{Monster, WallTile},
    resources::{config, DungeonPosition, RandomGenerator},
};
use bevy::{
    math::Vec2,
    prelude::{Query, ResMut, Transform, With, Without},
    utils::hashbrown::HashSet,
};

type MonsterQuery<'w, 's, 't> = Query<'w, 's, &'t mut Transform, With<Monster>>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub fn move_monsters(
    mut monsters: MonsterQuery,
    mut rng: ResMut<RandomGenerator>,
    wall_tiles: WallQuery,
) {
    let walls = create_wall_set(&wall_tiles);
    let mut planned = create_current_monster_positions_set(&monsters);

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

fn determine_movement(rng: &mut RandomGenerator) -> Option<Vec2> {
    match rng.gen_range(0..config::MONSTER_FRAMES_PER_MOVE) {
        0 => Some(Vec2::Y),
        1 => Some(Vec2::NEG_Y),
        2 => Some(Vec2::NEG_X),
        3 => Some(Vec2::X),
        _ => None,
    }
}
