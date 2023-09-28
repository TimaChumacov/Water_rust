//cd Documents/rust/water
use std::{
    io,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use std::cmp::Ordering;
use clearscreen;
use rand::Rng;

use eframe::egui;

use egui::Vec2;

use egui::{FontFamily, FontId, RichText, TextStyle};
use crate::FontFamily::Monospace;

#[derive(Default)]
struct MyEguiApp {
    max_grid_x: i32, // the grid size
    max_grid_y: i32,
    grid: Vec<Vec<String>>, // grid itself. 2-dimensional vector
    row: Vec<String>, // useful when we define grid size
    is_first: bool, // first room gotta make an opening for water and etc.
    is_water_on: bool, // should the water flow. triggered with start button
    water_source: Vec<i32>, // coordinates of the point the water flows from
    texty: String, // string containing the entire grid
    frames_in_tick: i32, // water gotta move slowly so i need ticks
    frames_passed: i32,
}

impl MyEguiApp {
    fn default() -> Self {
        Self {
            max_grid_x: 75, // the grid size
            max_grid_y: 45,
            grid: vec![vec![]], // grid itself. 2-dimensional vector
            row: vec![], // useful when we define grid size
            is_first: true, // first room gotta make an opening for water and etc.
            is_water_on: false, // should the water flow. triggered with start button
            water_source: vec![], //coordinates of the point the water flows from
            texty: "noniue".to_string(), // string containing the entire grid
            frames_in_tick: 0, // water gotta move slowly so i need ticks
            frames_passed: 0,
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // update function fires each frame
        self.max_grid_x = 70; 
        self.max_grid_y = 45; 
        self.frames_in_tick = 5;
        //self.water_source = vec![];

        if self.grid.len() == 0 {
        for y in 0..self.max_grid_y {
            for x in 0..self.max_grid_x {
                println!("y {y} x {x},");
                self.row.insert(x as usize, "#".to_string());
            }
            self.grid.insert(y as usize, self.row.clone());
            self.row.clear();
        }
        self.texty = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y); 
        self.is_first = checkIfFirst( //so coz it updates i gotta check 
            self.grid.clone(),        //all this stuff each frame
            self.max_grid_x, 
            self.max_grid_y
        );

        if self.is_water_on {
            if self.frames_passed >= self.frames_in_tick {
                //println!("{}", self.water_source.len());
                //println!("{}", self.grid[(self.water_source[1] + 1) as usize][self.water_source[0] as usize]);
    
                self.grid = moveWater(
                    self.grid.clone(),
                    self.max_grid_x, 
                    self.max_grid_y
                );
                self.grid[(self.water_source[1] + 1) as usize][self.water_source[0] as usize] = "0".to_string();
                self.texty = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y);
                self.frames_passed = 0;
            } else {
                self.frames_passed = self.frames_passed + 1;
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new(format!("{}", self.texty)).monospace());
            if ui.button("generate layout").clicked() {
                //self.texty = format!("{}1", self.texty);
                self.grid = room(
                    self.grid.clone(),
                    self.max_grid_x, 
                    self.max_grid_y,
                    self.is_first
                );
                print!("{}", renderGrid(&self.grid, self.max_grid_x, self.max_grid_y));
                self.texty = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y);
                self.is_first = false;
            }
            if ui.button("start").clicked() {
                //println!("{}", findSource(&self.grid, self.max_grid_x, self.max_grid_y).len());
                self.water_source = findSource(&self.grid, self.max_grid_x, self.max_grid_y);
                self.is_water_on = true;
                /*for y in 0..self.max_grid_y {
                    for x in 0..self.max_grid_x {
                        self.row.insert(x as usize, "#".to_string());
                    }
                    self.grid.insert(y as usize, self.row.clone());
                    self.row.clear();
                }*/
                //println!("here {}", &mut self.grid[0][0]); //renderGrid(&self.grid, self.max_grid_x, self.max_grid_y));
            }
        });
        ctx.request_repaint();
   }
}
}

