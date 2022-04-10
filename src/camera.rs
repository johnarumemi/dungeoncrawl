use crate::prelude::*;

/// Camera whose bounds  define the visible secion of the map
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    /// Create new camera centered on player position
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

    /// function to be called on player move with player's new position
    pub fn on_player_move(self: &mut Self, player_position: Point) {
        self.set_position(player_position);
    }

    /// Update camera boundary
    fn set_position(&mut self, point: Point) {
        self.left_x = point.x - DISPLAY_WIDTH / 2;
        self.right_x = point.x + DISPLAY_WIDTH / 2;
        self.top_y = point.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = point.y + DISPLAY_HEIGHT / 2;
    }
}
