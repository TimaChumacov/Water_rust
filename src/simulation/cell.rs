/// checks if the cell is water.
/// The reason why "W" is explained in is_wall function
pub fn is_water(value: &String) -> bool {
    value == "0" || value == "(" || value == ")"
}

/// checks if the cell is water.
/// "W" is here because it doesn't have to move, and is pretty much a wall once placed.
/// It would lag if "W" was water
pub fn is_wall(value: &String) -> bool {
    value == "#" || value == "B" || value == "E" || value == "W"
}


