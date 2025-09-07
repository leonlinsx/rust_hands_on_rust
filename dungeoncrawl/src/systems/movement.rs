use crate::prelude::*;

#[system(for_each)] // for each Entity with WantsToMove component
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // if the destination tile is walkable, add a Point component to the entity with the new position
        commands.add_component(want_move.entity, want_move.destination);

        // refresh the FOV of the entity that moved
        // access the details of another component on entity outside of the query for_each
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                // if the entity has a Player component
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);

                    // cumulatively add to revealed tiles
                    fov.visible_tiles.iter().for_each(|p| {
                        map.revealed_tiles[map_idx(p.x, p.y)] = true;
                    });
                }
            }
        }
    }
    // remove the WantsToMove component
    commands.remove(*entity);
}
