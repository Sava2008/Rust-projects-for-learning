	 /* task: the robot is on coordinates [0, 0]. '.' is take 1 step forward in the current 
		direction, '<' is to rotate 90 degrees counterclockwise and '>' - 90 degrees clockwise.
		At the start the robot is facing East. Write a function that accepts a string as argument
		and evaluates the robot's position after all steps and rotations. Notice that coordinates 
		shouldn't go negative. example:
		
		input - "..<.>..<<...<.."
		1. '.' -> [1, 0];
		2. '.' -> [2, 0];
		3. '<' -> facing: north;
		4. '.' -> [2, 0];
		5. '>' -> facing: east;
		6. '.' -> [3, 0];
		7. '.' -> [4, 0];
		8. '<' -> facing: north;
		9. '<' -> facing: west;
		10. '.' -> [3, 0];
		11. '.' -> [2, 0];
		12. '.' -> [1, 0];
		13. '<' -> facing: south;
		14. '.' -> [1, 1]; 
		15. '.' -> [1, 2] */

fn main() {
	println!("{:?}", track_robot("..<.>..<<...<.."));
}

fn track_robot<'a>(steps: &'a str) -> Result<[u8; 2], String> {
	let mut robot_coords: [u8; 2] = [0, 0];
	if steps.len() == 0 {
		return Ok(robot_coords);
	}
	let mut robot_direction: u8 = 2;
	for command in steps.chars() {
		match command {
			'.' => { match robot_direction {
						1 => { if robot_coords[1] > 0 { robot_coords[1] -= 1; } },
						2 => { robot_coords[0] += 1; },
						3 => { robot_coords[1] += 1; },
						4 => { if robot_coords[0] > 0 { robot_coords[0] -= 1; } },
						_ => return Err(format!("{robot_direction} should be in range 1..=4")),
				}; }
			'>' => { if robot_direction < 4 { robot_direction += 1; } 
					 else { robot_direction = 1; } },
			'<' => { if robot_direction > 1 { robot_direction -= 1; }
					 else { robot_direction = 4; } },
			_ => return Err(format!("{command} is not a supported command")), }
	}
	return Ok(robot_coords);
}
