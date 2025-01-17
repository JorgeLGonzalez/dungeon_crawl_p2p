use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Clone, Component, Copy, Debug)]
pub struct Player {
    pub id: PlayerId,
}

pub type PlayerId = usize;
