use crate::util::*;
use std::num::ParseIntError;
use std::ops::Mul;
use std::str::FromStr;

use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Heading {
    East,
    West,
    North,
    South,
}

impl Heading {
    fn to_pos(&self) -> Position {
        match self {
            Heading::East => Position(1, 0),
            Heading::West => Position(-1, 0),
            Heading::North => Position(0, 1),
            Heading::South => Position(0, -1),
        }
    }

    fn turn_left(&self) -> Heading {
        match self {
            Heading::East => Heading::North,
            Heading::West => Heading::South,
            Heading::North => Heading::West,
            Heading::South => Heading::East,
        }
    }

    fn turn_right(&self) -> Heading {
        match self {
            Heading::East => Heading::South,
            Heading::West => Heading::North,
            Heading::North => Heading::East,
            Heading::South => Heading::West,
        }
    }

    fn turn_around(&self) -> Heading {
        match self {
            Heading::East => Heading::West,
            Heading::West => Heading::East,
            Heading::North => Heading::South,
            Heading::South => Heading::North,
        }
    }
}

impl Mul<i64> for Heading {
    type Output = Position;

    fn mul(self, other: i64) -> Self::Output {
        let dir = self.to_pos();
        Position(other * dir.0, other * dir.1)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Position(i64, i64);

impl Position {
    fn turn_left(&mut self) {
        *self = Position(-self.1, self.0);
    }

    fn turn_around(&mut self) {
        *self = Position(-self.0, -self.1);
    }

    fn turn_right(&mut self) {
        *self = Position(self.1, -self.0);
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

impl Add<Heading> for Position {
    type Output = Self;

    fn add(self, other: Heading) -> Self::Output {
        let dir = other.to_pos();
        Self(self.0 + dir.0, self.1 + dir.1)
    }
}

impl AddAssign<Heading> for Position {
    fn add_assign(&mut self, other: Heading) {
        let dir = other.to_pos();
        *self = Self(self.0 + dir.0, self.1 + dir.1);
    }
}

impl Mul<i64> for Position {
    type Output = Position;

    fn mul(self, other: i64) -> Self::Output {
        Position(other * self.0, other * self.1)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Ship1 {
    heading: Heading,
    position: Position,
}

impl AddAssign<Movement> for Ship1 {
    fn add_assign(&mut self, other: Movement) {
        match other.direction {
            Direction::Heading(h) => self.position += h * other.amount,
            Direction::Forward => self.position += self.heading * other.amount,
            Direction::Left => {
                if other.amount == 90 {
                    self.heading = self.heading.turn_left();
                } else if other.amount == 180 {
                    self.heading = self.heading.turn_around();
                } else {
                    self.heading = self.heading.turn_right();
                }
            }
            Direction::Right => {
                if other.amount == 90 {
                    self.heading = self.heading.turn_right();
                } else if other.amount == 180 {
                    self.heading = self.heading.turn_around();
                } else {
                    self.heading = self.heading.turn_left();
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Ship2 {
    position: Position,
    waypoint_pos: Position,
}

impl AddAssign<Movement> for Ship2 {
    fn add_assign(&mut self, other: Movement) {
        match other.direction {
            Direction::Heading(h) => self.waypoint_pos += h * other.amount,
            Direction::Forward => self.position += self.waypoint_pos * other.amount,
            Direction::Left => {
                if other.amount == 90 {
                    self.waypoint_pos.turn_left();
                } else if other.amount == 180 {
                    self.waypoint_pos.turn_around();
                } else {
                    self.waypoint_pos.turn_right();
                }
            }
            Direction::Right => {
                if other.amount == 90 {
                    self.waypoint_pos.turn_right();
                } else if other.amount == 180 {
                    self.waypoint_pos.turn_around();
                } else {
                    self.waypoint_pos.turn_left();
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Heading(Heading),
    Forward,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Movement {
    direction: Direction,
    amount: i64,
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Movement, Self::Err> {
        let s = s.chars().collect::<Vec<char>>();
        let (dir, amnt) = s.split_at(1);
        let dir = match dir[0] {
            'N' => Direction::Heading(Heading::North),
            'S' => Direction::Heading(Heading::South),
            'E' => Direction::Heading(Heading::East),
            'W' => Direction::Heading(Heading::West),
            'F' => Direction::Forward,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        };

        Ok(Movement {
            direction: dir,
            amount: amnt.iter().collect::<String>().parse::<i64>()?,
        })
    }
}

pub fn solve() -> (Option<i64>, Option<i64>) {
    let movements = read_file_to_vec::<Movement>("src/days/input/12");

    let mut ship = Ship1 {
        heading: Heading::East,
        position: Position(0, 0),
    };
    for m in &movements {
        ship += *m;
    }
    let sol1 = ship.position.0.abs() + ship.position.1.abs();

    let mut ship = Ship2 {
        position: Position(0, 0),
        waypoint_pos: Position(10, 1),
    };
    for m in movements {
        ship += m;
    }
    let sol2 = ship.position.0.abs() + ship.position.1.abs();

    (Some(sol1), Some(sol2))
}
