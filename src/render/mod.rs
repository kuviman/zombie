use crate::{assets::Assets, model::*};

use evenio::{prelude::*, query};
use geng::prelude::*;

#[derive(Event)]
pub struct Draw {
    pub framebuffer: &'static mut ugli::Framebuffer<'static>,
}

#[derive(Component)]
pub struct Sprite {
    pub texture: Rc<ugli::Texture>,
    pub transform: mat3<f32>,
}

fn clear(mut receiver: ReceiverMut<Draw>) {
    let framebuffer = &mut *receiver.event.framebuffer;
    ugli::clear(framebuffer, Some(Rgba::BLUE), None, None);
}

#[derive(Component)]
pub struct Global {
    geng: Geng,
    assets: Rc<Assets>,
}

#[derive(Component)]
pub struct Camera {
    camera: Camera2d,
}

// TODO: add unit tests
fn draw_sprites(
    mut receiver: ReceiverMut<Draw>,
    sprites: Fetcher<&Sprite>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    for sprite in sprites {
        global.geng.draw2d().draw2d(
            framebuffer,
            &camera.camera,
            &draw2d::TexturedQuad::unit(&*sprite.texture).transform(sprite.transform),
        );
    }
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
    world.insert(
        global,
        Camera {
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: 20.0,
            },
        },
    );
    world.spawn();
    world.add_handler(update_transforms);
    world.add_handler(clear);
    world.add_handler(draw_sprites);
    world.add_handler(spawn_zombie);
}

fn update_transforms(_receiver: Receiver<Draw>, query: Fetcher<(&Position, &mut Sprite)>) {
    for (position, sprite) in query {
        sprite.transform = mat3::translate(position.0);
    }
}

// TODO: add zombie sounds
fn spawn_zombie(
    receiver: Receiver<ZombieSpawned, &Position>,
    mut sender: Sender<Insert<Sprite>>,
    global: Single<&Global>,
) {
    let position = receiver.query.0;
    sender.insert(
        receiver.event.entity,
        Sprite {
            texture: global.0.assets.zombie.clone(),
            transform: mat3::translate(position),
        },
    )
}
