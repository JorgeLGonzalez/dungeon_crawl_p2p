mod add_core_systems;
mod debug_ggrs;
mod events;
mod plugin;
mod random_generator;

pub mod fov {
    mod components;
    mod events;
    mod fov_queries;
    mod line_of_sight;
    mod plugin;
    mod recalculate_fov;
    mod visibility_toggler;

    pub use components::{FieldOfView, FovRadius, FovTileMap};
    pub use events::RecalculateFovEvent;
    pub use plugin::{FovCoreSet, FovPlugin};
}

pub mod health {
    mod components;
    mod drink_potion;
    mod events;
    mod healing;
    mod plugin;

    pub use components::{Damage, DamageUnit, Healing, Health, HealthUnit};
    pub use events::DrinkPotionEvent;
    pub use plugin::HealthPlugin;
}

pub use add_core_systems::add_core_systems;
pub use events::{DesyncEvent, SnapshotStateEvent};
pub use plugin::CommonPlugin;
pub use random_generator::{RandomCounter, RandomGenerator};
