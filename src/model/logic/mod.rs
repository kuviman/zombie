use super::*;

pub fn init(world: &mut World) {
    world.add_handler(spawn_zombie);
    world.add_handler(zombie_movement);
    world.add_handler(zombie_collision);
}

fn spawn_zombie(
    _receiver: Receiver<SpawnZombie>,
    mut sender: Sender<(Spawn, Insert<Position>, ZombieSpawned)>,
) {
    let entity = sender.spawn();
    let pos = thread_rng().gen_circle(vec2::ZERO, 10.0);
    sender.insert(entity, Position(pos));
    sender.send(ZombieSpawned { entity });
}

fn zombie_movement(receiver: Receiver<Update>, query: Fetcher<&mut Position>) {
    let delta_time = receiver.event.delta_time;
    for position in query {
        position.0 += (-position.0).clamp_len(..=delta_time.as_f32());
    }
}

fn iter_pairs<'a, T: 'a>(
    iter: impl IntoIterator<Item = &'a mut T>,
    mut f: impl FnMut(&mut T, &mut T),
) {
    let mut positions: Vec<_> = iter.into_iter().collect();
    for i in 1..positions.len() {
        let (head, tail) = positions.split_at_mut(i);
        let a = &mut **head.last_mut().unwrap();
        for b in tail {
            let b = &mut **b;
            f(a, b);
        }
    }
}

fn zombie_collision(_receiver: Receiver<Update>, query: Fetcher<&mut Position>) {
    iter_pairs(query, |a, b| {
        let radius = 1.0;
        let penetration = 2.0 * radius - (a.0 - b.0).len();
        if penetration > 0.0 {
            let v = (a.0 - b.0).normalize_or_zero();
            a.0 += v * penetration / 2.0;
            b.0 -= v * penetration / 2.0;
        }
    });
}
