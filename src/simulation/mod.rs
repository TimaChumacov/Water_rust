use crate::simulation::room::room;
use crate::simulation::water::move_water;

mod water;
mod room;
mod cell;

pub struct SimulationState {
    pub grid_str: String, // string containing the entire grid.
    pub can_generate: bool, // is "generate layout" button working
    pub room_count: usize,
    grid: Vec<Vec<String>>, // grid itself. 2-dimensional vector
    row: Vec<String>, // useful when we define grid size
    is_water_on: bool, // should the water flow. triggered with start button
    walls_rendered: bool, // true after pressing "water ON/OFF" and indicates that walls are now redrawn
    water_source: Vec<usize>, // coordinates of the point the water flows from. It looks like "^"
    frames_passed: i32,
}

impl Default for SimulationState {
    fn default() -> Self {
        let mut new_self = Self {
            grid_str: "".to_string(),
            can_generate: true,
            room_count: 0,
            grid: vec![],
            row: vec![],
            is_water_on: false,
            walls_rendered: false,
            water_source: vec![],
            frames_passed: 0,
        };

        // fill with "#"
        for y in 0..MAX_GRID_Y {
            for x in 0..MAX_GRID_X {
                new_self.row.insert(x, "#".to_string());
            }
            new_self.grid.insert(y, new_self.row.clone());
            new_self.row.clear();
        }

        new_self
    }
}

static MAX_GRID_X: usize = 70;
static MAX_GRID_Y: usize = 45;
static FRAMES_IN_TICK: i32 = 5;  // water gotta move slowly so I need ticks

pub fn on_simulation_update(state: &mut SimulationState) {
    if state.grid.len() == 0 { // fills the empty grid with "#"
        for y in 0..MAX_GRID_Y {
            for x in 0..MAX_GRID_X {
                println!("y {y} x {x},"); // some debugging output
                state.row.insert(x, "#".to_string());
            }
            state.grid.insert(y, state.row.clone());
            state.row.clear();
        }
        state.grid_str = render_grid(&state.grid); // empty grid_str gets its value too
    }
    state.grid_str = render_grid(&state.grid);

    if state.frames_passed >= FRAMES_IN_TICK {
        state.grid = move_water(state.grid.clone(), MAX_GRID_X, MAX_GRID_Y); // water moves each frame
        if state.is_water_on { // makes sure that water flows only when water is ON
            state.grid[state.water_source[1] + 1][state.water_source[0]] = "0".to_string();
        }
        state.grid_str = render_grid(&state.grid); // grid_str gets a new value each tick too
        state.frames_passed = 0;
    } else {
        state.frames_passed = state.frames_passed + 1;
    }
}

pub fn on_generate_layout(state: &mut SimulationState) {
    // works only if the water is off and enough space for a new room
    loop { // because of a bug I made it a loop so If I catch it re-generates the room
        let new_grid: Vec<Vec<String>> = room(
            state.grid.clone(),
            MAX_GRID_X,
            MAX_GRID_Y,
            state.room_count,
        );
        if new_grid[0][0] == "F".to_string() { // if there's "F" at (0; 0) then that's the mark for that bug and I re-generate
            continue;
        }
        if state.grid == new_grid { // if they're the same then there's no space to generate a room
            state.can_generate = false;
            break;
        } else { // if the new_grid is different (has a room) then the main grid variable steals the value
            state.grid = new_grid;
            break;
        }
    }
    //print!("{}", renderGrid(&self.grid, self.max_grid_x, self.max_grid_y)); //debugging
    state.grid_str = render_grid(&state.grid); // updated string
    state.room_count += 1;
    println!("ROOOM {}", state.room_count);
}

pub fn on_water_onoff(state: &mut SimulationState) {
    if state.room_count == 0 {
        return;
    }

    state.water_source = find_source(&state.grid); // now the point of waterflow is saved
    if !state.walls_rendered { // I want walls to be rerendered into the new version only once
        state.grid = render_wall(state.grid.clone(), MAX_GRID_X, MAX_GRID_Y);
        state.walls_rendered = true;
    }
    state.is_water_on = !state.is_water_on; // toggling the bool responsible for waterflow
    state.can_generate = false;
}

/// returns a new grid where only the outer walls are rendered
fn render_wall(mut grid: Vec<Vec<String>>, max_grid_x: usize, max_grid_y: usize) -> Vec<Vec<String>> {
    for y in 0..max_grid_y {
        for x in 0..max_grid_x {
            if grid[y][x] == "_" { // all "_" stay "_"
                grid[y][x] = "_".to_string();
            } else if grid[y][x] == "^" { // the "^" stays "^"
                grid[y][x] = "^".to_string();
            } else if y > 1 && x > 1 &&
                grid[y - 1][x - 1] == "_".to_string() {// only the walls that have a "_" around get rendered
                grid[y][x] = "#".to_string();                    // it checks 8 cells around the target cell
            } else if y > 1 &&
                grid[y - 1][x] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else if y > 1 && x < (max_grid_x - 1) &&
                grid[y - 1][x + 1] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else if x > 1 &&
                grid[y][x - 1] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else if x < (max_grid_x - 1) &&
                grid[y][x + 1] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else if y < (max_grid_y - 1) && x > 1 &&
                grid[y + 1][x - 1] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else if y < (max_grid_y - 1) &&
                grid[y + 1][x] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else if y < (max_grid_y - 1) && x < (max_grid_x - 1) &&
                grid[y + 1][x + 1] == "_".to_string() {
                grid[y][x] = "#".to_string();
            } else { // if a wall is not around "_" then it's replaced with empty " "
                grid[y][x] = " ".to_string();
            }
        }
    }
    return grid;
}

/// turns the grid into string for grid_str
fn render_grid(grid: &Vec<Vec<String>>) -> String {
    let mut whole_table: String = "".to_string();
    for y in 0..MAX_GRID_Y {
        for x in 0..MAX_GRID_X {
            whole_table = whole_table + &grid[y][x]; // just concatenates everything in one string
            if grid[y][x] == "".to_string() // this is only for debugging. If there's an "U", then an empty string was in the grid
            {
                whole_table = whole_table + &"U".to_string();
            }
        }
        whole_table = whole_table + "\n"; // newline each time y gets bigger
    }
    return whole_table;
}

/// just gives coordinates of "^"
fn find_source(grid: &Vec<Vec<String>>) -> Vec<usize> {
    let mut water_source: Vec<usize> = vec![];
    for y in 0..MAX_GRID_Y
    {
        for x in 0..MAX_GRID_X
        {
            if grid[y][x] == "^".to_string() {
                water_source = vec![x, y];
            }
        }
    }
    return water_source;
}