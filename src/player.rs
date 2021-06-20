
const COLL_L: i32 = 248;
const COLL_R: i32 = 1208;
const COLL_U: i32 = 72;
const COLL_D: i32 = 648;

pub struct Player {
    pos_x: i32,
    pos_y: i32,
    hbox_x: u32,
    hbox_y: u32,
    speed: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x: 512,
            pos_y: 256,
            hbox_x: 64,
            hbox_y: 32,
            speed: 3,
        }
    }

    pub fn update_pos(& mut self, mov_x: i32, mov_y: i32) {
        self.pos_x += mov_x * self.speed;
        self.pos_y += mov_y * self.speed;


        self.pos_x = self.pos_x.clamp(COLL_L + (self.hbox_x/2) as i32, COLL_R - (self.hbox_x/2) as i32);
        self.pos_y = self.pos_y.clamp(COLL_U + (self.hbox_y/2) as i32, COLL_D - (self.hbox_y/2) as i32);
    }

    pub fn get_pos_x(&self) -> i32 { self.pos_x }
    pub fn get_pos_y(&self) -> i32 { self.pos_y }

    pub fn get_hbox_x(&self) -> u32 { self.hbox_x }
    pub fn get_hbox_y(&self) -> u32 { self.hbox_y }
}