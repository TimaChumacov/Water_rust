#![windows_subsystem = "windows"]
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
    walls_rendered: bool, // true after pressing "water ON/OFF" and indicates that walls are now redrawn
    water_source: Vec<i32>, // coordinates of the point the water flows from. It looks like "^"
    grid_str: String, // string containing the entire grid.
    button_text: String, // "generate layout" dynamic text string
    can_generate: bool, // is "generate layout" button working
    room_count: i32, // amount of rooms. used to tune room generation.
    frames_in_tick: i32, // water gotta move slowly so I need ticks
    frames_passed: i32,
}

impl MyEguiApp {
    fn default() -> Self {
        Self {
            max_grid_x: 0,      // same variables as above but now they're set a default value
            max_grid_y: 0,      // default values somehow don't work in my case (code acts as if they don't have default),
            grid: vec![vec![]], // so I have to redifine everything again later and meanwhile set some nonsense here
            row: vec![],        
            is_first: true, 
            is_water_on: false, 
            walls_rendered: false,
            water_source: vec![], 
            grid_str: "".to_string(),
            button_text: "".to_string(), 
            can_generate: false,   
            room_count: 0,
            frames_in_tick: 0,
            frames_passed: 0,
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) { // update function fires each frame
        self.max_grid_x = 70; // now some of the variables from above are set a real value
        self.max_grid_y = 45; 
        self.frames_in_tick = 5;

        if self.grid.len() == 0 { // fills the empty grid with "#"
        for y in 0..self.max_grid_y {
            for x in 0..self.max_grid_x {
                println!("y {y} x {x},"); // some debugging output
                self.row.insert(x as usize, "#".to_string());
            }
            self.grid.insert(y as usize, self.row.clone());
            self.row.clear();
        }
        self.grid_str = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y); // empty grid_str gets its value too
        }
        if !self.can_generate && self.is_first {
            self.can_generate = true;
            self.button_text = "generate layout".to_string();
        }
        if !self.can_generate { // makes sure that water flows only when water is ON
            self.button_text = "can't generate".to_string();
        }
        self.grid_str = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y); 
        self.is_first = checkIfFirst( // default values don't work so each frame a value for is_first is generated
            self.grid.clone(),        
            self.max_grid_x, 
            self.max_grid_y
        );
        if self.is_first {
            self.room_count = 0;
        }

            if self.frames_passed >= self.frames_in_tick { // tick counter
                self.grid = moveWater( // water moves each frame
                    self.grid.clone(),
                    self.max_grid_x, 
                    self.max_grid_y
                );
                if self.is_water_on { // makes sure that water flows only when water is ON
                    self.grid[(self.water_source[1] + 1) as usize][self.water_source[0] as usize] = "0".to_string();
                }
                self.grid_str = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y); // grid_str gets a new value each tick too
                self.frames_passed = 0;
            } else {
                self.frames_passed = self.frames_passed + 1;
            }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new(format!("{}", self.grid_str)).monospace());
            ui.horizontal(|ui| { // invisible container that aligns every element horizontally
                if ui.button(egui::RichText::new(format!("{}", self.button_text)).monospace()).clicked() && self.can_generate { // generate layout button.
                                                                                        // works only if the water is off and enough space for a new room
                    loop { // because of a bug I made it a loop so If I catch it re-generates the room
                        let new_grid: Vec<Vec<String>> = room(                                                               
                            self.grid.clone(),
                            self.max_grid_x, 
                            self.max_grid_y,
                            self.is_first,
                            self.room_count
                        );
                        if new_grid[0][0] == "F".to_string() { // if there's "F" at (0; 0) then that's the mark for that bug and I re-generate
                            continue;
                        }
                        if self.grid == new_grid { // if they're the same then there's no space to generate a room 
                            self.can_generate = false;
                            break;
                        } else { // if the new_grid is different (has a room) then the main grid variable steals the value
                            self.grid = new_grid;
                            break;
                        }
                    }
                    //print!("{}", renderGrid(&self.grid, self.max_grid_x, self.max_grid_y)); //debugging
                    self.grid_str = renderGrid(&self.grid, self.max_grid_x, self.max_grid_y); // updated string
                    self.room_count += 1;
                    println!("ROOOM {}", self.room_count);
                }
                if ui.button("water ON/OFF").clicked() && !self.is_first { // second button. works only if at least 1 room is generated
                    self.water_source = findSource(&self.grid, self.max_grid_x, self.max_grid_y); // now the point of waterflow is saved
                    if !self.walls_rendered { // I want walls to be rerendered into the new version only once
                        self.grid = RenderWall(self.grid.clone(), self.max_grid_x, self.max_grid_y);
                        self.walls_rendered = true;
                    }
                    self.is_water_on = !self.is_water_on; // toggling the bool responsible for waterflow
                    self.can_generate = false;
                }
                if ui.button("reset layout").clicked() {// third button
                    self.is_water_on = false;           // resets everything
                    self.walls_rendered = false;
                    self.can_generate = false;
                    self.grid.clear();            // clears the grid
                    for y in 0..self.max_grid_y { // then fills it up with "#"
                        for x in 0..self.max_grid_x {
                            println!("y {y} x {x},");
                            self.row.insert(x as usize, "#".to_string());
                        }
                        self.grid.insert(y as usize, self.row.clone());
                        self.row.clear();
                    }
                }
                ui.label(egui::RichText::new("👇 expand the window! 👇").monospace()); // text label
            });
            // I'm really sorry but the string has to be that long. If I make newline in the editor it will also be in the string and mess up everything
            ui.label("\nHi! This rust apps lets you generate interconnected rooms and then see how they are filled up with water. \"generate layout\" button can be pressed multiple times and it generates a room each time. If there's little space left, the button will be disabled. \"water ON/OFF\" button makes the layout prettier and starts the waterflow. You can press again to to stop the water. You can't generate more rooms after pouring water. \"reset layout\" button resets everything.");
        });
        ctx.request_repaint(); // asks the window to update itself even if the user doesn't move the cursor, very important
   }
}

