use crate::prelude::*;

#[system(for_each)] // for each Entity with WantsToMove component
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        // access the details of another component on entity outside of the query for_each
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        // if the entity has a Player component
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity); // remove the WantsToMove component
}
