use crate::{player::PlayerId, prelude::*};

pub struct HealthEventsPlugin;

impl Plugin for HealthEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrinkPotionEvent>();
    }
}

#[derive(Event)]
pub struct DrinkPotionEvent {
    pub player: Entity,
    pub player_id: PlayerId,
    pub hp: HealthUnit,
}

impl DrinkPotionEvent {
    pub fn new(player: Entity, player_id: PlayerId, hp: HealthUnit) -> Self {
        Self {
            player,
            player_id,
            hp,
        }
    }
}
