use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {

    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    pub fn render(self: &Self, ctx: &mut BTerm, camera: &Camera){
        ctx.set_active_console(1); // render on the layer with no background and transparent
        let relative_x = self.position.x - camera.left_x;
        let relative_y  = self.position.y - camera.top_y;
        ctx.set(relative_x, relative_y, WHITE, BLACK, to_cp437('@'));
    }

    pub fn update(self: &mut Self, ctx: &mut BTerm, map: &Map, camera: &mut Camera){

        if let Some(key) = ctx.key {

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

            let new_position = self.position + delta;

            if map.can_enter_tile(new_position){
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}