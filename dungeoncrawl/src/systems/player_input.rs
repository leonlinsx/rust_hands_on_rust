use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Query player once, keep entity in outer scope
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let (player_entity, player_pos) = players
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos)))
        .unwrap();

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
            
            // pick up item
            VirtualKeyCode::G => {
                // return tuple of player entity and position
                let (player, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();
                let mut items = <(Entity, &Item, &Point)>::query();
                // return entities with item component at player position
                items
                    .iter(ecs)
                    .filter(|t| t.2 == &player_pos) // t: &(&Entity, &Item, &Point); t.2: &Point
                    .for_each(|t| {
                        let entity = t.0; // t.0: &Entity
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));
                    });

                Point::new(0, 0) // no movement
            }

            _ => Point::new(0, 0),
        };

        let mut did_something = false; // track if an action was taken
        let destination = player_pos + delta;

        if delta.x != 0 || delta.y != 0 {
            // try to attack enemies at destination; otherwise move
            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            let mut hit_something = false;

            // check if any enemies are at the destination
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        // Recover health if no action taken (delta was zero or nothing else happened)
        if !did_something {
            if let Ok(health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1); // capped at max health
                println!("You rest and recover 1 hp.");
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
