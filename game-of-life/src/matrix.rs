pub struct BidimensionalMatrix<T> {
    rows: uint,
    columns: uint,
    data: Vec<T>
}

impl BidimensionalMatrix<bool> {
    pub fn new(rows: uint, columns: uint, data: Vec<bool>) -> BidimensionalMatrix<bool> {
        BidimensionalMatrix {
            rows: rows,
            columns: columns,
            data: data
        }
    }


    pub fn get(&self, x: uint, y: uint) -> bool {
        self.data[x * self.columns + y]
    }


    pub fn set(&mut self, x: uint, y: uint, value: bool) {
        let position = x * self.columns + y;
        self.data.insert(position, value); 
        self.data.remove(position + 1);
    }


    pub fn draw(&self) {
        for row in range(0, self.rows-1) {
            for column in range(0, self.columns-1) {
                print!("{}", self.get(row, column) as uint);
            }
            print!("\n");
        }
    }


    pub fn clone(&self) -> BidimensionalMatrix<bool> {
        return BidimensionalMatrix::new(self.rows, self.columns, self.data.clone());
    }
}

#[test]
fn test_get_element() {
    let array = BidimensionalMatrix::new(2u, 2u, vec!(true, false, true, false));
    assert!(array.get(0, 0) == true);
    assert!(array.get(0, 1) == false);
    assert!(array.get(1, 0) == true);
    assert!(array.get(1, 1) == false);
}


#[test]
fn test_set_elements() {
    let mut array = BidimensionalMatrix::new(2u, 2u, vec!(true, false, true, false));
    assert!(array.get(0, 1) == false);
    array.set(0, 1, true);
    assert!(array.get(0, 1) == true);
}

