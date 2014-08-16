static rows: uint = 20;
static cols: uint = 20;

// only downside is reading/writing mutable statics requires unsafe blocks
static mut grid: [[State, ..cols], ..rows] = [[Dead, ..cols], ..rows];

enum State {
    Dead,
    Alive,
}

// n - number of alive neighbors
// state - current state of cell
fn cell_transition(n: uint, state: State) -> State {
    match (n, state) {
        (3, Dead) => Alive,
        (_, Dead) => Dead,
        (n, Alive) if n < 2 => Dead,
        (n, Alive) if n > 3 => Dead,
        (_, Alive) => Alive,
    }
}

fn alive_neighbors(i: uint, j: uint) -> uint {
    fn get_range(n: uint, size: uint) -> (uint, uint) {
        if n == 0 {
            (n, n + 1)
        } else if n == size - 1 {
            (n - 1, n)
        } else {
            (n - 1, n + 1)
        }
    }

    let (col_start, col_end) = get_range(j, cols);
    let (row_start, row_end) = get_range(i, rows);

    let mut count = 0;
    for a in range(row_start, row_end + 1) {
        for b in range(col_start, col_end + 1) {
            if a != i || b != j {
                unsafe {
                    match grid[a][b] {
                        Alive => count += 1,
                        _ => {},
                    }
                }
            }
        }
    }

    count
}

fn initialize_grid(init: Vec<(uint, uint)>) {
    for &(i, j) in init.iter() {
        unsafe { grid[i][j] = Alive; }
    }
}

fn step() {
    let mut new = [[Dead, ..cols], ..rows];
    for i in range(0, rows) {
        for j in range(0, cols) {
            unsafe {
                new[i][j] = cell_transition(alive_neighbors(i,j), grid[i][j])
            }
        }
    }

    unsafe { grid = new }
}

fn print_grid() {
    let mut sep = String::from_str("");
    for i in range(0, cols) {
        if i > 0 { sep.push_char('-') }
        sep.push_char('-');
    }

    println!("{}", sep);

    for i in range(0, rows) {
        for j in range(0, cols) {
            if j > 0 { print!(" ") }
            unsafe {
                match grid[i][j] {
                    Alive => print!("#"),
                    Dead  => print!(" "),
                }
            }
        }
        println!("");
    }

    println!("{}", sep);
}

fn main() {
    // initialize glider
    initialize_grid(vec!((3, 2), (3, 3), (3, 4), (2, 4), (1, 3)));

    print_grid();

    for _ in range(0u, 8) {
        step();
        print_grid();
    }
}
