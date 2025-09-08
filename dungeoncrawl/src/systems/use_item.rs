use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    // as system iterates through item effects, add healing events to this vec
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    {
        // Limit the lifetime of this query borrow
        <(Entity, &ActivateItem)>::query()
            .iter(ecs)
            .for_each(|(item_entity, activate)| {
                // entry_ref returns reference to entity not returned from query, which we can use to get components
                let item = ecs.entry_ref(activate.item);
                if let Ok(item) = item {
                    if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                        // queue healing to apply after iteration
                        healing_to_apply.push((activate.used_by, healing.amount));
                    }
                    if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                        // reveal whole map
                        map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                    }
                }
                commands.remove(activate.item); // remove item after use
                commands.remove(*item_entity);
            });
    }

    // apply healing after iteration to avoid mutable/immutable borrow issues
    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1); // capped at max health
                println!("You use a healing item, restoring {} hp.", heal.1);
            }
        }
    }
}
