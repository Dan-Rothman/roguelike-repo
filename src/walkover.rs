use crate::game::*;
use crate::menu::*;
use crate::util::*;
use crate::entity::*;
use crate::tile::*;
use crate::player::PowerUp;
use std::time::Instant;


pub fn base(mut game : &mut Game, mut menu : &mut MenuState){

    // Branch for tiles that should only be called once (doors, pickups
    if game.player.current_frame_tile != game.player.prev_frame_tile {
        //TODO: Find a way to make these chain calls less crazy

        // Set new room to visited
        // This is done to the now previous room to avoid having to do special logic on the first room
        game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].visited = true;

        match game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles[game.player.current_frame_tile.y as usize][game.player.current_frame_tile.x as usize].on_walkover() {
            WalkoverAction::DoNothing => {game.player.speed_adjust(WalkoverAction::DoNothing);},
            WalkoverAction::ChangeRooms => {
                //println!("Door tile walked over.");
                if game.player.current_frame_tile.x == 0 { // LEFT DOOR
                    // Current room one to the right
                    game.cr.x -= 1;
                    // Move player position to just outside of right door in new room
                    game.player.pos = Vec2::new((LEFT_WALL + 15 * 64) as f32 + 63.0, (TOP_WALL + 5 * 64) as f32 + 40.0);
                    game.trans_dir = Direction::Left;
                }
                if game.player.current_frame_tile.x == 16 { // RIGHT DOOR
                    // Current room one to the right
                    game.cr.x += 1;
                    // Move player position to just outside of left door in new room
                    game.player.pos = Vec2::new((LEFT_WALL + 1 * 64) as f32 + 1.0, (TOP_WALL + 5 * 64) as f32 + 40.0);
                    game.trans_dir = Direction::Right;
                }
                if game.player.current_frame_tile.y == 0 { // TOP DOOR
                    // Current room one up
                    game.cr.y -= 1;
                    // Move player position to just outside of bottom door in new room
                    game.player.pos = Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 9 * 64) as f32 + 50.0);
                    game.trans_dir = Direction::Up;
                }
                if game.player.current_frame_tile.y == 10 { // BOTTOM DOOR
                    // Current room one down
                    game.cr.y += 1;
                    // Move player position to just outside of bottom door in new room
                    game.player.pos = Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 1 * 64) as f32 + 10.0);
                    game.trans_dir = Direction::Down;
                }

                game.game_state = GameState::BetweenRooms;
                game.transition_start = Instant::now();
                // sleep(Duration::new(2, 0));

            },

            // Gem pickups
            WalkoverAction::BuffDamage => { game.player.plus_power_attack(); }
            WalkoverAction::BuffHealth => { game.player.plus_power_health(); }
            WalkoverAction::BuffSpeed => { game.player.plus_power_speed(); }

            WalkoverAction::GivePlayerKey => {
                println!("Key has been picked up!!!");
                game.player.has_key = true;
            },

            WalkoverAction::GivePlayerBomb => {
                println!("Bomb was picked up!!!");
                game.player.has_bomb = true;
            }

            WalkoverAction::Damage => {
                println!("You've stepped on spikes!");
                game.player.take_damage(1, 1000);
                game.player.speed_adjust(WalkoverAction::Damage);
                if game.player.death() {
                    *menu = MenuState::GameOver;
                }
            },

            WalkoverAction::GoToNextFloor => {
                if game.player.has_key {
                    println!("Congratulations! You made it to the next floor!!!");
                    game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize]
                        .tiles[game.player.current_frame_tile.y as usize][game.player.current_frame_tile.x as usize].unlock();
                    game.player.has_key = false;
                    //Debug: println!("{}", game.cf);

                    // FLOOR CHANGING DONE ONCE TRANSITION IS COMPLETE
                    //Debug: println!("{}", game.cf);
                    // THIS WILL NEED CHANGING

                    game.changed_floors = false;
                    game.transition_start = Instant::now();
                    game.game_state = GameState::BetweenFloors;

                }
                else {
                    println!("You need a key to unlock this door!");
                }

            }

        }
    }
    // TODO: else branch for continuous tiles (spike tile)
}