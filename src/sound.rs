use crate::{assets::Assets, model::*};
use evenio::prelude::*;
use geng::prelude::*;

#[derive(Component)]
pub struct Global {
    geng: Geng,
    assets: Rc<Assets>,
}

pub fn init(world: &mut World, geng: &Geng, assets: &Rc<Assets>) {
    let global = world.spawn();
    world.insert(
        global,
        Global {
            geng: geng.clone(),
            assets: assets.clone(),
        },
    );
    world.add_handler(spawn_zombie);
}

fn spawn_zombie(receiver: Receiver<ZombieSpawned, ()>, global: Single<&Global>) {
    global.assets.aaah.play();
}
