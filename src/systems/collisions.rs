use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use crate::prelude::*;

///
///
/// # Notes
/// CommandBuffer is a special container added by Legion, to allow you
/// to insert instructions for Legion to execute after the system has been run
///
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
        // note that closure has content 'moved' into it.
        .for_each(|pos| player_pos = *pos);

    // find monsters
    {

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        // filter for only entities that match players position
        // filter iterates over references to its contents.
        // Hence, by the time pos reaches filter it has type of &&Point,
        // so need **pos to dereference it
        // Overall, it entered query as a reference and 'filter' referenced it again
        .filter(|(_, pos)| **pos == player_pos)
        // iterator referenced entity, so it has type of &Entity by now
        // hence use *entity to get value of borrowed entity
        .for_each(|(entity,_)| {
            // remove entity at end of the frame
            commands.remove(*entity);
        });
    };
}
