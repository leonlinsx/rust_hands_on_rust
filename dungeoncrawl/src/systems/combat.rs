use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    // taking the iterator of attackers and collecting into a vector
    // do not modify ecs while iterating over it
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        // check if the victim is a player (to prevent removing the player on death)
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        // only modify the victim's health if it has a Health component
        if let Ok(victim_health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", victim_health.current); // debug print
            victim_health.current -= 1;
            if victim_health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", victim_health.current);
        }
        commands.remove(*message); // remove the WantsToAttack message
    });
}