fn checkIfFirst(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> bool {
    let mut is_first = true;
    for x in 0..max_grid_x {
        for y in 0..max_grid_y {
            if grid[y as usize][x as usize] != "#".to_string() {
                is_first = false;
            }
        }
    }
    return is_first;
}

fn main() {
    let termsize::Size {rows, cols} = termsize::get().unwrap();
    let max_grid_x: i32 = (cols - 1) as i32;
    let max_grid_y: i32 = (rows - 1) as i32;
    let mut grid: Vec<Vec<String>> = vec![vec![]];
    let mut row: Vec<String> = vec![];
    let mut is_first: bool = true;
    let mut water_source: Vec<i32> = vec![];

    let native_options = eframe::NativeOptions::default();

    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(
          Vec2::new(618 as f32, 700 as f32)
        ),
        min_window_size: Option::from(
            Vec2::new(618 as f32, 700 as f32)
          ),
        max_window_size: Option::from(
            Vec2::new(618 as f32, 700 as f32)
          ),
        resizable: true,
        transparent: true,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: native_options.hardware_acceleration,
        renderer: native_options.renderer,
        follow_system_theme: true,
        default_theme: native_options.default_theme,
        run_and_return: true,
        event_loop_builder: native_options.event_loop_builder,
        shader_version: native_options.shader_version,
        centered: false,
    };
    
    let app = MyEguiApp::default();
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::<MyEguiApp>::default()));
    //eframe::run_native("My egui App", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));

    for y in 0..max_grid_y {
        for x in 0..max_grid_x {
            row.insert(x as usize, "#".to_string());
        }
        grid.insert(y as usize, row.clone());
        row.clear();
    }

    //println!("cols: {}, rows: {}", cols, rows);
    //print!("{}", renderGrid(&grid, max_grid_x, max_grid_y));
    //print!("{}", renderGrid(&grid, max_grid_x, max_grid_y));
    //renderGrid(&grid, max_grid_x, max_grid_y);
    //let mut stdout = stdout();

    /*for i in 0..=100 {
        print!("\rProcessing {}%...", i);
        // or
        // stdout.write(format!("\rProcessing {}%...", i).as_bytes()).unwrap();

        stdout.flush().unwrap();
        sleep(Duration::from_millis(2));
    }*/
    
    /*loop {
        let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("error");
        let mut command = command.trim();
        //stdout.flush().unwrap();
        if command == "r" {
            clearscreen::clear().expect("failed to clear screen");
            grid = room(
                grid.clone(),
                max_grid_x, 
                max_grid_y,
                is_first
            );
            is_first = false;
            print!("{}", renderGrid(&grid, max_grid_x, max_grid_y));
        } else if command == "e"{
            if water_source == vec![] {
                water_source = findSource(&grid, max_grid_x, max_grid_y);
            }
            //clearscreen::clear().expect("failed to clear screen");
            //let mut stdout = stdout();
            let total_drops = 400; // how much waterdrops gonna be there
            let mut drops_used = 0;
            loop {
                if drops_used == 0 || 
                   grid != moveWater(
                        grid.clone(),
                        max_grid_x, 
                        max_grid_y
                    ) {
                    grid = moveWater(
                        grid.clone(),
                        max_grid_x, 
                        max_grid_y
                    );
                } else {
                    //break;
                }
                if drops_used < total_drops {
                    grid[(water_source[1] + 1) as usize][water_source[0] as usize] = "0".to_string();
                    drops_used = drops_used + 1;
                }
                print!("{}", renderGrid(&grid, max_grid_x, max_grid_y));
                //clearscreen::clear().expect("failed to clear screen");
                //print!("test {}", i);
                //print!("{}", renderGrid(&grid, max_grid_x, max_grid_y));
                /*stdout.write(format!("\r{}{}", "testfdjjjjjjjjjj", i).as_bytes()).unwrap();
                print!("\n");
                stdout.write(format!("\r{}{}", "testfdjjjjjjjjjj", i).as_bytes()).unwrap();*/
                //renderGrid(&grid, max_grid_x, max_grid_y);
                //stdout.flush().unwrap();
                sleep(Duration::from_millis(100));
            }
        }
    }*/
}

