use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &SubWorld, commands: &mut CommandBuffer){
    // command buffer are instructions for legion to perform after system has completed
    // use it to remove entities from the game
    let mut player_pos = Point::zero();

    // find players position
    <&Point>::query()
        .filter(component::<Player>()) // this is a query filter, not an iterator filter function
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);

    // find monsters
    {

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        // filter for only entities that match players position
        // by the time pos reaches filter it has type of &&Point, so need **pos to dereference it
        // it entered query as a reference and iterator referenced it again
        .filter(|(_, pos)| **pos == player_pos)
        // iterator referenced entity, so it has type of &Entity by now
        // hence use *entity to get value of borrowed entity
        .for_each(|(entity,_)| {
            // remove entity at end of the frame
            commands.remove(*entity);
        });
    };
}
