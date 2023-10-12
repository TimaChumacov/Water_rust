use rand::Rng;


struct DoorCell {
    pub y: usize,
    pub x: usize,
    pub direction_y: i32,
    pub direction_x: i32,
}


struct ByRoomAdjusting {
    pub min_y_size: usize,
    pub min_x_size: usize,  
    pub max_y_size: usize,
    pub max_x_size: usize,
    pub min_y_offset: usize,
    pub min_x_offset: usize,
    pub max_y_offset: usize,
    pub max_x_offset: usize,
}


pub fn room(mut grid: Vec<Vec<String>>, max_grid_x: usize, max_grid_y: usize, room_count: usize) -> Vec<Vec<String>> {
    let is_first = room_count == 0;
    let mut x: usize; // size of the room
    let mut y: usize;
    let mut offset_x: usize; // offset of the room from the left top corner
    let mut offset_y: usize;
    let mut enough_space: bool; // does the room fit?
    let mut attempts: i32 = 0; // how many attempts to generate. if over 1500, then can't generate.
    loop  // keeps generating rooms until all conditions are met
    {
        // TODO: These conditions are buggy, they need to be replaced // Sviat
        // let cond1 = (max_grid_x - 5 - room_count * 2) > 5;
        // let cond2 = max_grid_y - 5 - room_count - room_count * 4 > 5;
        // let cond3 = (1 + room_count * 5) <= (max_grid_y - y - 1 - (20 - room_count * 5));


        /// I want rooms to generate up to down, since the app shines best when there's lot's of space to flow beneath
        /// when room_count is low => min_offset is minimal, but max_offset is much reduced (so room ends up at top the top)
        /// when room_count is higher => min_offset gets bigger, but max_offset is unreduced (so room ends up at the bottom)
        if room_count <= 5 {
            let mut new_room = ByRoomAdjusting {
                min_y_size: 5,  
                min_x_size: 5,
                max_x_size: max_grid_x - 5 - room_count * 2,
                max_y_size: max_grid_y - 5 - room_count * 5 - (20 - room_count * 4),
                min_y_offset: 2 + room_count * 5,
                min_x_offset: 1,
                max_y_offset: 0,
                max_x_offset: 0,
            };
            y = rand::thread_rng().gen_range(new_room.min_y_size..new_room.max_y_size);
            x = rand::thread_rng().gen_range(new_room.min_x_size..new_room.max_x_size);
            new_room.max_y_offset = max_grid_y - y - 1 - (20 - room_count * 4);
            new_room.max_x_offset = max_grid_x - x - 1;
            offset_y = rand::thread_rng().gen_range(new_room.min_y_offset..=new_room.max_y_offset);
            offset_x = rand::thread_rng().gen_range(new_room.min_x_offset..=new_room.max_x_offset);
            enough_space = true;
            for a in 0..x + 2 { //cycle that goes through all cells in the room plus 1 cell outline
                for b in 0..y + 2 {
                    if grid[b + offset_y - 1][a + offset_x - 1] == "_".to_string() {
                        enough_space = false; // if in the newly randomly generated room there's a "_" (an intersection with another room), then there's not enough space
                    }
                }
            }
            let mut across_room = false; // there must be a room directly up, down, left or right
            if !is_first { // first room doesn't have to make tunnels to other rooms
                for a in 0..x { //cycle that goes through all cells in the room
                    for b in 0..y {
                        if grid[b + offset_y][a + offset_x] == "B".to_string() { // if there's a "B" and not a "#", then there's a room across
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
        }
        attempts += 1;
        if attempts > 5000 { // don't want it to be endless so after 1000 attempts it's over and returns the same grid
            return grid;
        }
    }
    for room_x in 0..max_grid_x //cycle that goes through all cells in the grid
    {
        for room_y in 0..max_grid_y
        {
            if (room_x >= offset_x && room_x < x + offset_x) || (room_y >= offset_y && room_y < y + offset_y) {
                if grid[room_y][room_x] == "B".to_string()  // turs all "B" that are horizontal and vertical to the room into "E".
                {                                                   // This'll be a sign for this room that it's across some other room
                    grid[room_y][room_x] = "E".to_string();
                } else if grid[room_y][room_x] == "#".to_string()
                {
                    grid[room_y][room_x] = "B".to_string(); // turs all "#" that are horizontal and vertical to the room into "B".
                }                                                   // This'll be a sign for other rooms that there's a room across somewhere
            }
        }
    }
    let mut door_cells: Vec<DoorCell> = vec![]; // cells that can make a tunnel to a room across. Later only one of them will be chosen.
    for room_x in 0..x
    {
        for room_y in 0..y
        {
            grid[room_y + offset_y][room_x + offset_x] = "_".to_string(); // fills the new room with "_"
        }
    }
    if is_first { // this one makes the upward tunnel for the first room and creates "^"
        let id = rand::thread_rng().gen_range(1..x - 1); // the position of the tunnel is also random
        let mut i = 1;
        loop {
            if offset_y - i != 0 {
                grid[offset_y - i][offset_x + id] = "_".to_string();
            } else {
                grid[offset_y - i][offset_x + id] = "^".to_string();
                return grid;
            }
            i = i + 1;
        }
    }
    for a in 0..x
    {
        for b in 0..y
        {
            let path_vec = can_connect(grid.clone(), a + offset_x, b + offset_y); // the direction in which the tunnel should go. Each cell is checked if it could a tunnel. more about in canConnect function
            if can_connect(grid.clone(), a + offset_x, b + offset_y) != vec![0, 0] { // If the cell can make a tunnel, it's added to door_cells
                let mut i = 1;
                loop {
                    let potentially_overflowing_y = (b as i32) + (offset_y as i32) + path_vec[1] * i;
                    let potentially_overflowing_x = (a as i32) + (offset_x as i32) + path_vec[0] * i;
                    let maybe_y = usize::try_from(potentially_overflowing_y).ok();
                    let maybe_x = usize::try_from(potentially_overflowing_x).ok();
                    if let Some((x, y)) = maybe_x.zip(maybe_y) {
                        if grid[y][x] != "_".to_string() { // also checks if there's no other tunnel/room in the way
                            let new_door_cell = DoorCell {
                                y: b + offset_y,
                                x: a + offset_x,
                                direction_y: path_vec[1],
                                direction_x: path_vec[0],
                            };
                            door_cells.push(new_door_cell);
                        } else {
                            break;
                        }
                        i = i + 1;
                    }
                }
            }
        }
    }
    if door_cells.len() > 1 { // randomly chooses a cell that will make the tunnel
        'outer: loop {
            let id = rand::thread_rng().gen_range(0..door_cells.len());
            let door_cell = &door_cells[id];
            let mut j = 1;
            'inner: loop {
                let y_shift = door_cell.direction_y * j;
                let x_shift = door_cell.direction_x * j;
                let maybe_new_y = usize::try_from(door_cell.y as i32 + y_shift).ok();
                let maybe_new_x = usize::try_from(door_cell.x as i32 + x_shift).ok();
                if let Some((new_x, new_y)) = maybe_new_x.zip(maybe_new_y) {
                    if new_y >= max_grid_y && new_x >= max_grid_x { // just to be sure it won't go out of bounds
                        continue 'inner;
                    }
                    if grid[new_y][new_x] != "_".to_string() {
                        // draws the tunnel until it hits "_" of another room
                        grid[new_y][new_x] = "_".to_string();
                    } else {
                        break 'outer;
                    }
                    j = j + 1;
                } else {
                    continue 'inner;
                }
            }
        }
    }
    return grid;
}


fn can_connect(grid: Vec<Vec<String>>, x: usize, y: usize) -> Vec<i32> {
    // returns a 2d Vector that points in which direction to build the tunnel
    let mut path_direction: Vec<i32> = vec![];
    path_direction.insert(0, 0);
    path_direction.insert(1, 0);


    // checks every direction, if there's "E", return thet direction
    if grid[y][x] == "_".to_string() {
        if (y - 1) > 0 && grid[y - 1][x] == "E".to_string() {
            let mut up = y - 1;
            loop {
                if grid[up][x] == "_".to_string() {
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
        if (x - 1) > 0 && grid[y][x - 1] == "E".to_string() {
            let mut left = x - 1;
            loop {
                if grid[y][left] == "_".to_string() {
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
        if (x + 1) < 65 && grid[y][x + 1] == "E".to_string() {
            let mut right = x + 1;
            loop {
                if grid[y][right] == "_".to_string() {
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
