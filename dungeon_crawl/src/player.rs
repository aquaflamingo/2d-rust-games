use crate::prelude::*;

pub struct Player {
    pub position : Point
}

impl Player {
    pub fn new(p : Point) -> Self {
        Self {
            position: p,
        }
    }

    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        // Use the second layer to render
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - cam.left_x, 
            self.position.y - cam.top_y, 
            WHITE, 
            BLACK, 
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, c: &mut BTerm, map : &Map, camera: &mut Camera) {
        if let Some(key) = c.key {
            let delta = match key {
                VirtualKeyCode::Right => Point::new(1,0),
                VirtualKeyCode::Left => Point::new(-1,0),
                VirtualKeyCode::Up => Point::new(0,-1),
                VirtualKeyCode::Down => Point::new(0,1),
                _ => Point::zero()
            };

            let new_pos = self.position + delta;
            if map.can_enter_tile(new_pos) {
                self.position = new_pos;
                camera.on_player_move(new_pos)
            }
        }
    }
}
