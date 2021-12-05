use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use crate::prelude::*;

// receives WantsToMove messages and applies movement
// <(&Entity, &WantsToMove)>::query().iter_mut(ecs).for_each()
// the

#[system(for_each)] // derives query from system parameters, runs systems once for every matching entity
#[read_component(Player)]
pub fn movement(
    entity: &Entity,            // used to derive query
    want_move: &WantsToMove,   // used to derive query
    #[resource] map: &Map,          // resources not included in query
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commnads: &mut CommandBuffer
){
    // entity is the message entity
    // want_move is a component of the message entity
    // want_move.entity is the entity being reference by the message entity
    if map.can_enter_tile(want_move.destination){
        // add component to referenced entity. if its type exists (i.e. Point) then it is replaced
        commnads.add_component(want_move.entity, want_move.destination);

        /*
        In order entry_ref to be able to retrieve the Player entity, it must be available within the SubWorld.
        Hence we must use #[read_component(Player)] (or write_component) declaration in order for entities
        with that component to be available in the SubWorld passed into this system

        if entity is player then also update camera, hence require #[read_component(Player)] macro.
        use entity_ref to access component from an entity
         */
        if ecs.entry_ref(want_move.entity) // returns a Result indicating if entity is available in SubWorld
            .unwrap() // unwrap Option to get its contents
            .get_component::<Player>()// access entities component, returns a Result
            .is_ok() { // is Result is ok, execute statements

            // move player to destination and update camera information
            camera.on_player_move(want_move.destination);

        };
    };

    // remove message entity
    commnads.remove(*entity);
}