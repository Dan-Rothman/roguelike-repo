use crate::player::*;
use crate::attack::*;
use crate::util::*;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use crate::boxes::*;
use crate::yellowenemy::*;
use crate::blackboard::*;

use rand::Rng;

pub trait Health {
    // TODO: Add iframe setup so player can't be damaged for a bit after being damaged.
    // Otherwise a spike tile doing half a heart of damage would kill a 3 heart player in 6 frames.

    fn max_hp(&self) -> i32;  //the maximum HP the entity can have
    fn health(&self) -> i32;    // the current HP the entity has
    fn damage(&mut self, d: i32) -> i32;  // applying the amount of damage received
    //fn damage_type(&self) -> i32; // the damage category
    //fn bonus_type(&self) -> i32;    // the type of bonus dropped by enemy
    //fn percent_damaged(&self) -> f32;
    fn heal(&mut self, h: i32) -> i32;
    fn death(&mut self) -> bool;
}

#[derive(Debug)]
#[derive (Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum EnemyKind {
    Attack,
    Health,
    Speed
}

#[derive(Clone)]
pub enum State{
    Attack,
    Retreat,
    TakeCover,
    Chase,
    Heal,
    Idle,
}

#[derive(Clone)]
pub struct Enemy {
    pub pos: Vec2<f32>,
    pub lastpos:Vec2<f32>,
    pub box_es: Box,
    pub speed: f32,
    pub dir: Direction,
    pub hp: i32,    //store the health for speed enemy
    pub m_hp: i32,
    pub movement_vec: Vec2<f32>,
    pub last_dir_update: Option< Instant >,
    pub kind: EnemyKind,
    pub death: bool,
    pub power: bool,
    pub atk_list: Vec<AtkProjectile>,
    pub last_invincibility_time: Option<Instant>,
    pub state: State,
    pub is_attacking: bool,
    pub last_attack_time: Option<Instant>,
    pub current_frame_tile: Vec2<i32>,
    pub is_healing: bool,
}

impl Health for Enemy {
    fn max_hp(&self) -> i32 { self.m_hp }
    fn health(&self) -> i32 { self.hp }
    fn damage(&mut self, d: i32) -> i32 {
        self.hp = (self.hp - d).max(P_DEATH_HP);
        self.death();
        self.hp
    }

    fn heal(&mut self, h: i32) -> i32 {
        self.hp = (self.hp + h).min(self.m_hp);
        self.hp
    }

    fn death(&mut self) -> bool {
        if self.hp <= P_DEATH_HP {
            self.death = true;
        }
        self.death
    }
}

impl Enemy {
    pub fn new(position: Vec2<f32>, kind: EnemyKind) -> Enemy {
        Enemy {
            pos: position,
            lastpos: Vec2::new(-1.0, 0.0),
            box_es: Box::new(Vec2::new(40, 30), Vec2::new(40, 40), Vec2::new(40, 30)),
            speed: speed_kind(kind),
            dir: Direction::Right,
            hp: health_kind(kind),
            m_hp: health_kind(kind),
            movement_vec: Vec2::new(-1.0, 0.0),
            last_dir_update: None,
            kind: kind,
            death: false,
            power: false,
            atk_list: Vec::new(),
            state: State::Idle,

            current_frame_tile: Vec2::new(0,0),
            last_invincibility_time: None,

            //timing attacks so they aren't just 'on'
            is_attacking: false,
            last_attack_time: None,

            is_healing: false
        }
    }

    pub fn update(& mut self, blackboard: &BlackBoard) {
        self.current_frame_tile = Vec2::new(
            (self.get_pos_x() - LEFT_WALL) / TILE_WIDTH,
            (self.get_pos_y() - TOP_WALL) / TILE_WIDTH
        );

        self.update_dir(blackboard.player_frame_tile);
        //println!("{:?}", self.current_frame_tile);
        match self.kind {
            EnemyKind::Health => {
            }
            EnemyKind::Speed => {
            }
            EnemyKind::Attack => {
                crate::yellowenemy::update(self, blackboard);
            }
        }
    }

    pub fn update_invincibility_time(&mut self) {
        self.last_invincibility_time = Some(Instant::now());
    }

    pub fn take_damage(&mut self, amount: i32, cooldown_window_ms: u64) {
        match self.last_invincibility_time {
            // If there is an old invincibility time for the player,
            // see if the "invincibility window" has elapsed since then...
            Some( time ) => {
                if time.elapsed() >= Duration::from_millis(cooldown_window_ms) {
                    // If so, update the invincibility time and take damage to the player.
                    self.update_invincibility_time();
                    self.damage(amount);
                }
            },
            None => {
                // Otherwise, take damage as there was
                // no previous "invincibility window" to account for
                self.update_invincibility_time();
                self.damage(amount);
            }
        }
    }

    pub fn update_dir(& mut self, frame_tile: Vec2<i32>){
        let e_x = self.current_frame_tile.x;
        let e_y = self.current_frame_tile.y;
        let p_x = frame_tile.x;
        let p_y = frame_tile.y;
        if(e_x == p_x && e_y < p_y)
        {
            self.dir = Direction::Down;
        }

        if(e_x == p_x && e_y > p_y)
        {
            self.dir = Direction::Up;
        }

        if e_x > p_x
        {
            self.dir = Direction::Left;
        }

        if e_x < p_x
        {
            self.dir = Direction::Right;
        }
    }

