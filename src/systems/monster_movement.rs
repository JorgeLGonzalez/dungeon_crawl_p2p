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

pub fn move_monsters(
    mut monsters: Query<&mut Transform, With<Monster>>,
    wall_tiles: Query<&Transform, (With<WallTile>, Without<Monster>)>,
) {
    let mut rng = thread_rng();
    let walls = HashSet::<DungeonPosition>::from_iter(
        wall_tiles
            .iter()
            .map(|w| DungeonPosition::from_vec3(w.translation)),
    );
    let mut planned = HashSet::<DungeonPosition>::from_iter(
        monsters
            .iter()
            .map(|m| DungeonPosition::from_vec3(m.translation)),
    );

    for mut monster in &mut monsters {
        if let Some(movement) = determine_movement(&mut rng) {
            let pos = DungeonPosition::from_vec2(monster.translation.truncate() + movement);

            if !planned.contains(&pos) && !walls.contains(&pos) {
                planned.remove(&DungeonPosition::from_vec3(monster.translation));
                monster.translation = pos.to_vec3(config::MONSTER_Z_LAYER);
                planned.insert(pos);
            }
        }
    }
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
