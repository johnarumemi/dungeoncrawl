use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use crate::prelude::*;

/// Receives WantsToMove messages/entities and applies movement
///
/// ## Notes
/// '#[system(for_each)]' derives query from system/function parameters,
/// runs systems once for every matching entity
///
/// Below function signature is equivalent to using
/// <(&Entity, &WantsToMove)>::query().iter_mut(ecs).for_each()
///
/// ## Arguments
/// entity: component used for deriving query. Entity that has WantsToMove component.
/// want_move: component used for deriving query.
#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
){
    // entity is the message entity
    // want_move is a component of the message entity
    // want_move.entity is the entity being reference by the message entity
    if map.can_enter_tile(want_move.destination){
        // add component to referenced entity.
        // If its type exists (i.e. Point) then it is replaced
        commands.add_component(want_move.entity, want_move.destination);

        /*
        In order for entry_ref to be able to retrieve the Player entity,
        it must be available within the SubWorld.
        Hence we must use #[read_component(Player)] (or write_component)
        declaration in order for entities with that component to be
        available in the SubWorld passed into this system

        If entity is player then also update camera,
        hence require #[read_component(Player)] macro.
        use get_component::<Component>() to access component within an entity
         */
        // returns a Result indicating if entity is available in SubWorld
        if ecs.entry_ref(want_move.entity)
            // unwrap Option to get its contents
            .unwrap()
            // access entities component, returns a Result
            .get_component::<Player>()
            .is_ok() { // if Result is ok, execute statements

            // move player to destination and update camera information
            camera.on_player_move(want_move.destination);

        };
    };

    // remove message entity after processing is complete
    commands.remove(*entity);
}