    pub fn player_close(enemy: & mut Enemy, blackboard: &BlackBoard) -> bool{
        /*
        let e_x = enemy.pos.x;
        let e_y = enemy.pos.y;
        let p_x = blackboard.playerpos.x;
        let p_y = blackboard.playerpos.y;

        if (e_x == p_x && ((e_y-20.0) <= p_y || (e_y+20.0) >= p_y)) || //If the player is right above or below the enemy
        (e_y == p_y && ((e_x-20.0) <= p_x || (e_x+20.0) >= p_x)) || //If the player is on either side of the enemy
        (e_y == p_y && e_x == p_x){ //If the player is on top of the enemy*/

        if (enemy.box_es.get_walkbox(enemy.pos).has_intersection(blackboard.player_box.get_walkbox(blackboard.playerpos))){
            return true;
        }
        else{
            return false;
        }
    }
    // Using Connor's player implementation for this design:

    pub fn type_eq(a: EnemyKind, b: EnemyKind) -> bool{
        let num1 = Enemy::assign_num(a);
        let num2 = Enemy::assign_num(b);
        //println!("{:?}, {}", a, num1);
        //println!("{:?}, {}", b, num2);
        if(num1 == num2){
            return true;
            }
        else{
            return false;
            }
    }

    pub fn assign_num(a: EnemyKind) -> i32
    {
        match a {
            EnemyKind::Health => {
                return 0;
            }
            EnemyKind::Speed => {
                return 1;
            }
            EnemyKind::Attack => {
                return 2;
            }
        }
    }

    pub fn signal_attack(&mut self) {
        match self.last_attack_time {
            Some (time) => {
                let res = time.elapsed() <= Duration::from_millis(500+600);
                if !res {
                    self.is_attacking = true;
                    self.last_attack_time = Some(Instant::now());
                }
                else {
                    self.is_attacking = false;
                }
            }

            None => {
                self.is_attacking = true;
                self.last_attack_time = Some(Instant::now());
            }
        }
    }

    pub fn recently_attacked(&mut self) -> bool {
        match self.last_attack_time {
            Some( time ) => {
                let res = time.elapsed() <= Duration::from_millis(500);
                if !res {
                    self.is_attacking = false;
                }

                res
            },
            None => false
        }
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32 }
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32 }

    pub fn update_pos(& mut self) {
        if self.death {
            self.movement_vec.x = 0.0;
            self.movement_vec.y = 0.0;
            return;
        }


        let now = Instant::now();

        let mut rng = rand::thread_rng();

        match self.last_dir_update {
            Some(update_time) => {
                if update_time.elapsed() >= Duration::from_secs(2) {

                    //Make a new attack projectile every time the enemy moves. For test things
                    let new_atk = AtkProjectile::new(self.pos, self.movement_vec, &self.kind);
                    self.atk_list.push(new_atk);

                    match rng.gen_range( 0 ..= 15 ) {
                        0 => {
                            self.movement_vec.x = 0.0;
                            self.movement_vec.y = -1.0;
                        },
                        1 | 2 => {
                            self.movement_vec.x = 0.0;
                            self.movement_vec.y = 1.0;
                        },
                        3 | 4 => {
                            self.movement_vec.x = -1.0;
                            self.movement_vec.y = 0.0;
                        },
                        5 | 6 => {
                            self.movement_vec.x = 1.0;
                            self.movement_vec.y = 0.0;
                        },
                        7 | 8 => {
                            self.movement_vec.x = DIAGONAL_VEC;
                            self.movement_vec.y = DIAGONAL_VEC;
                        },
                        9 | 10 => {
                            self.movement_vec.x = -DIAGONAL_VEC;
                            self.movement_vec.y = -DIAGONAL_VEC;
                        },
                        11 | 12 => {
                            self.movement_vec.x = DIAGONAL_VEC;
                            self.movement_vec.y = -DIAGONAL_VEC;
                        },
                        13 | 14 => {
                            self.movement_vec.x = -DIAGONAL_VEC;
                            self.movement_vec.y = DIAGONAL_VEC;
                        },
                        15 => {
                            self.movement_vec.x = 0.0;
                            self.movement_vec.y = 0.0;
                        }
                        _ => {}
                    }
                    //self.movement_vec.x = -self.movement_vec.x;
                    //self.movement_vec.y = rng.gen_range(-1.0 ..= 1.0);
                    self.last_dir_update = Some(now);
                }
            },
            None => {
                self.last_dir_update = Some(now);
            }
        }

        // Update position using movement vector and speed
        self.pos.x += self.movement_vec.x * self.speed;
        self.pos.y += self.movement_vec.y * self.speed;

        //Moves all the attacks that this enemy shot

        let mut index = 0;
        let mut to_remove = Vec::new();
        for mut atk in &mut self.atk_list {
            atk.pos.x += atk.movement_vec.x * atk.speed;
            atk.pos.y += atk.movement_vec.y * atk.speed;

            //If the attack is off screen, remove it from the atk vector

            if atk.pos.x < 0.0 || atk.pos.y < 0.0 || atk.pos.x > WINDOW_WIDTH as f32|| atk.pos.y > WINDOW_HEIGHT as f32
            {
                to_remove.push(index);
            }
            index+=1;
        }

        for rmv in &mut to_remove {
            self.atk_list.remove(*rmv);
            println!("Bullet Removed");
        }
    }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }
}

pub fn speed_kind(kind: EnemyKind) -> f32 {
    let mut speed = 0.0;
    match kind {
        EnemyKind::Health => {
            speed = 1.8;
        }
        EnemyKind::Speed =>{
            speed = 3.8;
        }
        EnemyKind::Attack => {
            speed = 2.8;
        }
    }
    return speed;
}pub fn health_kind(kind: EnemyKind) -> i32 {
    let mut health = 0;
    match kind {
        EnemyKind::Health => {
            health = 5;
        }
        EnemyKind::Speed =>{
            health = 2;
        }
        EnemyKind::Attack => {
            health = 3;
        }
    }
    return health;
}
