use crate::{
    components::{MoveDir, Player},
    resources::{
        calculate_direction,
        config::{GgrsSessionConfig, MAP_HEIGHT, MAP_WIDTH, PLAYER_SPEED},
    },
};
use bevy::{
    math::Vec2,
    prelude::{Query, Res, Transform},
    time::Time,
};
use bevy_ggrs::PlayerInputs;

pub fn move_players(
    mut players: Query<(&mut Transform, &mut MoveDir, &Player)>,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
    time: Res<Time>,
) {
    for (mut transform, mut move_dir, player) in &mut players {
        let old_pos = transform.translation.truncate();
        let input = inputs[player.id].0;
        let elapsed_secs = time.delta_secs();

        if let Some(direction) = calculate_direction(input) {
            move_dir.0 = direction;

            let pos = calculate_pos(old_pos, direction, elapsed_secs);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

fn calculate_pos(old_pos: Vec2, direction: Vec2, delta_seconds: f32) -> Vec2 {
    static LIMIT: Vec2 = Vec2::new(MAP_WIDTH as f32 / 2. - 0.5, MAP_HEIGHT as f32 / 2. - 0.5);

    let move_delta = direction * PLAYER_SPEED * delta_seconds;

    (old_pos + move_delta).clamp(-LIMIT, LIMIT)
}
