use crate::game::*;
use crate::util::*;
use crate::player::PowerUp;
use crate::entity::*;
use crate::tile::*;
use crate::menu::*;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use roguelike::SDLCore;
use crate::boxes::*;

pub fn enemy_collision(enemy: &mut Enemy, x: &i32, y: &i32) {
    let intersection = enemy.box_es.get_walkbox(enemy.pos).intersection(Rect::new(
        LEFT_WALL + x * TILE_WIDTH, 
        TOP_WALL + y * TILE_WIDTH, 
        TILE_WIDTH as u32, 
        TILE_WIDTH as u32
    ));

    let inter_rect = match intersection {
        Some(x) => x,
        None => return,
    };

    let mut x_offset = inter_rect.width() as i32;
    let mut y_offset = inter_rect.height() as i32;

    if enemy.pos.x < inter_rect.x() as f32 {
        // TO THE LEFT OF ROCK
        y_offset = 0;
    }
    if enemy.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32 {
        // TO THE RIGHT OF ROCK
        x_offset *= -1;
        y_offset = 0;
    }
    if enemy.pos.y < inter_rect.y() as f32 {
        // ABOVE ROCK
        x_offset = 0;
    }
    if enemy.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32 {
        // BELOW ROCK
        x_offset = 0;
        y_offset *= -1;
    }

    enemy.pos.x -= x_offset as f32;
    enemy.pos.y -= y_offset as f32;
}

