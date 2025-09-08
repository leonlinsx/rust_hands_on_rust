use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Get the player entity and position first (short-lived borrow)
    let (player_entity, player_pos) = {
        let mut q = <(Entity, &Point)>::query().filter(component::<Player>());
        q.iter(ecs)
            .find_map(|(e, p)| Some((*e, *p)))
            .expect("Player entity not found")
    };

    // Convert key input into an Action enum
    let action = if let Some(key) = *key {
        match key {
            // Movement keys
            VirtualKeyCode::Left | VirtualKeyCode::A => Action::Move(Point::new(-1, 0)),
            VirtualKeyCode::Right | VirtualKeyCode::D => Action::Move(Point::new(1, 0)),
            VirtualKeyCode::Up | VirtualKeyCode::W => Action::Move(Point::new(0, -1)),
            VirtualKeyCode::Down | VirtualKeyCode::S => Action::Move(Point::new(0, 1)),
            // Pickup item
            VirtualKeyCode::G => Action::PickupAt(player_pos),
            // Use item from inventory (1-9 keys)
            VirtualKeyCode::Key1 => Action::Use(0),
            VirtualKeyCode::Key2 => Action::Use(1),
            VirtualKeyCode::Key3 => Action::Use(2),
            VirtualKeyCode::Key4 => Action::Use(3),
            VirtualKeyCode::Key5 => Action::Use(4),
            VirtualKeyCode::Key6 => Action::Use(5),
            VirtualKeyCode::Key7 => Action::Use(6),
            VirtualKeyCode::Key8 => Action::Use(7),
            VirtualKeyCode::Key9 => Action::Use(8),
            // No action for other keys
            _ => Action::None,
        }
    } else {
        Action::None
    };

    let mut did_something = false;
    let mut item_to_activate: Option<Entity> = None;

    // Handle the action (all ECS borrows are scoped)
    match action {
        Action::Move(delta) => {
            if delta != Point::zero() {
                let destination = player_pos + delta;

                let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
                let mut hit = false;

                enemies
                    .iter(ecs)
                    .filter(|(_, pos)| **pos == destination)
                    .for_each(|(victim, _)| {
                        hit = true;
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: player_entity,
                                victim: *victim,
                            },
                        ));
                    });

                if !hit {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: player_entity,
                            destination,
                        },
                    ));
                }

                did_something = true;
            }
        }

        Action::PickupAt(pos) => {
            let mut items = <(Entity, &Item, &Point)>::query();
            items
                .iter(ecs)
                .filter(|(_, _, item_pos)| *item_pos == &pos)
                .for_each(|(entity, _, _)| {
                    commands.remove_component::<Point>(*entity);
                    commands.add_component(*entity, Carried(player_entity));
                });

            did_something = true;
        }

        Action::Use(index) => {
            let mut q = <(Entity, &Item, &Carried)>::query();
            let result = q
                .iter(ecs)
                .filter(|(_, _, carried)| carried.0 == player_entity)
                .enumerate()
                .find_map(|(i, (entity, _, _))| (i == index).then_some(*entity));

            if let Some(item) = result {
                item_to_activate = Some(item);
                did_something = true;
            }
        }

        Action::None => {
            // If there's no input, we do NOT set `turn_state` – just return early
            return;
        }
    }

    // Push the item activation as a deferred command
    if let Some(item) = item_to_activate {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item,
            },
        ));
    }

    if did_something {
        *turn_state = TurnState::PlayerTurn;
    }
}

// Internal enum to represent player actions
enum Action {
    None,
    Move(Point),
    PickupAt(Point),
    Use(usize),
}
