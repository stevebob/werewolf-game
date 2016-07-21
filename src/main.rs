#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use] mod ecs;
#[macro_use] mod debug;
mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
mod game;
mod terminal;
mod allocator;
mod tests;

use ecs::entity_table::EntityTable;
use ecs::entity_types::*;
use ecs::message::Field;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use ecs::entity::EntityId;
use ecs::system::{System, SystemName};
use ecs::systems::window_renderer::WindowRenderer;
use ecs::system_queue::SystemQueue;

use terminal::window_manager::WindowManager;
use terminal::window_buffer::WindowBuffer;

use std::io;

const LEVEL_WIDTH: usize = 6;
const LEVEL_HEIGHT: usize = 4;

fn populate(entities: &mut EntityTable) -> Option<EntityId> {
    let level_id = entities.add(make_level(LEVEL_WIDTH, LEVEL_HEIGHT));

    for y in 0..LEVEL_HEIGHT {
        for x in 0..LEVEL_WIDTH {

            let floor = entities.add(make_floor(x as isize, y as isize));
            if let Some(&mut Level(ref mut level)) = entities.get_mut(level_id).get_mut(Type::Level) {
                level.add(floor);
            }

            if x == 0 || x == LEVEL_WIDTH - 1 || y == 0 || y == LEVEL_HEIGHT - 1 {
                let wall = entities.add(make_wall(x as isize, y as isize));
                if let Some(&mut Level(ref mut level)) = entities.get_mut(level_id).get_mut(Type::Level) {
                    level.add(wall);
                }
            }
        }
    }

    let pc = entities.add(make_pc(3, 2));
    if let Some(&mut Level(ref mut level)) = entities.get_mut(level_id).get_mut(Type::Level) {
        level.add(pc);
        Some(level_id)
    } else {
        None
    }
}

const DEBUG_WINDOW_WIDTH: usize = 40;
const DEBUG_WINDOW_HEIGHT: usize = 8;

fn main() {

    let wm = terminal::window_manager::WindowManager::new().unwrap();

    let input_source = wm.make_input_source();

    // Initialise debug window
    let mut debug_buffer = make_debug_window(&wm, DEBUG_WINDOW_WIDTH,
                                                  DEBUG_WINDOW_HEIGHT);
    debug::debug::init(&mut debug_buffer as &mut io::Write);


    let game_window = wm.make_window(0, 0, 80, 20);

    let mut systems = SystemQueue::new();
    {
        systems.add(SystemName::Renderer, System::WindowRenderer(WindowRenderer::new(game_window)));
    }

    let mut entities = EntityTable::new();

    if let Some(level_id) = populate(&mut entities) {

        let mut message = message![
            Field::RenderLevel { level: level_id },
        ];

        for system in systems.iter() {
            system.borrow_mut().process_message(&mut message, &mut entities, &systems);
        }
    }

    debug_println!("{:?}", systems);

    input_source.get_event().unwrap();
}



fn make_debug_window<'a>(wm: &'a WindowManager, width: usize, height: usize)
    -> WindowBuffer<'a>
{
    let debug_buffer = wm.make_window_buffer(
        (wm.get_width() - width) as isize,
        (wm.get_height() - height) as isize,
        width, height, 2, 1);

    debug_buffer.draw_borders();

    debug_buffer
}
