use rand::Rng;

#[derive(PartialEq)]
struct Cell {
    id: Option<u16>,
    east: bool,
    south: bool
}

impl Cell {
    fn default() -> Self {
        Self {
            id: None,
            east: false,
            south: false
        }
    }

    fn set_id(&mut self, id: u16) {
        self.id = Some(id);
     }

    fn open(&mut self, dir: &str) {
        match dir { //TODO use an enum
            "east" => self.east = true,
            "south" => self.south = true,
            _ => todo!()
        }
    }
}

fn maze_complete(maze: &Vec<Vec<Cell>>) -> bool {
    for row in maze {
        for cell in row {
            if cell.id == None { return false };
        }
    }
    return true;
}

fn maze_display(maze: &Vec<Vec<Cell>>) {
    for _ in 0..1+maze[0].len()*2 { print!("#"); }
    println!();
    for row in maze {
        print!("#");
        for cell in row {
            print!(" ");
            if cell.east { print!(" "); }
            else { print!("#"); }
        }
        println!();
        print!("#");
        for cell in row {
            if cell.south{ print!(" "); }
            else { print!("#"); }
            print!("#");
        }
        println!();
    }

}

fn get_coord(maze: &Vec<Vec<Cell>>, cell: &Cell) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        if row.contains(cell) {
            for (x, _cell) in maze.iter().enumerate() {
                return (x, y);
            }
        }
    }
    return (0, 0); //TODO use Option ?
}

fn maze_gen(maze: &mut Vec<Vec<Cell>>) {
    let height = maze.len()-1;
    let width = maze[0].len()-1;
    let mut rng = rand::thread_rng();
    let mut cell_todo: Vec<(usize, usize)> = Vec::new();
    for y in 0..height { for x in 0..width { cell_todo.push((x, y)); } }
    while cell_todo.len() > 0 {
        let i = rng.gen_range(0..cell_todo.len());
        let (x, y) = cell_todo[i];
        let mut cell = &mut maze[y][x];
        if x != width && rng.gen() { cell.open("east"); }
        if y != height && rng.gen() { cell.open("south"); }
        cell_todo.remove(i);
    }

}

fn main() {
    let width = 50;
    let height = 15;
    let mut maze = Vec::new();//vec![vec![Cell::default(); width]; height];

    //initialization
    for _y in 0..height {
        let mut row = Vec::new();
        for _x in 0..width {
            row.push(Cell::default());
        }
        maze.push(row);
    }

    maze_gen(&mut maze);

    maze_display(&maze);

    println!("{}", maze_complete(&maze));
}