fn checkIfFirst(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> bool { //returns value for self.is_first
    for x in 0..max_grid_x {     // if there are no rooms, then the grid only has "#", so next room generated will be first
        for y in 0..max_grid_y { // if there are already some rooms, then the loop will find other symbols and know that the room won't be first
            if grid[y as usize][x as usize] != "#".to_string() { // best thing is that the "^" is always at the top so the loop doesn't check for too long
                return false;
            }
        }
    }
    return true;
}

fn main() { // Here there's mostly only egui stuff which is needed for the window rendering
    let native_options = eframe::NativeOptions::default();

    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from( // size of the window
          Vec2::new(530 as f32, 680 as f32)
        ),
        min_window_size: Option::from(
            Vec2::new(530 as f32, 680 as f32)
          ),
        max_window_size: Option::from(
            Vec2::new(530 as f32, 820 as f32) // max size bigger vertically so you could see the description
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
    eframe::run_native("Water rooms", options, Box::new(|_cc| Box::<MyEguiApp>::default()));
}

fn renderGrid(grid: &Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> String{ // turns the grid into string for grid_str
    let mut whole_table: String = "".to_string();
    for y in 0..max_grid_y {
        for x in 0..max_grid_x {
            whole_table = whole_table + &grid[y as usize][x  as usize]; // just concatenates everything in one string
            if grid[y as usize][x  as usize] == "".to_string() // this is only for debugging. If there's an "U", then an empty string was in the grid
            {
                whole_table = whole_table + &"U".to_string();
            }
        }
        whole_table = whole_table + "\n"; // newline each time y gets bigger
    }
    return whole_table;
}

fn RenderWall(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> Vec<Vec<String>> { //returns a new grid where only the outer walls are rendered
    for y in 0..max_grid_y {
        for x in 0..max_grid_x {
            if grid[y as usize][x as usize] == "_" { // all "_" stay "_"
                grid[y as usize][x as usize] = "_".to_string();
            } else if grid[y as usize][x as usize] == "^" { // the "^" stays "^"
                grid[y as usize][x as usize] = "^".to_string();
            } else if y > 1 && x > 1 &&
                      grid[(y - 1) as usize][(x - 1) as usize] == "_".to_string() {// only the walls that have a "_" around get rendered
                grid[y as usize][x as usize] = "#".to_string();                    // it checks 8 cells around the target cell
            } else if y > 1 &&
                      grid[(y - 1) as usize][x as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else if y > 1 && x < (max_grid_x - 1) &&
                      grid[(y - 1) as usize][(x + 1) as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else if x > 1 &&
                      grid[y as usize][(x - 1) as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else if x < (max_grid_x - 1) &&
                      grid[y as usize][(x + 1) as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else if y < (max_grid_y - 1) && x > 1 &&
                      grid[(y + 1) as usize][(x - 1) as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else if y < (max_grid_y - 1) &&
                      grid[(y + 1) as usize][x as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else if y < (max_grid_y - 1) && x < (max_grid_x - 1) &&
                      grid[(y + 1) as usize][(x + 1) as usize] == "_".to_string() {
                grid[y as usize][x as usize] = "#".to_string();
            } else { // if a wall is not around "_" then it's replaced with empty " "
                grid[y as usize][x as usize] = " ".to_string();
            }
        }
    }
    return grid;
}

fn moveWater(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> Vec<Vec<String>> { // the most important thing, here all the logic of waterflow happens
    // 0 - water droplet
    // ( and ) - water flowing left or right
    // W - water that got into place and won't move. Code counts it as a Wall in isWall function so It wouldn't lag. Becuse of this the water can "pile up" if you spam ON/OFF though

    // There's one cycle for y and two for x
    for y_normal in 0..max_grid_y { // y cycle goes down to up
        let y = max_grid_y - y_normal - 1; // without -1 it goes ot of bounds
        for x in 0..max_grid_x { // this x cycle goes left to right and only moves water that flows to the left, "("
            if grid[y as usize][x as usize] == "(".to_string() {
                if grid[y as usize][(x - 1) as usize] == "_".to_string() { // if empty space to the left, then move
                    grid[y as usize][x as usize] = "_".to_string();
                    grid[y as usize][(x - 1) as usize] = "(".to_string();
                }
                if isWall(&grid[y as usize][(x - 1) as usize]) || grid[y as usize][(x - 1) as usize] == "0".to_string() { // if a wall or "0" to the left, turn into "W"
                    grid[y as usize][x as usize] = "W".to_string();
                }
            }
        }
        for x_normal in 0..max_grid_x { // this x cycle goes right to left. Not only does it move ")" water, but also makes droplets drop and turns water into "w"
            let x = max_grid_x - x_normal - 1; // without -1 it goes ot of bounds
            if isWater(&grid[y as usize][x as usize]) { // check if the cell is water.
                if grid[(y + 1) as usize][x as usize] == "_".to_string() { // if cell below empty, move down.
                    grid[y as usize][x as usize] = "_".to_string();
                    grid[(y + 1) as usize][x as usize] = "0".to_string();
                }
                if grid[y as usize][x as usize] == ")".to_string() {    // check if the cell is ")"
                    if grid[y as usize][(x + 1) as usize] == "_".to_string() { // if empty space to the right, move
                        grid[y as usize][x as usize] = "_".to_string();          
                        grid[y as usize][(x + 1) as usize] = ")".to_string();
                    } 
                    if isWall(&grid[y as usize][(x + 1) as usize]) || grid[y as usize][(x + 1) as usize] == "0".to_string() { // if a wall or "0" to the right, turn into "W"
                        grid[y as usize][x as usize] = "W".to_string();
                    }
                }
                if isWall(&grid[(y + 1) as usize][x as usize]) { // if cell below is wall.     
                    if isWall(&grid[y as usize][(x - 1) as usize]) && // if wall to both sides
                       isWall(&grid[y as usize][(x + 1) as usize]) {
                        if grid[(y - 1) as usize][x as usize] == "0".to_string() || grid[y as usize][x as usize] == "0".to_string() { // if cell above is 0
                            grid[y as usize][x as usize] = "W".to_string(); // turn water into "W"
                        }
                    } else if !(isWall(&grid[y as usize][(x - 1) as usize]) && // if walls aren't to both sides
                                isWall(&grid[y as usize][(x + 1) as usize])) {
                        if grid[(y - 1) as usize][x as usize] != "0".to_string() && // if there's no "0" above
                           (isWall(&grid[y as usize][(x - 1) as usize]) || // and if there's at least 1 wall to the side (it can't be 2 walls btw, it wouldn;t pass the check above)
                           isWall(&grid[y as usize][(x + 1) as usize])) {
                            if grid[y as usize][x as usize] == "0".to_string() && // and if target cell is "0"
                             !(grid[(y - 1) as usize][x as usize] == "_".to_string() && // and if there's no "_" above that has walls to its sides
                               grid[(y - 1) as usize][(x - 1) as usize] == "#".to_string() &&
                               grid[(y - 1) as usize][(x + 1) as usize] == "#".to_string()) {
                                grid[y as usize][x as usize] = "W".to_string(); // turn into "W"
                            } 
                        }
                    
                        let mut flow_dir = flowDirection( // more about it in flowDirection function
                            &grid.clone(),
                            max_grid_x, 
                            max_grid_y,
                            x,
                            y
                        );
                        if grid[y as usize][x as usize]      == "0".to_string()   && // and if target cell is "0"
                           (grid[y as usize][(x - 1) as usize] != "0".to_string() || // and there's no still water left and right to target cell
                           grid[y as usize][(x + 1) as usize] != "0".to_string()) {
                            grid[y as usize][x as usize] = "_".to_string();          // turns target cell into "_"
                            if flow_dir == 1 { // more about it in flowDirection function
                                grid[y as usize][(x + 1) as usize] = ")".to_string();
                            }
                            if flow_dir == -1 {
                                grid[y as usize][(x - 1) as usize] = "(".to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    return grid;
}

fn flowDirection(grid: &Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32, target_x: i32, target_y: i32) -> i32 {
    // the function return a direction for water to flow, right is 1 and left is -1
    // option 1: if empty space to both sides of the flow point and the surface is flat, choose randomly
    // option 2: if empty space to both sides of the flow point and the surface is uneven, choose the direction where the water would be able to fall deeper
    // option 3: if only one side is empty, then flow to that side
    
    if checkSide(grid, max_grid_x, target_x, target_y) != 0 { // option 2. more about it in checkSide function. If it returns zero, then code goes to options 1 and 2
        return checkSide(grid, max_grid_x, target_x, target_y); 
    } else {
        if grid[target_y as usize][(target_x + 1) as usize] == "_".to_string() && // checks for option 3
            grid[target_y as usize][(target_x - 1) as usize] != "_".to_string() {
                return 1;
        } 
        if grid[target_y as usize][(target_x + 1) as usize] != "_".to_string() { // checks for option 3
                if grid[target_y as usize][(target_x - 1) as usize] == "_".to_string() {
                    return -1;
                } 
        }
        let mut random_flow = rand::thread_rng().gen_range(0..=1); // option 1
        if random_flow == 0 {
            random_flow = -1;
        } else {
            random_flow = 1;
        }
        if grid[target_y as usize][(target_x + 1) as usize] == "0".to_string() && // pretty much just for debugging, this never happens
           grid[target_y as usize][(target_x - 1) as usize] == "0".to_string() {
            random_flow = 0;
        }
        return random_flow;
    }
}

fn checkSide(grid: &Vec<Vec<String>>, max_grid_x: i32, target_x: i32, target_y: i32) -> i32 {
    // the function checks if there's a way to flow somewhere deeper on either side.
    // returns: -1 = left; 1 = right; 0 = neither, the surface is flat or walls are really close;
    let mut dir = 0;
    let mut left = 0;
    let mut right = 0;
    loop { 
        // each cycle it checks a cell further left. If it's a wall, then nowhere to flow to. If not a wall, check the cell below from that point, if it's "_", then water can flow there.
        left += 1;
        if target_x - left < 0 {
            break;
        }
        if isWall(&grid[target_y as usize][(target_x - left) as usize]) {
            break;
        }
        if grid[(target_y + 1) as usize][(target_x - left) as usize] == "_".to_string() {
            dir = -1;
        }
    }
    loop {
        // works same as previous loop but goes right
        right += 1;
        if target_x + left > max_grid_x {
            break;
        }
        if isWall(&grid[target_y as usize][(target_x + right) as usize]) {
            break;
        }
        if grid[(target_y + 1) as usize][(target_x + right) as usize] == "_".to_string() {
            dir = 1;
        }
    }
    return dir;
}


fn room(mut grid: Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32, is_first: bool, room_count: i32) -> Vec<Vec<String>> {
    let mut x: i32 = 0; // size of the room
    let mut y: i32 = 0;
    let mut offset_x: i32 = 0; // offset of the room from the left top corner
    let mut offset_y: i32 = 0;
    let mut enough_space: bool = false; // does the room fit?
    let mut across_room: bool = false; // there must be a room directly up, down, left or right
    let mut attempts: i32 = 0; // how many attempts to generate. if over 1500, then can't generate.
    loop  // keeps generating rooms until all conditions are met
    {
        if 5 < (max_grid_x - 5 - room_count * 2) && 5 < max_grid_y - 5 - room_count - room_count * 4 && 
               (1 + room_count * 5) <= (max_grid_y - y - 1 - (20 - room_count * 5)) && 0 <= (20 - room_count * 4) {
            x        = rand::thread_rng().gen_range(5..max_grid_x - 5 - room_count * 2); // random size, min 5 and max pretty much all the width/height
            y        = rand::thread_rng().gen_range(5..max_grid_y - 5 - room_count * 5 - (20 - room_count * 4));
            offset_x = rand::thread_rng().gen_range(1..=(max_grid_x - x - 1)); // random offset, min 1 and max so that the room still fits into grid
            offset_y = rand::thread_rng().gen_range((2 + room_count * 5)..=(max_grid_y - y - 1 - (20 - room_count * 4)));
            enough_space = true;
            for a in -1..x + 1 { //cycle that goes through all cells in the room plus 1 cell outline
                for b in -1..y + 1 {
                    if grid[(b + offset_y) as usize][(a + offset_x) as usize] == "_".to_string() {
                        enough_space = false; // if in the newly randomly generated room there's a "_" (an intersection with another room), then there's not enough space
                    }
                }
            }
            across_room = false;
            if !is_first { // first room doesn't have to make tunnels to other rooms
                for a in 0..x { //cycle that goes through all cells in the room
                    for b in 0..y {
                        if grid[(b + offset_y) as usize][(a + offset_x) as usize] == "B".to_string() { // if there's a "B" and not a "#", then there's a room across
                            across_room = true;
                        }
                    }
                }
            }
            if enough_space && across_room {
                break;
            }
            if is_first {
                break;
            }
            /*if !is_first && !across_room { // little workaround: even if there's enough space, but no room across, we'll say that there enough_space = false so the while cycle would keep going
                enough_space = false;
            }*/
        }
        attempts += 1;
        if attempts > 5000 { // don't want it to be endless so after 1000 attempts it's over and returns the same grid
            return grid;
        }
    }
    let mut a = 0;
    for a in 0..max_grid_x //cycle that goes through all cells in the grid
    {
        let mut b = 0;
        for b in 0..max_grid_y
        {
            if (a >= offset_x && a < x + offset_x) || (b >= offset_y && b < y + offset_y) {
                if grid[b as usize][a as usize] == "B".to_string()  // turs all "B" that are horizontal and vertical to the room into "E".
                {                                                   // This'll be a sign for this room that it's across some other room                                        
                    grid[b as usize][a as usize] = "E".to_string();
                } else if grid[b as usize][a as usize] == "#".to_string()
                {
                    grid[b as usize][a as usize] = "B".to_string(); // turs all "#" that are horizontal and vertical to the room into "B".
                }                                                   // This'll be a sign for other rooms that there's a room across somewhere
            }
        }
    }
    let mut door_cells: Vec<Vec<i32>> = vec![vec![]]; // cells that can make a tunnel to a room across. Later only one of them will be chosen.
                                                      // each cell stored has an id and 4 value stored: cells x and y, and the x and y of 2d Vector pointing the direction of the tunnel
    let mut path_vec: Vec<i32> = vec![]; // the direction in which the tunnel should go
    let mut a = 0;
    for a in 0..x
    {
        let mut b = 0;
        for b in 0..y
        {
            grid[(b + offset_y) as usize][(a + offset_x) as usize] = "_".to_string(); // fills the new room with "_"
        }
    }
    if is_first { // this one makes the upward tunnel for the first room and creates "^"
        let id = rand::thread_rng().gen_range(1..x - 1); // the position of the tunnel is also random
        let mut i = 1;
        loop {
            if offset_y - i != 0 {
                grid[(offset_y - i) as usize][(offset_x + id) as usize] = "_".to_string();
            } else {
                grid[(offset_y - i) as usize][(offset_x + id) as usize] = "^".to_string();
                return grid;
            }
            i = i + 1;
        }
    }
    for a in 0..x
    {
        let mut b = 0;
        for b in 0..y
        {
            path_vec = canConnect(grid.clone(), a + offset_x, b + offset_y); // each cell is checked if it could a tunnel. more about in canConnect function
            if canConnect(grid.clone(), a + offset_x, b + offset_y) != vec![0, 0] { // If the cell can make a tunnel, it's added to door_cells
                let mut i = 1;
                loop {
                    if grid[(b + offset_y + path_vec[1] * i) as usize][(a + offset_x + path_vec[0] * i) as usize] != "_".to_string() { // also checks if there's no other tunnel/room in the way
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
    if door_cells.len() > 1 { // randomly choses a cell that will make the tunnel
        'outer: loop {
            let id = rand::thread_rng().gen_range(0..door_cells.len());
            let mut j = 1;
            'inner: loop { 
                if door_cells[id].len() < 1 { // there's a bug that makes door_cells empty and crashes the app, so I gotta check if door_cells is empty or not
                    println!("caught it!");
                    grid[0][0] = "F".to_string(); // F at (0; 0) is a sign that the generation was bugged and gotta be remade.
                    return grid;
                }
                if door_cells[id][0] + door_cells[id][2] * j >= max_grid_y && door_cells[id][1] + door_cells[id][3] * j >= max_grid_x { // just to be sure it won't go out of bounds
                    continue 'inner;
                }
                if grid[(door_cells[id][0] + door_cells[id][2] * j) as usize][(door_cells[id][1] + door_cells[id][3] * j) as usize] != "_".to_string() {
                    // draws the tunnel until it hits "_" of another room
                    grid[(door_cells[id][0] + door_cells[id][2] * j) as usize][(door_cells[id][1] + door_cells[id][3] * j) as usize] = "_".to_string();
                } else {
                    j = 1;
                    break 'outer;
                }
                j = j + 1;
            }
        }
    }
    return grid;
}

fn canConnect(mut grid: Vec<Vec<String>>, mut x: i32, mut y: i32) -> Vec<i32> {
    // returns a 2d Vector that points in which direction to build the tunnel
    let mut path_direction: Vec<i32> = vec![];
    path_direction.insert(0, 0);
    path_direction.insert(1, 0);

    // checks every direction, if there's "E", return thet direction
    if grid[y as usize][x as usize] == "_".to_string() {
        if (y - 1) > 0 && grid[(y - 1) as usize][x as usize] == "E".to_string() {
            let mut up = y - 1;
            loop {
                if grid[up as usize][x as usize] == "_".to_string() {
                    path_direction[1] = -1;
                    break;
                }
                if up < 1 {
                    break;
                }
                up = up - 1;
                continue;
            }        
        }
        if (x - 1) > 0 && grid[y as usize][(x - 1) as usize] == "E".to_string() {
            let mut left = x - 1;
            loop {
                if grid[y as usize][left as usize] == "_".to_string() {
                    if path_direction == vec![0, 0] {
                        path_direction[0] = -1;
                    }
                    break;
                }
                if left < 1 {
                    break;
                }
                left = left - 1;
                continue;
            }        
        }
        if (x + 1) < 65 && grid[y as usize][(x + 1) as usize] == "E".to_string() {
            let mut right = x + 1;
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
    }
    return path_direction;
}

fn findSource(mut grid: &Vec<Vec<String>>, max_grid_x: i32, max_grid_y: i32) -> Vec<i32> { // just gives coordinates of "^"
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

fn isWater(value: &String) -> bool { // checks if the cell is water. The reason why "W" is explained in isWall function
    if value == &"0".to_string() || value == &"(".to_string() || value == &")".to_string() {
        return true;
    } else {
        return false;
    }
}

fn isWall(value: &String) -> bool {  // checks if the cell is water. "W" is here because it doesn't have to move, and is pretty much a wall once placed. It would lag if "W" was water
    if value == &"#".to_string() || value == &"B".to_string() || value == &"E".to_string() || value == &"W".to_string() {
        return true;
    } else {
        return false;
    }
}

// KNOWN BUGS:
// - rooms can generate diagonally realy closely but without a path to each other
// - by spamming ON/OFF water can sometimes pile up like sand. Happens only in very specific corners.