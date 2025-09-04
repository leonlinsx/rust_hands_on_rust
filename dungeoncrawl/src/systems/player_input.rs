use crate::prelude::*;

#[system]
#[write_component(Point)] // requests write access to Point component
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            // query for all entities with Point and Player components, returning Point as mutable
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let new_pos = *pos + delta;
                if map.can_enter_tile(new_pos) {
                    *pos = new_pos;
                    camera.on_player_move(new_pos);
                }
            });
        }
    }
}
