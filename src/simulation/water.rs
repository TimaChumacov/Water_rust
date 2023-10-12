use rand::Rng;
use crate::simulation::cell::*;

/// the most important thing, here all the logic of water flow happens
pub fn move_water(mut grid: Vec<Vec<String>>, max_grid_x: usize, max_grid_y: usize) -> Vec<Vec<String>> {
    // 0 - water droplet
    // ( and ) - water flowing left or right
    // W - water that got into place and won't move. Code counts it as a wall in is_wall function so It wouldn't lag. Becuse of this the water can "pile up" if you spam ON/OFF though

    // There's one cycle for y and two for x
    for y_normal in 0..max_grid_y { // y cycle goes down to up
        let y = max_grid_y - y_normal - 1; // without -1 it goes ot of bounds
        for x in 0..max_grid_x { // this x cycle goes left to right and only moves water that flows to the left, "("
            if grid[y][x] == "(".to_string() {
                if grid[y][x - 1] == "_".to_string() { // if empty space to the left, then move
                    grid[y][x] = "_".to_string();
                    grid[y][x - 1] = "(".to_string();
                }
                if is_wall(&grid[y][x - 1]) || grid[y][x - 1] == "0".to_string() { // if a wall or "0" to the left, turn into "W"
                    grid[y][x] = "W".to_string();
                }
            }
        }
        for x_normal in 0..max_grid_x { // this x cycle goes right to left. Not only does it move ")" water, but also makes droplets drop and turns water into "w"
            let x = max_grid_x - x_normal - 1; // without -1 it goes ot of bounds
            if is_water(&grid[y][x]) { // check if the cell is water.
                if grid[y + 1][x] == "_".to_string() { // if cell below empty, move down.
                    grid[y][x] = "_".to_string();
                    grid[y + 1][x] = "0".to_string();
                }
                if grid[y][x] == ")".to_string() {    // check if the cell is ")"
                    if grid[y][x + 1] == "_".to_string() { // if empty space to the right, move
                        grid[y][x] = "_".to_string();
                        grid[y][x + 1] = ")".to_string();
                    }
                    if is_wall(&grid[y][x + 1]) || grid[y][x + 1] == "0".to_string() { // if a wall or "0" to the right, turn into "W"
                        grid[y][x] = "W".to_string();
                    }
                }
                if is_wall(&grid[y + 1][x]) { // if cell below is wall.
                    if is_wall(&grid[y][x - 1]) && // if wall to both sides
                        is_wall(&grid[y][x + 1]) {
                        if grid[y - 1][x] == "0".to_string() || grid[y][x] == "0".to_string() { // if cell above is 0
                            grid[y][x] = "W".to_string(); // turn water into "W"
                        }
                    } else if !(is_wall(&grid[y][x - 1]) && // if walls aren't to both sides
                        is_wall(&grid[y][x + 1])) {
                        if grid[y - 1][x] != "0".to_string() && // if there's no "0" above
                            (is_wall(&grid[y][x - 1]) || // and if there's at least 1 wall to the side (it can't be 2 walls btw, it wouldn;t pass the check above)
                                is_wall(&grid[y][x + 1])) {
                            if grid[y][x] == "0".to_string() && // and if target cell is "0"
                                !(grid[y - 1][x] == "_".to_string() && // and if there's no "_" above that has walls to its sides
                                    grid[y - 1][x - 1] == "#".to_string() &&
                                    grid[y - 1][x + 1] == "#".to_string()) {
                                grid[y][x] = "W".to_string(); // turn into "W"
                            }
                        }

                        // more about it in flowDirection function
                        let flow_dir = flow_direction(&grid.clone(), max_grid_x, x, y);
                        if grid[y][x] == "0".to_string() && // and if target cell is "0"
                            (grid[y][x - 1] != "0".to_string() || // and there's no still water left and right to target cell
                                grid[y][x + 1] != "0".to_string()) {
                            grid[y][x] = "_".to_string();          // turns target cell into "_"
                            if flow_dir == 1 { // more about it in flowDirection function
                                grid[y][x + 1] = ")".to_string();
                            }
                            if flow_dir == -1 {
                                grid[y][x - 1] = "(".to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    return grid;
}

fn flow_direction(grid: &Vec<Vec<String>>, max_grid_x: usize, target_x: usize, target_y: usize) -> i32 {
    // the function return a direction for water to flow, right is 1 and left is -1
    // option 1: if empty space to both sides of the flow point and the surface is flat, choose randomly
    // option 2: if empty space to both sides of the flow point and the surface is uneven, choose the direction where the water would be able to fall deeper
    // option 3: if only one side is empty, then flow to that side

    if check_side(grid, max_grid_x, target_x, target_y) != 0 { // option 2. more about it in checkSide function. If it returns zero, then code goes to options 1 and 2
        return check_side(grid, max_grid_x, target_x, target_y);
    } else {
        if grid[target_y][target_x + 1] == "_".to_string() && // checks for option 3
            grid[target_y][target_x - 1] != "_".to_string() {
            return 1;
        }
        if grid[target_y][target_x + 1] != "_".to_string() { // checks for option 3
            if grid[target_y][target_x - 1] == "_".to_string() {
                return -1;
            }
        }
        let mut random_flow = rand::thread_rng().gen_range(0..=1); // option 1
        if random_flow == 0 {
            random_flow = -1;
        } else {
            random_flow = 1;
        }
        if grid[target_y][target_x + 1] == "0".to_string() && // pretty much just for debugging, this never happens
            grid[target_y][target_x - 1] == "0".to_string() {
            random_flow = 0;
        }
        return random_flow;
    }
}

fn check_side(grid: &Vec<Vec<String>>, max_grid_x: usize, target_x: usize, target_y: usize) -> i32 {
    // the function checks if there's a way to flow somewhere deeper on either side.
    // returns: -1 = left; 1 = right; 0 = neither, the surface is flat or walls are really close;
    let mut dir = 0;
    let mut left = 0;
    let mut right = 0;
    loop {
        // each cycle it checks a cell further left. If it's a wall, then nowhere to flow to. If not a wall, check the cell below from that point, if it's "_", then water can flow there.
        left += 1;
        if is_wall(&grid[target_y][target_x - left]) {
            break;
        }
        if grid[target_y + 1][target_x - left] == "_".to_string() {
            dir = -1;
        }
    }
    loop {
        // works same as previous loop but goes right
        right += 1;
        if target_x + left > max_grid_x {
            break;
        }
        if is_wall(&grid[target_y][target_x + right]) {
            break;
        }
        if grid[target_y + 1][target_x + right] == "_".to_string() {
            dir = 1;
        }
    }
    return dir;
}