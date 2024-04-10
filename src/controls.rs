use crate::model::*;
use evenio::prelude::*;
use geng::prelude::*;

#[derive(Event)]
pub struct GengEvent(pub geng::Event);

pub fn init(world: &mut World, _geng: &Geng) {
    world.add_handler(spawn_zombie);
}

fn spawn_zombie(receiver: Receiver<GengEvent>, mut sender: Sender<SpawnZombie>) {
    if let geng::Event::KeyPress {
        key: geng::Key::Space,
    } = receiver.event.0
    {
        sender.send(SpawnZombie);
    }
}
