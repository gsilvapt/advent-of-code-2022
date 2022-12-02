pub struct Draws(Vec<(char, char)>);

impl Draws {
    pub fn from(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    (
                        line.chars().nth(0).expect("must have first choice"),
                        line.chars().nth(1).expect("must have second choice"),
                    )
                })
                .collect(),
        )
    }
}