pub fn base(mut game : &mut Game, mut core : &mut SDLCore, mut menu : &mut MenuState){
// Outermost wall collision
        game.player.pos.x = game.player.pos.x.clamp(
            LEFT_WALL as f32 + (game.player.box_es.walkbox.x/2) as f32,
            RIGHT_WALL as f32 - (game.player.box_es.walkbox.x/2) as f32
        );
        game.player.pos.y = game.player.pos.y.clamp(
            TOP_WALL as f32 + (game.player.box_es.walkbox.y/2) as f32, 
            BOT_WALL as f32 - (game.player.box_es.walkbox.y/2) as f32
        );

        // TODO: Goal is to generalize hitbox data into a trait so that we can condense logic

        // Maintain enemy bounds for the room and check player collisions
        let mut enemy_list = game.current_room().enemies.clone();

        let mut live_count = 0;
        for enemy in enemy_list.iter_mut() {
            if enemy.death == false{
                live_count += 1;
            }
        }

        for enemy in enemy_list.iter_mut() {

            enemy.lastpos = enemy.pos; //Update the last position
            enemy.pos.x = enemy.pos.x.clamp(
                (LEFT_WALL as f32 + (enemy.box_es.walkbox.x * 4) as f32) - TILE_WIDTH as f32,
                (RIGHT_WALL as f32 - (enemy.box_es.walkbox.x * 4) as f32) + TILE_WIDTH as f32
            );
            enemy.pos.y = enemy.pos.y.clamp(
                (TOP_WALL as f32 + (enemy.box_es.walkbox.y * 4) as f32) - TILE_WIDTH as f32, 
                (BOT_WALL as f32 - (enemy.box_es.walkbox.y * 4) as f32) + TILE_WIDTH as f32
            ); 
            
            let player_test = game.player.box_es.get_hitbox(game.player.pos);
            // If the test enemy is in the current room of the player...

            if !enemy.death() {
                
                //If enemy is attacking
                if(enemy.recently_attacked()) {
                    //See if player collides with attackbox
                    let enemy_attack = enemy.box_es.get_attackbox(enemy.pos, enemy.dir);
                    if (player_test.has_intersection(enemy_attack)) {
                        //Enemy attacked player
                        game.player.take_damage(2, P_INVINCIBILITY_TIME);
                        if game.player.death() {
                            *menu = MenuState::GameOver;
                        }
                    }
                }
                
                // If the test enemy's walkbox intersects with the player walkbox...
                let wb_test = enemy.box_es.get_walkbox(enemy.pos);
                // Attempt at collision with attackbox
                if game.player.is_attacking {
                    let player_attack = game.player.box_es.get_attackbox(game.player.pos, game.player.dir);
                    //let player_attack = game.player.get_attackbox_world();
                    if wb_test.has_intersection(player_attack) {
                        println!("Attack collided with enemy!");
                        enemy.damage(game.player.attack);
                        println!("damage done was {}", game.player.attack);
                        if enemy.death == true && live_count == 1
                        {
                            enemy.power = true;
                        }
                        if enemy.power == true {
                            // Place gem on enemy's current tile.
                            // TODO: Factor in walkability for tile that the gem drops on.
                            game.current_room_mut()
                                .tile_at(enemy.get_pos_x(), enemy.get_pos_y())
                                .place_gem(match enemy.kind {
                                    EnemyKind::Health => Gem::Red,
                                    EnemyKind::Speed => Gem::Blue,
                                    EnemyKind::Attack => Gem::Yellow,
                                });
                            enemy.power = false;
                        }
                    }
                }
                if game.player.using_bomb {
                    let player_bomb = game.player.box_es.get_bombbox(game.player.pos_static, game.player.dir);
                    if wb_test.has_intersection(player_bomb) {
                        println!("Bomb collided with enemy!");
                        enemy.damage(3); //Bomb deals 3 damage
                        println!("damage done was 3 from bomb");
                        if enemy.death == true && live_count == 1
                        {
                            enemy.power = true;
                        }
                        if enemy.power == true {
                            // Place gem on enemy's current tile.
                            // TODO: Factor in walkability for tile that the gem drops on.
                            game.current_room_mut()
                                .tile_at(enemy.get_pos_x(), enemy.get_pos_y())
                                .place_gem(match enemy.kind {
                                    EnemyKind::Health => Gem::Red,
                                    EnemyKind::Speed => Gem::Blue,
                                    EnemyKind::Attack => Gem::Yellow,
                                });

                            enemy.power = false;
                        }
                    }
                }


                // Then there's a collision!
                /*if wb_test.has_intersection(player_test) {
                    //Damage enemy also! For some reason
                    //enemy.damage(1);
                    // Update player invincibility window and take damage to the player.
                    // Parameters: 1 is the damage amount, 1750 is the amount of ms before the cooldown window expires
                    game.player.take_damage( ENEMY_INTERSECTION_DAMAGE, P_INVINCIBILITY_TIME );


                    // If the player is dead, update to the game over menu state
                    if game.player.death() {
                        *menu = MenuState::GameOver;
                    }
                }*/
            }
            for atk in &enemy.atk_list{
                let wb_test = atk.box_es.get_hitbox(atk.pos);
                let player_test = game.player.box_es.get_hitbox(game.player.pos);

                if wb_test.has_intersection(player_test){
                    game.player.take_damage(atk.damage, P_INVINCIBILITY_TIME);
                    if game.player.death() {
                        *menu = MenuState::GameOver;
                    }
                }

            }
        }

        core.wincan.set_draw_color(Color::RGBA(128, 0, 0, 255));
        let mut x = 0;
        let mut y = 0;
        // This can't be done with the current room function bc it returns a reference which messes up internal stuff
        for row in &game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles {
            for t in row {
                match t.walkability() {
                    Walkability::Wall | Walkability::Rock | Walkability::Pit => {
                        // Hacky af block collision that needs to be changed later
                        let opt = game.player.box_es.get_walkbox(game.player.pos).intersection(Rect::new(
                            LEFT_WALL + x * TILE_WIDTH, 
                            TOP_WALL + y * TILE_WIDTH, 
                            TILE_WIDTH as u32, 
                            TILE_WIDTH as u32
                        ));
                        for enemy in enemy_list.iter_mut() {
                            match enemy.kind {
                                EnemyKind::Speed => {},
                                _ => enemy_collision(enemy, &x, &y)
                            }
                        }
                        // increment x
                        // if we do this later it messes thing up due to the continue statement in
                        // the unboxing
                        x += 1;

                        let inter_rect = match opt {
                            Some(x) => x,
                            None => continue, // If no intersection just leave function, we're done
                        };
                        let mut x_offset = inter_rect.width() as i32;
                        let mut y_offset = inter_rect.height() as i32;

                        if game.player.pos.x < inter_rect.x() as f32 {
                            // TO THE LEFT OF ROCK
                            y_offset = 0;
                        }
                        if game.player.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32 {
                            // TO THE RIGHT OF ROCK
                            x_offset *= -1;
                            y_offset = 0;
                        }
                        if game.player.pos.y < inter_rect.y() as f32 {
                            // ABOVE ROCK
                            x_offset = 0;
                        }
                        if game.player.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32 {
                            // BELOW ROCK
                            x_offset = 0;
                            y_offset *= -1;
                        }

                        game.player.pos.x -= x_offset as f32;
                        game.player.pos.y -= y_offset as f32;
                    }

                    _ => x += 1,
                }
            }

            // Prepare for next iteration of loop
            y += 1;
            x = 0;
        }

        game.current_room_mut().enemies = enemy_list;
    }
