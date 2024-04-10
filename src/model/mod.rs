mod logic;

use evenio::prelude::*;
use geng::prelude::*;

pub type Time = R32;

#[derive(Event)]
pub struct Update {
    pub delta_time: Time,
}

#[derive(Event)]
pub struct SpawnZombie;

#[derive(Event)]
pub struct ZombieSpawned {
    #[event(target)]
    pub entity: EntityId,
}

#[derive(Component)]
pub struct Position(pub vec2<f32>);

// TODO: add unit tests
pub fn init(world: &mut World) {
    logic::init(world);
}
