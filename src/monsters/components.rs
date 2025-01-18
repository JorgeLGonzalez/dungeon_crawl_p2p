use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct LastAction {
    pub time: f32,
}

impl LastAction {
    pub fn new() -> Self {
        // Monsters will act immediately upon spawning since we need to wait for
        // the GgrsSchedule Time to synchronize among clients
        Self { time: 0. }
    }
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Monster;

#[derive(Component)]
pub enum MonsterType {
    Ettin,
    Goblin,
    Ogre,
    Orc,
}
