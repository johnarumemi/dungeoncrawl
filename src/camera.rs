use crate::prelude::*;


// create camera and update it when player moves
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {

    pub fn new(player_position: Point) -> Self {
        // center camera on player position
        let mut camera = Camera {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0,
        };

        camera.set_position(player_position);

        camera
    }

    pub fn on_player_move(self: &mut Self, player_position: Point) {
        self.set_position(player_position);
    }

    fn set_position(&mut self, point: Point){
        self.left_x = point.x - DISPLAY_WIDTH / 2;
        self.right_x = point.x + DISPLAY_WIDTH / 2;
        self.top_y = point.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = point.y + DISPLAY_HEIGHT / 2;
    }
}

