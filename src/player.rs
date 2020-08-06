use ggez::{graphics};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub jump_v: Option<f32>,
    pub lr_v: f32
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            x,
            y,
            jump_v: None,
            lr_v: 0.0
        }
    }

    pub fn move_player(&mut self, platfroms: &Vec<(i16, i16, i16)>) {
        if let Some(v) = self.jump_v{
            let dir = if v < 0.0 {
                -1.0
            } else {
                1.0
            };
            self.y -= (v * v) as f32 * 0.007 * dir;
            self.jump_v = Some(v - 1.0);
        }
        if self.y > 24.0 {
            self.jump_v = None;
            self.y = 24.0
        }
        for pos in platfroms {
            if graphics::Rect::new(self.x + 0.1, self.y + 0.4, 0.8, 0.8).overlaps(
                &graphics::Rect::new(pos.0 as f32, pos.1 as f32, pos.2  as f32, 0.1 * self.jump_v.unwrap_or(0.0).abs())
            ) && self.jump_v.unwrap_or(1.0) < 0.0
            {
                self.jump_v = None;
                self.y = pos.1 as f32 - 1.0
            }
        }
        if self.lr_v != 0.0 {
            self.x += self.lr_v * if self.jump_v == None {0.02} else {0.03};
            self.lr_v = self.lr_v / if self.jump_v == None {1.3} else {1.05};
            if self.lr_v * self.lr_v < if self.jump_v == None {1.0} else {0.0} {
                self.lr_v = 0.0
            }
        }
        let mut on_platfrom = false;
        for pos in platfroms {
            if  self.jump_v == None {
                // if self.x >= pos.0 as f32 - 1.0 && self.x <= pos.2 as f32 + pos.0 as f32 && self.y == pos.1 as f32 - 1.0
                if graphics::Rect::new(self.x, self.y + 0.2, 1.0, 0.8).overlaps(
                    &graphics::Rect::new(pos.0 as f32, pos.1  as f32, pos.2  as f32, 0.1)
                ) {
                    on_platfrom = true
                }
                else if self.y >= 24.0{
                    on_platfrom = true
                }
            } else {
                on_platfrom = true
            }
        }
        if !on_platfrom {
            self.jump_v = Some(-1.0);
        }
        if self.x < -1.0 {
            self.x = 34.5
        }
        if self.x > 35.0 {
            self.x = -0.5
        }
    }
}