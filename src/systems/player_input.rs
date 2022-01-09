use legion::systems::CommandBuffer;
use legion::world::SubWorld;

// nested module inside the 'systems' module
use crate::prelude::*;

// use procedural macro to inject boilerplate ... similar to a decorator
#[system]  // legion macro
#[read_component(Point)] // request write access to a component type i.e. Point
#[read_component(Player)] // request read access to Player component
pub fn player_input(
    ecs: &mut SubWorld, // a SubWorld can only see components that were requested
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
){
    if let Some(key) = key {
        // get requested movement
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),

            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0) // all other keys are ignored
        }; // use semi-colon as this is a statement due to 'let'

        // query SubWorld for player
        if delta.x != 0 || delta.y != 0 {
            // use <&mut Point> to ensure query returns a mutable reference to the components for entities
            let mut players = <(Entity, &Point)>::query()
                .filter(component::<Player>()); // the query output won't actually contain the Player component's data though

            // The query does not become an iterator until you call iter or iter_mut on it
            // hence using .filter() does not change the query to an iterator but rather returns a query
            // use iter_mut to obtain a mutable iterator
            players.iter(ecs).for_each(|(entity, pos)| {
                // use *pos to get actual value being referenced (dereference)
                let destination = *pos + delta;
                /*
                 NOTE: there is no check anymore to see if destination is a valid tile move
                 before ending the players turn
                 */

                /*
                 emit WantsToMove message
                 push does not support singular components, hence the use of the () as a component
                 */
                commands.push( ( (), WantsToMove{ entity: *entity, destination}) );

                // update turn state
                *turn_state = TurnState::PlayerTurn;
            })
        }
    }
}
