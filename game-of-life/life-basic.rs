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
use std::path::Path;
use std::io;
use std::io::fs::File;
use std::io::buffered::BufferedReader;


struct World {
    current: ~[~[bool]],
    next: ~[~[bool]],
    num_of_rows: uint,
    num_of_columns: uint
}

impl World {
    fn new(map:~[~[bool]]) -> World {
        World {
            current: map.clone(),
            next: map.clone(),
            num_of_rows: map.len(),
            num_of_columns: map[0].len()
        }
    }

    fn draw(&self) {
        // Clear screen
        print!("\x1b[1J");

        // Reset cursor
        print!("\x1b[;H");

        for row in self.current.iter() {
            for cell in row.iter() {
                match *cell {
                    true  => print!("*"),
                    false => print!("·")
                }
            }
            print!("\n");
        }
    }

    fn step(&mut self) {

        for i in range(0, self.num_of_rows) {
            for j in range(0, self.num_of_columns) {
                self.next[i][j] = self.get_next_value(i, j);
                if self.current[i][j] {
                    format!("{:u} - {:u} is alive and next is {:b}", i, j, self.next[i][j]);
                }
            }
        }
        self.current = self.next.clone();
    }

    fn get_alive_neighbors(&self, i:uint, j:uint) -> uint {
        let mut alives = 0u;

        alives += self.current
                    [(i-1)%self.num_of_rows]
                    [(j-1)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i-1)%self.num_of_rows]
                    [(j)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i)%self.num_of_rows]
                    [(j-1)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i+1)%self.num_of_rows]
                    [(j)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i)%self.num_of_rows]
                    [(j+1)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i-1)%self.num_of_rows]
                    [(j+1)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i+1)%self.num_of_rows]
                    [(j-1)%self.num_of_columns]
                    .to_bit();

        alives += self.current
                    [(i+1)%self.num_of_rows]
                    [(j+1)%self.num_of_columns]
                    .to_bit();

        alives
    }

    fn get_next_value(&self, i:uint, j:uint) -> bool {
        let is_h_border = i==0 || i==self.num_of_rows-1;
        let is_v_border = j==0 || j==self.num_of_columns-1;
        let is_alive = self.current[i][j];
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

fn create_world_from_file(file_name: ~str) -> World {
    let path : Path    = Path::new(file_name);
    let on_error       = || fail!("open of {:?} failed", path);
    let freader : File = File::open(&path).unwrap_or_else(on_error);

    let mut reader = BufferedReader::new(freader);

    let mut world_map = ~[]; 
    for line in reader.lines() {
        let mut row = ~[];
        for cell in line.chars() {
            let cell_value = match cell {
                '1' => true,
                _   => false
            };
            row.push(cell_value);
        }
        world_map.push(row);
    }

    World::new(world_map)
}

fn main() {
    // Constants
    let TIME_STEP = 100;

    // Create world
    let mut world = create_world_from_file(~"conway-data.dat");
    world.next[0][1] = true;

    world.draw();

    loop {
        // Step world
        world.step();

        // Draw
        world.draw();

        // Sleep
        io::timer::sleep(TIME_STEP);
    }
}


//////////////////
/// UNIT TESTS ///
//////////////////
#[test]
fn test_get_next_for_rule_1() {
    let map = ~[
        ~[false, false, false],
        ~[false, true, true],
        ~[false, false, false],
    ];

    let world = World::new(map);
    assert!(!world.get_next_value(1u, 1u));
}

#[test]
fn test_get_next_for_rule_2() {
    let map = ~[
        ~[false, false, false],
        ~[false, true, true],
        ~[false, false, true],
    ];

    let world = World::new(map);
    assert!(world.get_next_value(1u, 1u));
}

#[test]
fn test_get_next_for_rule_3() {
    let map = ~[
        ~[true, true, true],
        ~[false, true, true],
        ~[false, false, false],
    ];

    let world = World::new(map);
    assert!(!world.get_next_value(1u, 1u));
}

#[test]
fn test_get_next_for_rule_4() {
    let map = ~[
        ~[false, true, true],
        ~[false, false, true],
        ~[false, false, false],
    ];

    let world = World::new(map);
    assert!(world.get_next_value(1u, 1u));
}

#[test]
fn test_calculate_alives() {
    let map = ~[
        ~[false, false, true, false, true],
        ~[false, false, true, false, true],
        ~[false, true, true, false, true],
        ~[false, true, true, true, true],
        ~[false, true, true, false, true]
     ];

    let world = World::new(map);
    assert!(world.get_alive_neighbors(1u, 1u) == 4);
    assert!(world.get_alive_neighbors(2u, 2u) == 5);
    assert!(world.get_alive_neighbors(3u, 3u) == 6);
}

