use rand::Rng;
use std::collections::HashSet;
use std::fmt;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgCell {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
impl RgCell {
    pub fn tick(&mut self, width: u32, height: u32) {
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..4);
        self.x = ((self.x as i32 + width as i32 + dirs[r].0) % width as i32) as u32;
        self.y = ((self.y as i32 + height as i32 + dirs[r].1) % height as i32) as u32
    }
}

#[wasm_bindgen]
pub struct RandGame {
    width: u32,
    height: u32,
    cells: Vec<RgCell>,
    count: Vec<i32>, // 当前位置的小人个数
}

#[wasm_bindgen]
impl RandGame {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn count(&self) -> *const i32 {
        self.count.as_ptr()
    }
}

// 要导出的方法
#[wasm_bindgen]
impl RandGame {
    pub fn tick(&mut self) {
        let width = self.width;
        for cell in &mut self.cells {
            let idx = (cell.x * width + cell.y) as usize;
            self.count[idx] -= 1;
            cell.tick(self.width, self.height);
            let idx = (cell.x * width + cell.y) as usize;
            self.count[idx] += 1;
        }
    }

    pub fn new(mut cnt: u32) -> Self {
        let width = 64;
        let height = 64;
        if cnt > width * height {
            cnt = width * height;
        }

        let mut count: Vec<i32> = (0..width * height).map(|_| 0).collect();
        let cells = (0..cnt)
            .map(|i| {
                let x = i / width;
                let y = i % width;
                let idx = (x * width + y) as usize;
                count[idx] = 1;
                RgCell { x, y }
            })
            .collect();

        RandGame {
            width,
            height,
            cells,
            count,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RandGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m = HashSet::new();
        for cell in &self.cells {
            m.insert((cell.x, cell.y));
        }
        for x in 0..self.height {
            for y in 0..self.width {
                let symbol = if m.contains(&(x, y)) { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
