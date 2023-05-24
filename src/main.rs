#[allow(non_camel_case_types)]
use rand::Rng;
use raylib::prelude::*;


fn main() {
    let mut rng = rand::thread_rng();
    let win_props: Vec<i32> = vec![200,200,10];     //w,h,res
    let cols      = win_props[0] / win_props[2];
    let rows      = win_props[1] / win_props[2];
    let mut grid      = makeGrid(cols,rows);

    for i in 0..cols {
        for j in 0..rows {
            grid[i as usize][j as usize] = rng.gen_range(0 ..2);
        }
    }

    let (mut rl, thread) = open_window(win_props,"Input");
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}



fn open_window(win_props: Vec<i32>, name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread) {
        let (mut rl, thread) = raylib::init()
            .size(win_props[0].into(),win_props[1].into())
            .title(name)
            .build();
        rl.set_target_fps(60);
        (rl, thread)
}

//TODO:migrate to matrices for a more largebra oriented design
fn makeGrid(cols: i32,rows: i32)-> Vec<Vec<u64>> {
    let mut vec: Vec<Vec<u64>> = vec![vec![0; rows as usize];cols as usize];
    return vec;
}
