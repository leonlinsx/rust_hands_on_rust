use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Grail)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    // Access the ECS world to query entities, filtering to player health
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut grail = <&Point>::query().filter(component::<Grail>());
    let current_state = turn_state.clone();
    // cannot do oneliner because of borrow checker
    let grail_default = Point::new(-1, -1);
    let grail_position = grail.iter(ecs).nth(0).unwrap_or(&grail_default);
    // let grail_position = grail.iter(ecs).nth(0).unwrap();

    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => current_state,
    };
    player_hp.iter(ecs).for_each(|(hp, pos)| {
        // Check if the player is dead
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        // Check if the player has reached the grail
        if pos == grail_position {
            new_state = TurnState::Victory;
        }

        // Check if player is on exit tiles
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state; // dereference to assign the new state
}
