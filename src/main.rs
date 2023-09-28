use std::env;
use std::io::stdin;
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
    let wall = "\u{2592}";
    for _ in 0..1+maze[0].len()*2 { print!("{wall}"); }
    println!();
    for row in maze {
        print!("{wall}");
        for cell in row {
            print!(" ");
            if cell.east { print!(" "); }
            else { print!("{wall}"); }
        }
        println!();
        print!("{wall}");
        for cell in row {
            if cell.south{ print!(" "); }
            else { print!("{wall}"); }
            print!("{wall}");
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

fn trans_id(v: Option<u16>) -> u16 {
    match v {
        Some(v) => v,
        None => 999
    }
}

fn maze_gen(maze: &mut Vec<Vec<Cell>>) {
    let height = maze.len();
    let width = maze[0].len();
    let mut rng = rand::thread_rng();

    let mut cell_todo: Vec<(usize, usize)> = Vec::new();
    for y in 0..height { for x in 0..width { cell_todo.push((x, y)); } }

    let mut ii = 0;
    while cell_todo.len() > 0 {
        let i = rng.gen_range(0..cell_todo.len());
        let (x, y) = cell_todo[i];
        //println!("chose cell at {}:{}", x, y);
        let id = match maze[y][x].id {
            Some(n) => n,
            None => { ii += 1;
                ii
            }
        };

        //get a list of neighbours
        let mut neighbours = Vec::new();
        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            //filter on id (must be a different one)
            let (nx, ny) = (x as i16 + dx, y as i16 + dy);
            if nx < 0 || ny < 0 { continue; }
            let (nx, ny): (usize, usize) = (nx.try_into().unwrap(), ny.try_into().unwrap());
            if maze.get(ny) != None && maze[ny].get(nx) != None
                && (maze[ny][nx].id == None || maze[y][x].id == None
                || maze[ny][nx].id != maze[y][x].id) {
                neighbours.push((nx, ny));
                //println!("nn {}:{}", nx, ny);
            }
        }
        /*
        for (dx, dy) in &neighbours {
            println!("neighbour at {}:{} ({}x{})", dx, dy,
                match maze[*dy][*dx].id {
                    Some(v) => v,
                    None => 999
                }
                , match maze[y][x].id {
                    Some(v) => v,
                    None => 999
                });
        }
        */
        if !neighbours.is_empty() {
            //open path
            let i = rng.gen_range(0..neighbours.len());
            let (nx, ny) = neighbours.remove(i);
            //println!("chosen : {}; {}:{}", i, nx, ny);
            //Propagate ID
            let oid = maze[ny][nx].id;
            maze[ny][nx].set_id(id);
            if oid != None {
                //println!("{} -> {}", trans_id(oid), id);
                for (px, py) in &cell_todo { if maze[*py][*px].id == oid {maze[*py][*px].set_id(id);} }
            }
            if nx > x { maze[y][x].open("east"); }
            else if ny > y { maze[y][x].open("south"); }
            else if nx < x { maze[ny][nx].open("east"); }
            else if ny < y { maze[ny][nx].open("south"); }
        }
        maze[y][x].set_id(id);
        //Remove cell from todo if every neighbour has the same id
        if neighbours.is_empty() { cell_todo.remove(i); }

        /* DEBUG
        println!("TODO");
        for (x, y) in &cell_todo {
            println!("todo {}:{} {}", x, y, match maze[*y][*x].id
                    { Some(v) => v,
                    None => 999
                    }
                    );
        }
        maze_display(&maze);
        let mut s = String::new();
        let _ = stdin().read_line(&mut s);
        */
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: usize = args[1].parse().expect("int expected in arg1");
    let height: usize = args[2].parse().expect("int expected in arg2");
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
}
