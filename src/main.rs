use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
    terminal::{disable_raw_mode, enable_raw_mode}, queue
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> Result<()>{

    let mut stdout = stdout();

    enable_raw_mode();

    let mut game = Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);

    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent {code, .. }) = read()? {
                match code {
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            queue!(stdout, Clear(ClearType::All))?;
            let mut i = 0;
            while let Some(line) = game.row_as_string(i) {
                queue!(stdout, MoveTo(0, i as u16), Print(line))?;
                i += 1;
            }

            queue!(
                stdout,
                MoveTo(0, (i + 1) as u16),
                Print("Press Esc to exit...")
            )?;
            stdout.flush()?;
            game.tick();
        }
    }
    execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                next[idx] = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cells = next;
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.height;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn row_as_string(&self, row: u32) -> Option<String> {
        if row < self.height {
            let mut row_string = String::new();
            let start = self.get_index(row, 0);
            let end = self.get_index(row, self.width);
            let line = &self.cells[start..end];
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                row_string.push(symbol);
                row_string.push(' ')
            }
            Some(row_string)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, " {}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
