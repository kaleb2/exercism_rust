// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction: Direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            x: self.x,
            y: self.y,
            direction: new_direction,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction: Direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            x: self.x,
            y: self.y,
            direction: new_direction,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self {
                x: self.x,
                y: self.y + 1,
                direction: self.direction,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
                direction: self.direction,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y - 1,
                direction: self.direction,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
                direction: self.direction,
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut new_robot: Robot = Self {
            x: self.x,
            y: self.y,
            direction: self.direction,
        };
        for c in instructions.chars() {
            new_robot = match c {
                'R' => new_robot.turn_right(),
                'L' => new_robot.turn_left(),
                'A' => new_robot.advance(),
                _ => panic!("Instruction not allowed"),
            };
        }
        Self {
            x: new_robot.x,
            y: new_robot.y,
            direction: new_robot.direction,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
