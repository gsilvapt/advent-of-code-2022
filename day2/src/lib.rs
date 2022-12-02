pub fn to_shapes(lines: String) -> Vec<(Shape, Shape)> {
    // to_shapes takes a string, splits by line and tries to parse Char primitives into Shape.
    // Looks for the first and the second char and the format of the file should be 'A B', thus we
    // need to be careful to skip the space.
    // Another caveat of the exercise is that the second position is actual the player's hand.
    // The stupid hack there was to invert the nth to then to do scoring.
    // Returns a vector with the tuples of shapes for later computing.
    lines
        .lines()
        .map(|line| {
            let first: Shape =
                match Shape::into_shape(line.chars().nth(2).expect("must have first choice")) {
                    Ok(v) => v,
                    Err(e) => panic!("failed to parse shape into enum: {e:?}"),
                };
            let second: Shape =
                match Shape::into_shape(line.chars().nth(0).expect("must have second choice")) {
                    Ok(v) => v,
                    Err(e) => panic!("failed to parse shape into enum: {e:?}"),
                };
            (first, second)
        })
        .collect()
}

pub enum Shape {
    // Shape is the enum with the three types of shapes the game can assume.
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    pub fn reverse_shape_finder(result: &Draws, oppo: &Self) -> Self {
        // reverse_shape_finder checks the output from result and the opponent play, returning 
        // the Shape that would lead to that result.
        match oppo {
            Shape::Rock => {
                match result {
                    Draws::Win => Shape::Paper,
                    Draws::Loss => Shape::Scissor,
                    Draws::Draw => Shape::Rock,
                }
            },
            Shape::Paper => {
                match result {
                    Draws::Win => Shape::Scissor,
                    Draws::Loss => Shape::Rock,
                    Draws::Draw => Shape::Paper,
                }
            },
            Shape::Scissor => {
                match result {
                    Draws::Win => Shape::Rock,
                    Draws::Loss => Shape::Paper,
                    Draws::Draw => Shape::Scissor,
                }
            },
        }
    }

    pub fn into_shape(c: char) -> Result<Self, &'static str> {
        // into_shape converts a char to a shape, returning a Result with the corresponding type.
        // For unrecognized characters, it returns an error.
        match c {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissor),
            _ => Err("unrecognized char"),
        }
    }

    pub fn score(self: &Self, other: &Shape) -> i32 {
        // score compares the current shape with another and adds the base score.
        // Returns total sum
        let result: &Draws = self.compute_result(other);
        self.base_score() + result.score()
    }

    fn compute_result(self: &Self, other: &Self) -> &Draws {
        // compute_resultis a match matrix to create a Draw instance of the correct type,
        // considering all the shapes and all the game possibilities.
        match self {
            Shape::Rock => match other {
                Shape::Rock => &Draws::Draw,
                Shape::Scissor => &Draws::Win,
                Shape::Paper => &Draws::Loss,
            },
            Shape::Scissor => match other {
                Shape::Rock => &Draws::Loss,
                Shape::Paper => &Draws::Win,
                Shape::Scissor => &Draws::Draw,
            },
            Shape::Paper => match other {
                Shape::Rock => &Draws::Win,
                Shape::Paper => &Draws::Draw,
                Shape::Scissor => &Draws::Loss,
            },
        }
    }

    fn base_score(self: &Self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

pub enum Draws {
    Win,
    Loss,
    Draw,
}

impl Draws {
    pub fn score(&self) -> i32 {
        match self {
            Draws::Win => 6,
            Draws::Loss => 0,
            Draws::Draw => 3,
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'X' => Draws::Loss,
            'Y' => Draws::Draw,
            'Z' => Draws::Win,
            _ => Draws::Draw, // poop
        }
    }
}
