use std::{thread, time};

fn main() {
    let mut board = Board::new(20, 40);
    board.turn_on(9, 20);
    board.turn_on(10, 19);
    board.turn_on(10, 20);
    board.turn_on(10, 21);
    board.turn_on(11, 19);
    board.turn_on(11, 21);
    board.turn_on(12, 20);

    let mut normal_printer = PrintNormal {};

    loop {
        board.print(&mut normal_printer);
        board.update();
        let sixty_millis = time::Duration::from_millis(260);
        thread::sleep(sixty_millis);
    }
}

trait Print {
    fn print_char(&mut self, character: char);
}

#[derive(Clone)]
struct Board {
    rows: usize,
    cols: usize,
    cells: Vec<bool>,
}

struct PrintNormal;
impl Print for PrintNormal {
    fn print_char(&mut self, character: char) {
        print!("{}", character);
    }
}

struct PrintChecker {
    characters: Vec<char>,
}

impl PrintChecker {
    fn new() -> Self {
        let characters = Vec::new();
        PrintChecker { characters }
    }
}

impl Print for PrintChecker {
    fn print_char(&mut self, character: char) {
        self.characters.push(character);
    }
}

fn safe(val: i64, extent: i64) -> i64 {
    (val + extent) % extent
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        let mut cells: Vec<bool> = Vec::new();
        cells.resize_with(rows * cols, Default::default);
        Board { rows, cols, cells }
    }

    fn get_neighbours(&self, r: usize, c: usize) -> usize {
        let directions = [
            (-1i64, 0i64),
            (-1i64, -1i64),
            (0i64, -1i64),
            (1i64, -1i64),
            (1i64, 0i64),
            (1i64, 1i64),
            (0i64, 1i64),
            (-1i64, 1i64),
        ];

        let mut neighbour_count = 0;
        for &dir in directions.iter() {
            if self.is_on(
                safe(r as i64 + dir.0, self.rows as i64) as usize,
                safe(c as i64 + dir.1, self.cols as i64) as usize,
            ) {
                neighbour_count += 1;
            }
        }

        neighbour_count
    }

    fn turn_on(&mut self, r: usize, c: usize) {
        let index = self.index(r, c);
        self.cells[index] = true;
    }

    fn turn_off(&mut self, r: usize, c: usize) {
        let index = self.index(r, c);
        self.cells[index] = false;
    }

    fn change(&mut self, r: usize, c: usize, on: bool) {
        let index = self.index(r, c);
        self.cells[index] = on;
    }

    fn is_on(&self, r: usize, c: usize) -> bool {
        self.cells[self.index(r, c)]
    }

    fn is_off(&self, r: usize, c: usize) -> bool {
        !self.cells[self.index(r, c)]
    }

    fn index(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    fn update(&mut self) {
        let prev = self.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let neighbours = prev.get_neighbours(row, col);
                if self.is_on(row, col) && neighbours < 2 || neighbours > 3 {
                    self.turn_off(row, col);
                } else {
                    if self.is_off(row, col) && neighbours == 3 {
                        self.turn_on(row, col);
                    }
                }
            }
        }
    }

    fn print(&self, printer: &mut Print) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.is_on(row, col) {
                    printer.print_char('@');
                } else {
                    printer.print_char('*');
                }
            }
            print!("\n");
        }
        print!("{}[2J", 27 as char);
    }
}

#[test]
fn cell_has_correct_number_of_neighbours() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 1);
    board.turn_on(1, 1);
    board.turn_on(1, 0);
    assert_eq!(board.get_neighbours(0, 0), 3);
    assert_eq!(board.get_neighbours(5, 5), 0);
}

#[test]
fn cell_dies_with_fewer_than_two_neighbours() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 0);
    board.turn_on(0, 1);
    board.update();
    assert_eq!(board.is_on(0, 1), false);
}

#[test]
fn cell_lives_with_two_neighbours() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 0);
    board.turn_on(0, 1);
    board.turn_on(0, 2);
    board.update();
    assert!(board.is_on(0, 1));
}

#[test]
fn cell_dies_with_more_than_three_neighbours() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 0);
    board.turn_on(0, 1);
    board.turn_on(0, 2);
    board.turn_on(1, 0);
    board.turn_on(1, 1);
    board.update();
    assert!(board.is_off(1, 1));
}

#[test]
fn cell_lives_with_three_neighbours() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 0);
    board.turn_on(0, 1);
    board.turn_on(0, 2);
    board.turn_on(1, 1);
    board.update();
    assert!(board.is_on(0, 1));
}

#[test]
fn cell_turns_on_with_three_neighbours() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 0);
    board.turn_on(0, 1);
    board.turn_on(0, 2);
    board.update();
    assert!(board.is_on(1, 1));
}

#[test]
fn change_cell_to_alive() {
    let mut board = Board::new(10, 10);
    board.turn_on(0, 0);
    assert_eq!(board.is_on(0, 0), true);
}

#[test]
fn change_cell_to_dead() {
    let mut board = Board::new(10, 10);
    let r = 3;
    let c = 4;
    assert_eq!(board.is_off(r, c), true);
    board.turn_on(r, c);
    assert_eq!(board.is_off(r, c), false);
    assert_eq!(board.is_on(r, c), true);
    board.turn_off(r, c);
    assert_eq!(board.is_off(r, c), true);
}

#[test]
fn ensure_cells_do_not_match() {
    let mut board = Board::new(10, 10);

    {
        let r = 3;
        let c = 4;
        assert_eq!(board.is_off(r, c), true);
        board.turn_on(r, c);
        assert_eq!(board.is_off(r, c), false);
        assert_eq!(board.is_on(r, c), true);
    }

    {
        let r = 6;
        let c = 7;
        assert_eq!(board.is_off(r, c), true);
        board.turn_on(r, c);
        assert_eq!(board.is_off(r, c), false);
        assert_eq!(board.is_on(r, c), true);
    }
}

#[test]
fn board_initialized_off() {
    let board = Board::new(10, 10);
    for r in 0..10 {
        for c in 0..10 {
            assert_eq!(board.is_on(r, c), false);
        }
    }
}

#[test]
fn value_is_always_safe() {
    let safe_index_1 = safe(-1, 10);
    assert_eq!(safe_index_1, 9);
    let safe_index_2 = safe(11, 10);
    assert_eq!(safe_index_2, 1);
}

#[test]
fn board_is_printed() {
    let board = Board::new(5, 5);
    let mut print_checker = PrintChecker::new();
    board.print(&mut print_checker);

    assert_eq!(print_checker.characters.len(), 25);

    for &character in print_checker.characters.iter() {
        assert_eq!(character, '*');
    }
}
