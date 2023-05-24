#[allow(non_camel_case_types)]
use rand::Rng;
use raylib::prelude::*;

//TODO: add customization menu using egui

fn main() {
    let mut rng = rand::thread_rng();
    let win_props: Vec<i32> = vec![640,480,10];     //w,h,res
    let cols : i32 = win_props[0] / win_props[2];
    let rows : i32 = win_props[1] / win_props[2];
    let mut grid = make_grid(cols,rows);

    for i in 0..cols {
        for j in 0..rows {
            grid[i as usize][j as usize] = rng.gen_range(0 ..2);
        }
    }

    let (mut rl, thread) = open_window(win_props.clone(),"Input");
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for i in 0..cols {
            for j in 0..rows {
                let x = i * win_props[2];
                let y = j * win_props[2];
                if grid[i as usize][j as usize] == 1 {
                    d.draw_rectangle(x, y, win_props[2] - 1, win_props[2] - 1, Color::WHITE);
                }          
            }
        }

        let mut next = make_grid(cols,rows);

        for i in 0..cols {
            for j in 0..rows {
            let state = grid[i as usize][j as usize];
            // Count live neighbors!
            let sum = 0;
            let neighbors = count_neighbors(grid.clone(), i, j,cols,rows);

            if state == 0 && neighbors == 3 {
                next[i as usize][j as usize] = 1;
            } else if state == 1 && (neighbors < 2 || neighbors > 3) {
                next[i as usize][j as usize] = 0;
            } else {
                next[i as usize][j as usize] = state;
            }
    }
  }

        grid = next;

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
fn make_grid(cols: i32,rows: i32)-> Vec<Vec<u64>> {
    let mut vec: Vec<Vec<u64>> = vec![vec![0; rows as usize];cols as usize];
    return vec;
}

fn count_living_neighbors(grid: Vec<Vec<u64>>, x: i32, y: i32,cols: i32, rows: i32) -> u64 {
  let mut sum = 0;
  for i in 0..3 {
    for j in 0..3 {
      let col = (x + i-1 + cols) % cols;
      let row = (y + j-1 + rows) % rows;
      sum = sum + grid[col as usize][row as usize];
    }
  }
  sum = sum - grid[x as usize][y as usize];
  return sum;
}

