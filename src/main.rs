#[allow(non_camel_case_types)]
use raylib::prelude::*;


fn main() {
    //w,h,res
    let win_props: Vec<i32> = vec![200,200,10];
    let cols      = win_props[0] / win_props[2];
    let rows      = win_props[1] / win_props[2];
    let grid      = makeGrid(cols,rows      );


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
fn makeGrid(cols: i32,rows: i32){

    //let mut vec = vec![0; cols as usize];

    //let iter: usize = 0;
    //while iter < vec.len() {
        //vec.push(&vec![0; rows as usize]);
    //}
    let mut vec: Vec<Vec<u64>> = vec![vec![25,25,25];cols as usize];
    println!{"{:?}",vec};
}

