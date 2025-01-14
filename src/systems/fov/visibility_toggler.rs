use crate::dungeon::FloorTile;
use bevy::{prelude::*, utils::hashbrown::HashSet};

pub type VisibilityQuery<'w, 's, 't, 'v> =
    Query<'w, 's, (&'t Transform, &'v mut Visibility), Without<FloorTile>>;

/// Toggle the visibility of entities based on their position in the player's FOV.
pub struct VisibilityToggler {
    mover: Entity,
    local_player_fov: HashSet<IVec2>,
}

impl VisibilityToggler {
    pub fn new(mover: Entity, local_player_fov: HashSet<IVec2>) -> Self {
        Self {
            mover,
            local_player_fov,
        }
    }

    /// Toggle the visibility of entities based on their position in the local
    /// player's FOV. If the trigger was caused by the local player moving, check
    /// all other relevant entities. Otherwise, we only need to check the entity
    /// that moved.
    pub fn toggle(&self, mover_is_local_player: bool, entities: &mut VisibilityQuery) {
        if mover_is_local_player {
            entities.iter_mut().for_each(|(transform, mut visibility)| {
                self.toggle_if_needed(transform, &mut visibility);
            });
        } else {
            let (transform, mut visibility) = entities.get_mut(self.mover).expect("Inconceivable!");
            self.toggle_if_needed(transform, &mut visibility);
        }
    }

    /// Change the entity's (e.g. monster, remote player) visibility based on
    /// the local player's FOV
    fn toggle_if_needed(&self, entity_transform: &Transform, visibility: &mut Visibility) {
        let entity_pos = entity_transform.translation.truncate().as_ivec2();
        let expected_visibility = match self.local_player_fov.contains(&entity_pos) {
            false => Visibility::Hidden,
            true => Visibility::Visible,
        };

        if *visibility != expected_visibility {
            visibility.toggle_visible_hidden();
        }
    }
}
