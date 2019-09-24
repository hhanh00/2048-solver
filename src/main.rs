extern crate rand;

use rand::prelude::*;

const nsamples: usize = 500;

#[derive(Debug)]
struct Board {
    cells: [i32; 16],
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up, Down, Left, Right
}

impl Clone for Board {
    fn clone(&self) -> Self {
        return Board { 
            cells: self.cells
        }
    }
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [0; 16]
        }
    }

    fn get(&self, i: usize, j: usize) -> i32 {
        self.cells[i*4+j]
    }

    fn set(&mut self, i: usize, j: usize, v: i32) {
        self.cells[i*4+j] = v;
    }

    fn print(&self) {
        for i in 0..4 {
            for j in 0..4 {
                print!("{} ", self.get(i, j))
            }
            println!();
        }
        println!("****")
    }

    fn get_xy(&self, dir: Dir, i: usize, j: usize) -> usize {
        match dir {
            Dir::Left => i*4+j,
            Dir::Right => i*4+3-j,
            Dir::Up => j*4+i,
            Dir::Down => (3-j)*4+i
        }
    }

    fn swipe_move(&mut self, dir: Dir) -> Option<i32> {
        let mut moved = false;
        let mut score = 0;
        for i in 0..4 {
            let mut edge = 0;
            for j in 0..4 {
                let c = self.cells[self.get_xy(dir, i, j)];
                
                if c == 0 {
                    continue;
                }

                let mut j2 = edge;
                for k in (edge..j).rev() {
                    let c2 = self.cells[self.get_xy(dir, i, k)];
                    if c2 == 0 {
                        continue;
                    }

                    j2 = 
                        if c == c2 {
                            edge = k + 1;
                            k
                        }
                        else {
                            k + 1
                        };
                    break;
                }

                if j != j2 {
                    self.cells[self.get_xy(dir, i, j)] = 0;
                    let c2_offset = self.get_xy(dir, i, j2);
                    score += self.cells[c2_offset];
                    self.cells[c2_offset] += c;
                    moved = true
                }
            }
        }
        return if moved { Some(score) } else { None };
    }

    fn spawn(&mut self, rng: &mut ThreadRng) {
        let c_empty = self.cells.iter().filter(|&&x| x == 0).count();
        let mut k = rng.gen_range(0, c_empty);
        let mut i = 0;
        loop {
            while self.cells[i] != 0 {
                i += 1;
            }
            if k == 0 {
                break
            }
            k = k - 1;
        }
        let x: f64 = rng.gen();
        self.cells[i] = if x < 0.9 { 2 } else { 4 };
    }

    fn random_actions(&mut self, rng: &mut ThreadRng) -> Option<i32> {
        let actions = [ Dir::Up, Dir::Down, Dir::Left, Dir::Right ];
        let random_actions = actions.choose_multiple(rng, 4);
        for action in random_actions {
            match self.swipe_move(*action) {
                Some(score) => return Some(score),
                None => ()
            }
        }
        return None
    }

    fn eval(&mut self, rng: &mut ThreadRng) -> i32 {
        let mut total_score = 0;
        loop {
            let score = self.random_actions(rng);
            match score {
                Some(s) => total_score += s,
                None => break
            }
            self.spawn(rng);
        }
        return total_score;
    }

    fn run(&mut self, rng: &mut ThreadRng) -> i32 {
        self.spawn(rng);
        return self.eval(rng);
    }

    fn best_move(&mut self, rng: &mut ThreadRng) -> Option<Dir> {
        let mut best_score = -1;
        let mut best_move = None;

        let actions = [ Dir::Up, Dir::Down, Dir::Left, Dir::Right ];
        for action in &actions {
            let mut b2 = self.clone();
            let m = b2.swipe_move(*action);
            let mut base_score;
            match m {
                Some(s) => base_score = s,
                None => continue
            }
            let mut score = 0;
            for i in 0..nsamples {
                let mut b3 = b2.clone();
                score += base_score + b3.run(rng);
            }
            if score > best_score { 
                best_score = score;
                best_move = Some(*action);
            }
        }
        return best_move;
    }
}

fn main() {
    let mut board = Board::new();
    let mut rng = thread_rng();

    board.print();
    board.spawn(&mut rng);

    loop {
        board.spawn(&mut rng);
        board.print();
        let best_move = board.best_move(&mut rng);
        println!("{:?}", best_move);
        let _ = match best_move {
            Some(m) => board.swipe_move(m),
            _ => break
        };
    }
}
