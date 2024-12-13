use crate::{
    components::{MoveDir, Player, WallTile},
    resources::{
        calculate_direction,
        config::{
            GgrsSessionConfig, MAP_HEIGHT, MAP_WIDTH, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH,
            TILE_HEIGHT, TILE_WIDTH,
        },
    },
};
use bevy::{
    log::info,
    math::Vec2,
    prelude::{Query, Res, Transform, With, Without},
    time::Time,
};
use bevy_ggrs::PlayerInputs;

pub fn move_players(
    mut players: Query<(&mut Transform, &mut MoveDir, &Player), With<Player>>,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
    time: Res<Time>,
    walls: Query<&Transform, (With<WallTile>, Without<Player>)>,
) {
    for (mut transform, mut move_dir, player) in &mut players {
        let old_pos = transform.translation.truncate();
        let input = inputs[player.id].0;
        let elapsed_secs = time.delta_secs();

        if let Some(direction) = calculate_direction(input) {
            move_dir.0 = direction;

            let pos = calculate_pos(old_pos, direction, elapsed_secs);
            info!(
                "Player {} moves {:?} from {:?} to {:?}",
                player.id,
                pos - old_pos,
                old_pos,
                pos
            );

            let hit_wall = walls.iter().any(|wall_transform| {
                if intersects(&pos, wall_transform) {
                    info!("Hit wall");
                    return true;
                }
                return false;
            });

            if !hit_wall {
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
        }
    }
}

fn calculate_pos(old_pos: Vec2, direction: Vec2, delta_seconds: f32) -> Vec2 {
    static MIN: Vec2 = Vec2::new(MAP_WIDTH as f32 / 2., MAP_HEIGHT as f32 / 2.);
    static MAX: Vec2 = Vec2::new(
        MAP_WIDTH as f32 / 2. - PLAYER_WIDTH,
        MAP_HEIGHT as f32 / 2. - PLAYER_HEIGHT,
    );

    let move_delta = direction * PLAYER_SPEED * delta_seconds;

    (old_pos + move_delta).clamp(-MIN, MAX)
}

fn intersects(player: &Vec2, wall: &Transform) -> bool {
    static PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
    static WALL_SIZE: Vec2 = Vec2::new(TILE_WIDTH, TILE_HEIGHT);

    let player_min = player - PLAYER_SIZE / 2.0;
    let player_max = player + PLAYER_SIZE / 2.0;
    let wall_min = wall.translation.truncate() - WALL_SIZE / 2.0;
    let wall_max = wall.translation.truncate() + WALL_SIZE / 2.0;

    player_min.x < wall_max.x
        && player_max.x > wall_min.x
        && player_min.y < wall_max.y
        && player_max.y > wall_min.y
}