fn renderGrid(grid: &Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> String{
    let mut whole_table: String = "".to_string();
    for y in 0..max_grid_y {
        for x in 0..max_grid_x {
            whole_table = whole_table + &grid[y as usize][x  as usize];
            if grid[y as usize][x  as usize] == "".to_string()
            {
                whole_table = whole_table + &"U".to_string();
            }
        }
        whole_table = whole_table + "\n";
    }
    return whole_table;
}

fn moveWater(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> Vec<Vec<String>> {
    for y_normal in 0..max_grid_y { // cycle goes down to up
        let y = max_grid_y - y_normal - 1; // wanna make the cycle go down to up, without -1 it doesnt work.
        for x in 0..max_grid_x {
            if grid[y as usize][x as usize] == "(".to_string() {
                if grid[y as usize][(x - 1) as usize] == "_".to_string() {
                    grid[y as usize][x as usize] = "_".to_string();
                    grid[y as usize][(x - 1) as usize] = "(".to_string();
                }
                if isWall(&grid[y as usize][(x - 1) as usize]) || grid[y as usize][(x - 1) as usize] == "0".to_string() {
                    grid[y as usize][x as usize] = "0".to_string();
                }
            }
        }
        for x_normal in 0..max_grid_x { // cycle goes right to left
            let x = max_grid_x - x_normal - 1; // wanna make the cycle go right to left, without -1 it doesnt work.
            if isWater(&grid[y as usize][x as usize]) && ShouldProcess(&grid.clone(), x, y) { // check if the cell is water.
                if grid[y as usize][x as usize] == ")".to_string() {    // check if the cell is
                    if grid[y as usize][(x + 1) as usize] == "_".to_string() { // moving water, if so, we
                        grid[y as usize][x as usize] = "_".to_string();            // move it further.
                        grid[y as usize][(x + 1) as usize] = ")".to_string();
                    } else if grid[y as usize][(x + 1) as usize] == "0".to_string() {
                        grid[y as usize][x as usize] = "X".to_string();
                    }
                }
                if grid[(y + 1) as usize][x as usize] == "_".to_string() { // if cell below empty, move down.
                    grid[y as usize][x as usize] = "_".to_string();
                    grid[(y + 1) as usize][x as usize] = "0".to_string();
                }
                if isWall(&grid[(y + 1) as usize][x as usize]) || grid[(y + 1) as usize][x as usize] == "0".to_string() { // if cell below is wall.     
                    if isWall(&grid[y as usize][(x - 1) as usize]) || // if wall to the side
                       isWall(&grid[y as usize][(x + 1) as usize]) {
                        grid[y as usize][x as usize] = "0".to_string(); // turn water into "still" water,
                                                                        // "(" and ")" represent "flowing" water.
                    } else {
                        let mut flow_dir = flowDirection(
                            &grid.clone(),
                            max_grid_x, 
                            max_grid_y,
                            x,
                            y
                        );
                        if grid[(y - 1) as usize][x as usize] == "0".to_string()  && // if cell above is still water
                           grid[y as usize][x as usize]      == "0".to_string()   && // or if target cell is still water
                           (grid[y as usize][(x - 1) as usize] != "0".to_string() || // and there's no still water left and right to target cell
                           grid[y as usize][(x + 1) as usize] != "0".to_string()) {
                            grid[y as usize][x as usize] = "_".to_string();          // turns target cell into empty space
                            if flow_dir == 1 {
                                grid[y as usize][(x + 1) as usize] = ")".to_string();
                            }
                            if flow_dir == -1 {
                                grid[y as usize][(x - 1) as usize] = "(".to_string();
                            }
                        }
                    }
                }
                if isRowBelowWater(&grid.clone(), x, y) { // this one checks if water should fill next layer
                    print!("{}", grid[(y - 1) as usize][(x - 1) as usize]);
                    print!("{}", grid[(y - 1) as usize][x as usize]);
                    println!("{}", grid[(y - 1) as usize][(x + 1) as usize]);
                    print!("{}", grid[y as usize][(x - 1) as usize]);
                    print!("{}", grid[y as usize][x as usize]);
                    println!("{}", grid[y as usize][(x + 1) as usize]);
                    print!("{}", grid[(y + 1) as usize][(x - 1) as usize]);
                    print!("{}", grid[(y + 1) as usize][x as usize]);
                    println!("{}", grid[(y + 1) as usize][(x + 1) as usize]);
                    println!("---------------------");
                    /*if (grid[y as usize][x as usize] == "(".to_string() ||
                       grid[y as usize][x as usize] == ")".to_string()) &&
                       (isWall(&grid[y as usize][(x - 1) as usize]) || // if wall to the side
                       isWall(&grid[y as usize][(x + 1) as usize]) ||
                       grid[y as usize][(x - 1) as usize] == "0".to_string() ||
                       grid[y as usize][(x + 1) as usize] == "0".to_string()) {
                        
                        grid[y as usize][x as usize] = "0".to_string(); // turn water into "still" water,
                                                                            // "(" and ")" represent "flowing" water.
                    } else*/ if grid[y as usize][x as usize] == "0".to_string() {
                        let mut flow_dir = flowDirection(
                            &grid.clone(),
                            max_grid_x, 
                            max_grid_y,
                            x,
                            y
                        );
                        grid[y as usize][x as usize] = "_".to_string();
                        if flow_dir == 1 {
                            grid[y as usize][(x + 1) as usize] = ")".to_string(); // and make the water flow randomly
                        }
                        if flow_dir == -1 {
                            grid[y as usize][(x - 1) as usize] = "(".to_string();
                        }
                    }
                }
            }
            if grid[y as usize][x as usize] == "M".to_string() {
                grid[y as usize][x as usize] = "(".to_string();
            }
            if grid[y as usize][x as usize] == "X".to_string() {
                grid[y as usize][x as usize] = "0".to_string();
            }
            if grid[y as usize][x as usize] == "C".to_string() {
                grid[y as usize][x as usize] = "(".to_string();
                grid[y as usize][(x - 1) as usize] = "C".to_string();
            }
        }
    }
    return grid;
}

fn flowDirection(grid: &Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32, target_x: i32, target_y: i32) -> i32 {
   for right in target_x..max_grid_x {
        if grid[(target_y + 1) as usize][right as usize] == "_".to_string() {
            //return 1;
        }
   }
   for left in target_x..0 {
        if grid[(target_y + 1) as usize][left as usize] == "_".to_string() {
            //return -1;
        }
   }
   if grid[target_y as usize][(target_x + 1) as usize] == "_".to_string() &&
      grid[target_y as usize][(target_x - 1) as usize] != "_".to_string() {
        return 1;
   } 
   if grid[target_y as usize][(target_x + 1) as usize] != "_".to_string() {
        if grid[target_y as usize][(target_x - 1) as usize] == "_".to_string() {
            return -1;
        } 
   }
   let mut random_flow = rand::thread_rng().gen_range(0..=1);
   if random_flow == 0 { // && grid[target_y as usize][(target_x - 1) as usize] == "_".to_string() {
        random_flow = -1;
   } else {
        random_flow = 1;
   }
   //if grid[target_y as usize][(target_x + 1) as usize] == "0".to_string() {
   //   random_flow = 0;
   //}
   if grid[target_y as usize][(target_x + 1) as usize] == "0".to_string() &&
      grid[target_y as usize][(target_x - 1) as usize] == "0".to_string() {
        random_flow = 0;
   }
   return random_flow;
}


fn room(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32, is_first: bool) -> Vec<Vec<String>> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut offset_x: i32 = 0;
    let mut offset_y: i32 = 0;
    let mut enough_space: bool = false;
    let mut across_room: bool = false; // there must be a room directly up down left or right
    while !enough_space 
    {
        x        = rand::thread_rng().gen_range(5..max_grid_x - 5);
        y        = rand::thread_rng().gen_range(5..max_grid_y - 5);
        offset_x = rand::thread_rng().gen_range(1..=(max_grid_x - x - 1));
        offset_y = rand::thread_rng().gen_range(1..=(max_grid_y - y - 1));
        enough_space = true;
        for a in -1..x + 1 {
            for b in -1..y + 1 {
                if grid[(b + offset_y) as usize][(a + offset_x) as usize] == "_".to_string() {
                    enough_space = false;
                }
            }
        }
        if !is_first {
            for a in 0..x {
                for b in 0..y {
                    if grid[(b + offset_y) as usize][(a + offset_x) as usize] == "B".to_string() {
                        across_room = true;
                    }
                }
            }
        }
        if !is_first && !across_room {
            enough_space = false;
        }
        println!{"is first: {is_first} enough: {enough_space}, and room {across_room}"};
    }
    let mut a = 0;
    for a in 0..max_grid_x
    {
        let mut b = 0;
        for b in 0..max_grid_y
        {
            if (a > offset_x && a < x + offset_x - 1) || (b > offset_y && b < y + offset_y - 1) {
                if grid[b as usize][a as usize] == "B".to_string()
                {
                    grid[b as usize][a as usize] = "E".to_string();
                } else if grid[b as usize][a as usize] != "_".to_string() && 
                          grid[b as usize][a as usize] != "E".to_string() &&
                          grid[b as usize][a as usize] != "^".to_string()
                {
                    grid[b as usize][a as usize] = "B".to_string();
                }
            }
        }
        //print!("\n");
    }
    let mut door_cells: Vec<Vec<i32>> = vec![vec![]];
    let mut path_vec: Vec<i32> = vec![];
    let mut a = 0;
    for a in 0..x
    {
        let mut b = 0;
        for b in 0..y
        {
            //print!("<{b} + {offset_x}, {a} + {offset_x}: {} => >", grid[(b + offset_y) as usize][(a + offset_x) as usize]);
            grid[(b + offset_y) as usize][(a + offset_x) as usize] = "_".to_string();
            // print!("{}>\n", grid[(b + offset_y) as usize][(a + offset_x) as usize]);
        }
    }
    for a in 0..x
    {
        let mut b = 0;
        for b in 0..y
        {
            path_vec = canConnect(grid.clone(), a + offset_x, b + offset_y);
            if canConnect(grid.clone(), a + offset_x, b + offset_y) != vec![0, 0] {
                let mut i = 1; // used for cycle that checks if there is room above
                loop {
                    if grid[(b + offset_y + path_vec[1] * i) as usize][(a + offset_x + path_vec[0] * i) as usize] != "_".to_string() {
                        door_cells.insert(door_cells.len() - 1, vec![b + offset_y, a + offset_x, path_vec[1], path_vec[0]]);
                    } else {
                        i = 1;
                        break;
                    }
                    i = i + 1;
                }
            }
        }
    }
    if door_cells.len() > 1 {
        let id = rand::thread_rng().gen_range(0..door_cells.len());
        println!("id {id}, y {}, x {}", door_cells[id][0], door_cells[id][1]);
                let mut j = 1;
                loop {
                    if grid[(door_cells[id][0] + door_cells[id][2] * j) as usize][(door_cells[id][1] + door_cells[id][3] * j) as usize] != "_".to_string() {
                        grid[(door_cells[id][0] + door_cells[id][2] * j) as usize][(door_cells[id][1] + door_cells[id][3] * j) as usize] = "_".to_string();
                    } else {
                        j = 1;
                        break;
                    }
                    j = j + 1;
                }
    }
    if is_first {
        let id = rand::thread_rng().gen_range(1..x - 1);
        let mut i = 1;
        loop {
            if offset_y - i != 0 {
                grid[(offset_y - i) as usize][(offset_x + id) as usize] = "_".to_string();
            } else {
                grid[(offset_y - i) as usize][(offset_x + id) as usize] = "^".to_string();
                break;
            }
            i = i + 1;
        }
    }
    return grid;
}

fn canConnect(mut grid: Vec<Vec<String>>, mut x: i32, mut y: i32) -> Vec<i32> {

    let mut path_direction: Vec<i32> = vec![];
    path_direction.insert(0, 0);
    path_direction.insert(1, 0);

    if grid[y as usize][x as usize] == "_".to_string() {
        if (y - 1) > 0 && grid[(y - 1) as usize][x as usize] == "E".to_string() {
            let mut up = y - 1;// used for cycle that checks if there is room above
            loop {
                if grid[up as usize][x as usize] == "_".to_string() {
                    path_direction[1] = -1;
                    break;
                }
                if up < 1 {
                    //println!("index was {up}");
                    break;
                }
                up = up - 1;
                continue;
            }        
        }
        if (x - 1) > 0 && grid[y as usize][(x - 1) as usize] == "E".to_string() {
            let mut left = x - 1;// used for cycle that checks if there is room above
            loop {
                if grid[y as usize][left as usize] == "_".to_string() {
                    if path_direction == vec![0, 0] {
                        path_direction[0] = -1;
                    }
                    break;
                }
                if left < 1 {
                    //println!("index was {up}");
                    break;
                }
                left = left - 1;
                continue;
            }        
        }
        if (x + 1) < 65 && grid[y as usize][(x + 1) as usize] == "E".to_string() {
            let mut right = x + 1;// used for cycle that checks if there is room above
            loop {
                if grid[y as usize][right as usize] == "_".to_string() {
                    if path_direction == vec![0, 0] {
                        path_direction[0] = 1;
                    }
                    break;
                }
                if right > 66 {
                    break;
                }
                right = right + 1;
                continue;
            }
        }
        if (y + 1) < 35 && grid[(y + 1) as usize][x as usize] == "E".to_string() {
            let mut down = y + 1;// used for cycle that checks if there is room above
            loop {
                if grid[down as usize][x as usize] == "_".to_string() {
                    if path_direction == vec![0, 0] {
                        path_direction[1] = 1;
                    }
                    break;
                }
                if down > 36 {
                    break;
                }
                down = down + 1;
                continue;
            }
        }
    }
    return path_direction;
}

fn findSource(mut grid: &Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> Vec<i32> {
    let mut water_source: Vec<i32> = vec![];
    for y in 0..max_grid_y
    {
        for x in 0..max_grid_x
        {
            if grid[y as usize][x as usize] == "^".to_string() {
                water_source = vec![x, y];
            }
        }
    }
    return water_source;

}

fn isWater(value: &String) -> bool {
    if value == &"0".to_string() || value == &"(".to_string() || value == &")".to_string() {
        return true;
    } else {
        return false;
    }
}

fn isWall(value: &String) -> bool {
    if value == &"#".to_string() || value == &"B".to_string() || value == &"E".to_string() {
        return true;
    } else {
        return false;
    }
}

fn isRowBelowWater(grid: &Vec<Vec<String>>, target_x: i32, target_y: i32) -> bool {
    // this function takes a cell, and looks if the layer of water below is full. It indicates if the cell can flow on top of the existing layer.
    // the function also checks for instances when water is technicaly on top of full water layer but e.g. is near a wall or sandwiched between other layers.
    // so it's not just "is the layer below complete?" but also "should the cell flow and form a new layer?"
    let mut isRowFull = true; // return value. It's assumed to be true at first but can changed coz of the loops below
    let mut counter_left = 0;
    let mut counter_right = 0;
                                                                              // this "if" is reversed, so everything that PASSES the check gets false is kicked out.
    if grid[(target_y - 1) as usize][target_x as usize] != "0".to_string() || // if cell above isn't still (only water from the vertical "stream" won't be stopped here),
       grid[target_y as usize][target_x as usize] != "0".to_string() ||       // or if target cell isn't still ( the "(" and ")" are stopped here),
       (isWater(&grid[target_y as usize][(target_x - 1) as usize]) &&         // or if there's water on both sides,
       isWater(&grid[target_y as usize][(target_x + 1) as usize])) ||
       (isWall(&grid[target_y as usize][(target_x - 1) as usize]) ||          // or if there's a wall on any side,
       isWall(&grid[target_y as usize][(target_x + 1) as usize])) {
        isRowFull = false;                                                    // then get a false and get outta here.
    }
    // these two loops check each side below the target cell. 
    // "isRowFull" is already true, so the loops check if it should actually be false
    // if it finds something that isn't water or a wall, it returns false
    loop {
        counter_left += 1;
        if counter_left > 100 {
            break;
        }
        if grid[(target_y + 1) as usize][(target_x - counter_left) as usize] == "0".to_string() {
            continue;
        }
        if isWall(&grid[(target_y + 1) as usize][(target_x - counter_left) as usize]) && counter_left > 1 {
            break;
        }
        isRowFull = false;
    }
    loop {
        counter_right += 1;
        if counter_right > 100 {
            break;
        }
        if grid[(target_y + 1) as usize][(target_x + counter_right) as usize] == "0".to_string() {
            continue;
        }
        if isWall(&grid[(target_y + 1) as usize][(target_x + counter_right) as usize]) && counter_right > 1 {
            break;
        }
        isRowFull = false;
    }
    return isRowFull;
}

fn ShouldProcess(grid: &Vec<Vec<String>>, target_x: i32, target_y: i32) -> bool {
    if (grid[(target_y - 1) as usize][(target_x - 1) as usize] == "0".to_string() ||
    grid[(target_y - 1) as usize][(target_x + 1) as usize] == "0".to_string()) && 
    grid[(target_y - 1) as usize][target_x as usize] == "0".to_string() {
        return false;
    } else {
        return true;
    }
}