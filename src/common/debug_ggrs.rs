use crate::{
    dungeon::{DungeonPosition, RespawnState},
    items::MagicItem,
    monsters::Monster,
    player::{MoveThrottle, PlayerId},
    prelude::*,
    startup::checksum_transform,
};
use bevy_ggrs::RollbackFrameCount;

/// Log debugging info for GGRS. Note some of these lines (i.e. those that contain
/// a '|') are parsed by the `ggrs-utils` project/program.
pub fn debug_ggrs(
    items_q: Query<(Entity, &MagicItem, &Transform), With<MagicItem>>,
    monsters: Query<(Entity, &Health, &Monster, &Transform), With<Monster>>,
    players: Query<(Entity, &Health, Option<&MoveThrottle>, &Player, &Transform), With<Player>>,
    frame: Res<RollbackFrameCount>,
    respawn: Res<RespawnState>,
    rng: Res<RandomGenerator>,
    state: Res<State<GameState>>,
) {
    let frame = frame.0;

    info!(
        "Frame={frame} State={state:?} Respawn={respawn:?} rng={}",
        rng.counter
    );

    let mut sorted_items: Vec<_> = items_q.iter().map(ItemLogger::new).collect();
    sorted_items.sort_by_key(|i| i.sort_key());
    info!("Frame={frame} Total Items={}", sorted_items.len());
    for item in sorted_items {
        item.log(frame);
    }

    let mut sorted_players: Vec<_> = players.iter().map(PlayerLogger::new).collect();
    sorted_players.sort_by_key(|p| p.sort_key());
    info!("Frame={frame} Total Players={}", sorted_players.len());
    for player in sorted_players {
        player.log(frame);
    }

    let mut sorted_monsters: Vec<_> = monsters.iter().map(MonsterLogger::new).collect();
    sorted_monsters.sort_by_key(|m| m.sort_key());
    info!("Frame={frame} Total Monsters={}", sorted_monsters.len());
    for monster in sorted_monsters {
        monster.log(frame);
    }
}

struct ItemLogger {
    entity: Entity,
    item: String,
    pos: IVec2,
    transform_check: u64,
}

impl ItemLogger {
    pub fn new((entity, item, transform): (Entity, &MagicItem, &Transform)) -> Self {
        Self {
            entity,
            item: item.label(),
            pos: transform.translation.truncate().as_ivec2(),
            transform_check: checksum_transform(transform),
        }
    }

    pub fn log(&self, frame: i32) {
        let Self {
            entity,
            item,
            pos,
            transform_check,
        } = self;
        info!(
            "Frame={frame}|Pos={pos}|Transform={transform_check:X}|\
            Entity={entity}|Item={item}"
        );
    }

    pub fn sort_key(&self) -> String {
        let entity = self.entity;
        let pos_idx = DungeonPosition::from_ivec2(self.pos).to_index();
        format!("{pos_idx}_{entity}")
    }
}

struct PlayerLogger {
    entity: Entity,
    player_id: PlayerId,
    pos: IVec2,
    health: HealthUnit,
    throttle: f32,
    transform_check: u64,
}

impl PlayerLogger {
    pub fn new(
        (entity, health, move_throttle, player, transform): (
            Entity,
            &Health,
            Option<&MoveThrottle>,
            &Player,
            &Transform,
        ),
    ) -> Self {
        Self {
            entity,
            player_id: player.id,
            pos: transform.translation.truncate().as_ivec2(),
            health: health.current,
            throttle: move_throttle.map(|t| t.elapsed_secs()).unwrap_or(0.0),
            transform_check: checksum_transform(transform),
        }
    }

    pub fn log(&self, frame: i32) {
        let Self {
            entity,
            player_id,
            pos,
            health,
            throttle,
            transform_check,
        } = self;
        info!(
            "Frame={frame}|Player={player_id}|Pos={pos}|\
            Transform={transform_check:X}|Entity={entity}|Health={health}|\
            Throttle={throttle}"
        );
    }

    pub fn sort_key(&self) -> PlayerId {
        self.player_id
    }
}

struct MonsterLogger {
    entity: Entity,
    monster: String,
    pos: IVec2,
    health: HealthUnit,
    transform_check: u64,
}

impl MonsterLogger {
    pub fn new(
        (entity, health, monster, transform): (Entity, &Health, &Monster, &Transform),
    ) -> Self {
        Self {
            entity,
            monster: monster.name().to_string(),
            pos: transform.translation.truncate().as_ivec2(),
            health: health.current,
            transform_check: checksum_transform(transform),
        }
    }

    pub fn log(&self, frame: i32) {
        let Self {
            entity,
            monster,
            pos,
            health,
            transform_check,
        } = self;
        info!(
            "Frame={frame}|Pos={pos}|Transform={transform_check:X}|\
            Entity={entity}|Monster={monster}|Health={health}"
        );
    }

    pub fn sort_key(&self) -> String {
        let entity = self.entity;
        let pos_idx = DungeonPosition::from_ivec2(self.pos).to_index();
        format!("{pos_idx}_{entity}")
    }
}
