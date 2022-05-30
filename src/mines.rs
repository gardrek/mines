use std::collections::HashSet;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Default)]
pub struct MinesGame {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl std::fmt::Display for MinesGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                let (open, close) = if self.flagged_fields.contains(&pos) {
                    ("(", ")")
                } else {
                    if self.open_fields.contains(&pos) {
                        (" ", " ")
                    } else {
                        ("[", "]")
                    }
                };

                let middle = if self.open_fields.contains(&pos) {
                    if self.mines.contains(&pos) {
                        "*".to_string()
                    } else {
                        let mine_count = self.count_neighboring_mines(pos);

                        if mine_count == 0 {
                            " ".to_string()
                        } else {
                            mine_count.to_string()
                        }
                    }
                } else {
                    if self.flagged_fields.contains(&pos) && !self.open_fields.contains(&pos) {
                        "F".to_string()
                    } else {
                        " ".to_string()
                    }
                };

                write!(f, "{}{}{}", open, middle, close)?;

                /*
                if !self.open_fields.contains(&pos) {
                    if self.flagged_fields.contains(&pos) {
                        f.write_str("(F)")?;
                    } else {
                        f.write_str("[ ]")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str(" * ")?;
                } else {
                    let mine_count = self.count_neighboring_mines(pos);
                    if mine_count == 0 {
                        write!(f, "   ")?;
                    } else {
                        write!(f, " {} ", mine_count)?;
                    }
                }
                */

                f.write_str("|")?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl MinesGame {
    pub fn new(
        width: usize,
        height: usize,
        mine_count: usize,
        prng: &mut prng::Prng16,
    ) -> MinesGame {
        assert!(
            mine_count <= width * height,
            "Minefield was not big enough to hold all mines"
        );

        MinesGame {
            width,
            height,
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((
                        (prng.next().unwrap() as usize % width),
                        (prng.next().unwrap() as usize % height),
                    ));
                }

                mines
            },
            ..Default::default()
        }
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let (width, height) = (self.width, self.height);

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn count_neighboring_mines(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|position| self.mines.contains(position))
            .count() as u8
    }

    pub fn count_neighboring_flags(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|position| self.flagged_fields.contains(position))
            .count() as u8
    }

    pub fn click_open(&mut self, position: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&position) {
            return None;
        }

        if self.open_fields.contains(&position) {
            let mine_count = self.count_neighboring_mines(position);
            let flag_count = self.count_neighboring_flags(position);

            if mine_count == flag_count {
                for neighbor in self
                    .iter_neighbors(position)
                {
                    if !self.flagged_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            return None;
        }

        self.open(position)
    }

    fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.lost || self.open_fields.contains(&position) {
            return None;
        }

        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        Some(if is_mine {
            // TODO: what a waste haha
            let mines = self.mines.clone();

            for mine in mines.iter() {
                self.open(*mine);
            }

            self.lost = true;

            OpenResult::Mine
        } else {
            let mine_count = self.count_neighboring_mines(position);

            if mine_count == 0 {
                for neighbor in self.iter_neighbors(position) {
                    self.open(neighbor);
                }
            }

            OpenResult::NoMine(mine_count)
        })
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.lost {
            return;
        }

        if self.flagged_fields.contains(&position) {
            self.flagged_fields.remove(&position);
        } else if !self.open_fields.contains(&position) {
            self.flagged_fields.insert(position);
        }
    }
}
