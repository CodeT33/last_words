pub mod leaves;
pub mod leaf_pool;
pub mod desktop;
pub mod consts;
pub mod game;
pub mod player;

use crate::desktop::Desktop;
use crate::leaf_pool::LeafPool;
use raylib::prelude::*;

fn main() {
    // let (mut raylib_handle, thread) = init().size(640, 480).title("Last Words").build();
    //
    // while !raylib_handle.window_should_close() {
    //     let mut d = raylib_handle.begin_drawing(&thread);
    //
    //     d.clear_background(Color::GRAY);
    // }

    let leaf_pool = LeafPool::from_path("res/leaves/leaves1.md").unwrap();
    let mut desktop = Desktop::new(&leaf_pool, 3, 2);
}
