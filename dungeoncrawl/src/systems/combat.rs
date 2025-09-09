use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    // taking the iterator of attackers and collecting into a vector
    // do not modify ecs while iterating over it
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        // check if the victim is a player (to prevent removing the player on death)
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(damage) = v.get_component::<Damage>() {
                damage.0
            } else {
                0 // default damage if no Damage component
            }
        } else {
            0 // default damage if attacker entity not found
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, damage)| damage.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current); // debug print
            health.current -= final_damage;
            println!("Health after attack: {}", health.current);
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }

        commands.remove(*message); // remove the WantsToAttack message

        // // check if the victim is a player (to prevent removing the player on death)
        // let is_player = ecs
        //     .entry_ref(*victim)
        //     .unwrap()
        //     .get_component::<Player>()
        //     .is_ok();

        // // only modify the victim's health if it has a Health component
        // if let Ok(victim_health) = ecs
        //     .entry_mut(*victim)
        //     .unwrap()
        //     .get_component_mut::<Health>()
        // {
        //     println!("Health before attack: {}", victim_health.current); // debug print
        //     victim_health.current -= 1;
        //     if victim_health.current < 1 && !is_player {
        //         commands.remove(*victim);
        //     }
        //     println!("Health after attack: {}", victim_health.current);
        // }
        // commands.remove(*message); // remove the WantsToAttack message
    });
}
