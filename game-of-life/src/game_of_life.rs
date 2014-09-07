/**
 * A basic implementation of Conway's Game of Life to try out Rust
 *
 * Most naive implementation with a limited bidimensional array
 *
 * Testing several rust features
 * - Implementation of a struct
 * - Pattern matching
 * - Reading a file
 * - Unit testing
 *
 * Simplifications:
 * - Limited size
 * - Consider borders dead cells
 */

extern crate collections;

use std::path::Path;
use std::io::{File, Open, Read};
use std::time::duration::Duration;
use collections::string::String;


mod matrix;

struct World {
    current: ::matrix::BidimensionalMatrix<bool>,
    next: ::matrix::BidimensionalMatrix<bool>,
    num_of_rows: uint,
    num_of_columns: uint
}

impl World {
    fn new(rows: uint, columns: uint, data: Vec<bool>) -> World {
        let current = ::matrix::BidimensionalMatrix::new(rows, columns, data.clone());
        let next = ::matrix::BidimensionalMatrix::new(rows, columns, data.clone());

        World {
            current: current,
            next: next,
            num_of_rows: rows,
            num_of_columns: columns
        }
    }


    fn draw(&self) {
        // Clear screen
        print!("\x1b[1J");

        // Reset cursor
        print!("\x1b[;H");

        self.current.draw();
    }


    fn step(&mut self) {
        let mut next_value: bool;

        for i in range(0, self.num_of_rows) {
            for j in range(0, self.num_of_columns) {
                next_value = self.get_next_value(i, j);
                self.next.set(i, j, next_value);
            }
        }
        self.current = self.next.clone();
    }

    
    fn get_alive_neighbors(&self, i:uint, j:uint) -> uint {
        let mut pos_x: uint;
        let mut pos_y: uint;
        let mut alives = 0u;

        pos_x = (i-1) % self.num_of_rows;
        pos_y = (j-1) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i-1) % self.num_of_rows;
        pos_y = (j) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i) % self.num_of_rows;
        pos_y = (j-1) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i+1) % self.num_of_rows;
        pos_y = (j) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i) % self.num_of_rows;
        pos_y = (j+1) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i-1) % self.num_of_rows;
        pos_y = (j+1) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i+1) % self.num_of_rows;
        pos_y = (j-1) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        pos_x = (i+1) % self.num_of_rows;
        pos_y = (j+1) % self.num_of_columns;
        alives += self.current.get(pos_x, pos_y) as uint;

        alives
    }

    fn get_next_value(&self, i:uint, j:uint) -> bool {
        let is_h_border = i==0 || i==self.num_of_rows-1;
        let is_v_border = j==0 || j==self.num_of_columns-1;
        let is_alive = self.current.get(i, j);
        let alive_neighbors = self.get_alive_neighbors(i, j);

        match (is_h_border, is_v_border, is_alive, alive_neighbors) {
            // Simplification of boundaries (all dead)
            (true, _   , _   , _   )   => false,
            (_   , true, _   , _   )   => false,
            // Rule 1: Death by Under Population
            (false, false, true, 0..1) => false,
            // Rule 2: Survival
            (false, false, true, 2..3) => true,
            // Rule 3: Death by Overcrowding
            (false, false, true, _   ) => false,
            // Rule 4: Resurrection by Reproduction
            (false, false, false, 3  ) => true,
            // Complementary of 4 stays dead
            (false, false, false, _  ) => false
        }
    }
    
}


fn extract_world(d: String) -> World {

    let mut data = d.clone();

    let mut character : char;
    let mut counter = 0u;
    let mut number_of_rows = 0u;
    let mut number_of_columns = 0u;
    let mut world_data = vec!();

    loop {
        character = match data.shift_char() {
            Some(c) => c,
            None => '\0'
        };

        match character {
            '0' => {
                counter += 1u;
                world_data.push(false);
            },

            '1' => {
                counter += 1u;
                world_data.push(true);
            },

            '\n' => {
                if number_of_columns == 0 && number_of_rows == 0 {
                    number_of_columns = counter;
                }

                if counter != number_of_columns {
                    fail!("The number of columns per row is not constant!");
                }

                number_of_rows += 1;
                counter = 0u;
            },

            '\0' => {
                if counter > 0 {
                    number_of_rows += 1;
                }

                break;
            },
            _ => {
                fail!("Invalid map format");
            }
        };
    }

    World::new(number_of_rows, number_of_columns, world_data)
}


fn create_world_from_file(file_name: &str) -> World  {
    let path : Path    = Path::new(file_name);

    let mut reader = match File::open_mode(&path, Open, Read) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    let data = match reader.read_to_string() {
        Ok(text) => text,
        Err(e) => fail!("read error: {}", e),
    };

    extract_world(data)
}

fn main() {
    // Constants
    let TIME_STEP = Duration::milliseconds(100);

    // Create world
    let mut world = create_world_from_file("conway-data.dat");

    world.draw();

    loop {
        // Step world
        world.step();

        // Draw
        world.draw();

        // Sleep
        std::io::timer::sleep(TIME_STEP);
    }
}


//////////////////
/// UNIT TESTS ///
//////////////////

#[test]
fn test_extract_world() {
    let world = extract_world("01\n11".to_string());
    println!("Number of rows {}", world.num_of_rows);
    assert!(world.num_of_rows == 2u);
    assert!(world.num_of_columns == 2u);
}

#[test]
fn test_extract_world_with_empty_line_at_the_end() {
    let world = extract_world("01\n11\n".to_string());
    println!("Number of rows {}", world.num_of_rows);
    assert!(world.num_of_rows == 2u);
    assert!(world.num_of_columns == 2u);
}


#[test]
fn test_get_next_for_rule_1() {
    let world = World::new(3u, 3u, vec!(
            false, false, false,
            false, true, true,
            false, false, false));

    assert!(!world.get_next_value(1u, 1u));
}

#[test]
fn test_get_next_for_rule_2() {
    let world = World::new(3u, 3u, vec!(
            false, false, false,
            false, true, true,
            false, false, true));

    assert!(world.get_next_value(1u, 1u));
}

#[test]
fn test_get_next_for_rule_3() {
    let world = World::new(3u, 3u, vec!(
            true, true, true,
            false, true, true,
            false, false, false));

    assert!(!world.get_next_value(1u, 1u));
}

#[test]
fn test_get_next_for_rule_4() {
    let world = World::new(3u, 3u, vec!(
            false, true, true,
            false, false, true,
            false, false, false));

    assert!(world.get_next_value(1u, 1u));
}

#[test]
fn test_calculate_alives() {
    let world = World::new(5u, 5u, vec!(
            false, false, true, false, true,
            false, false, true, false, true,
            false, true, true, false, true,
            false, true, true, true, true,
            false, true, true, false, true));
    assert!(world.get_alive_neighbors(1u, 1u) == 4);
    assert!(world.get_alive_neighbors(2u, 2u) == 5);
    assert!(world.get_alive_neighbors(3u, 3u) == 6);
}
