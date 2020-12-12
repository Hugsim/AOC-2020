use crate::util::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Seat, ()> {
        match s {
            "." => Ok(Seat::Floor),
            "L" => Ok(Seat::Empty),
            "#" => Ok(Seat::Occupied),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct SeatState {
    seats: Vec<Vec<Seat>>,
}

impl SeatState {
    fn get_nearby(&self, x: usize, y: usize) -> Vec<Seat> {
        let mut ret = Vec::new();
        for r in (y as isize - 1).max(0) as usize..=(y + 1).min(self.seats.len() - 1) {
            for c in (x as isize - 1).max(0) as usize..=(x + 1).min(self.seats[0].len() - 1) {
                if !(c == x && r == y) {
                    ret.push((c, r));
                }
            }
        }

        ret.iter().map(|(c, r)| self.seats[*r][*c]).collect()
    }

    fn num_nearby(&self, x: usize, y: usize) -> (i64, i64) {
        let mut nearby_seats = self.get_nearby(x, y);
        let num_free_seats = nearby_seats
            .drain_filter(|x| matches!(x, Seat::Empty))
            .count();

        let num_occupied_seats = nearby_seats
            .iter()
            .filter(|x| matches!(x, Seat::Occupied))
            .count();

        (num_free_seats as i64, num_occupied_seats as i64)
    }

    fn num_nearby_far(&self, x: usize, y: usize) -> (i64, i64) {
        let mut free = 0;
        let mut occupied = 0;
        for x_diff in -1..=1 {
            'inner: for y_diff in -1..=1 {
                if x_diff == 0 && y_diff == 0 {
                    continue;
                }
                if x as i64 + x_diff < 0 || (x as i64 + x_diff) as usize >= self.seats[0].len() {
                    continue 'inner;
                }
                if y as i64 + y_diff < 0 || (y as i64 + y_diff) as usize >= self.seats.len() {
                    continue 'inner;
                }
                let mut x = x as i64 + x_diff;
                let mut y = y as i64 + y_diff;
                while self.seats[y as usize][x as usize] == Seat::Floor {
                    if x as i64 + x_diff < 0 || (x as i64 + x_diff) as usize >= self.seats[0].len()
                    {
                        continue 'inner;
                    }
                    if y as i64 + y_diff < 0 || (y as i64 + y_diff) as usize >= self.seats.len() {
                        continue 'inner;
                    }
                    x += x_diff;
                    y += y_diff;
                }
                match self.seats[y as usize][x as usize] {
                    Seat::Occupied => occupied += 1,
                    Seat::Empty => free += 1,
                    _ => unreachable!(),
                }
            }
        }

        (free, occupied)
    }

    fn step(&mut self) {
        let mut new_seats = Vec::new();
        for (y, r) in self.seats.iter().enumerate() {
            new_seats.push(Vec::new());
            for (x, seat) in r.iter().enumerate() {
                match seat {
                    Seat::Floor => {
                        new_seats[y].push(Seat::Floor);
                    }
                    Seat::Empty => {
                        let (_free, occupied) = self.num_nearby(x, y);
                        if occupied == 0 {
                            new_seats[y].push(Seat::Occupied);
                        } else {
                            new_seats[y].push(Seat::Empty);
                        }
                    }
                    Seat::Occupied => {
                        let (_free, occupied) = self.num_nearby(x, y);
                        if occupied >= 4 {
                            new_seats[y].push(Seat::Empty);
                        } else {
                            new_seats[y].push(Seat::Occupied);
                        }
                    }
                }
            }
        }

        self.seats = new_seats;
    }

    fn step2(&mut self) {
        let mut new_seats = Vec::new();
        for (y, r) in self.seats.iter().enumerate() {
            new_seats.push(Vec::new());
            for (x, seat) in r.iter().enumerate() {
                match seat {
                    Seat::Floor => {
                        new_seats[y].push(Seat::Floor);
                    }
                    Seat::Empty => {
                        let (_free, occupied) = self.num_nearby_far(x, y);
                        if occupied == 0 {
                            new_seats[y].push(Seat::Occupied);
                        } else {
                            new_seats[y].push(Seat::Empty);
                        }
                    }
                    Seat::Occupied => {
                        let (_free, occupied) = self.num_nearby_far(x, y);
                        if occupied >= 5 {
                            new_seats[y].push(Seat::Empty);
                        } else {
                            new_seats[y].push(Seat::Occupied);
                        }
                    }
                }
            }
        }

        self.seats = new_seats;
    }

    fn count_occupied(&self) -> i64 {
        self.seats
            .concat()
            .iter()
            .filter(|&&s| s == Seat::Occupied)
            .count() as i64
    }

    fn as_string(&self) -> String {
        self.seats
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| match s {
                        Seat::Floor => '.',
                        Seat::Empty => 'L',
                        Seat::Occupied => '#',
                    })
                    .collect::<String>()
            })
            .collect::<String>()
    }
}

pub fn solve() -> (Option<i64>, Option<i64>) {
    let data = read_file_to_vec::<String>("src/days/input/11");
    let data: Vec<Vec<Seat>> = data
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<Seat>().unwrap())
                .collect()
        })
        .collect();

    let mut seats = SeatState {
        seats: data.clone(),
    };
    let mut old_states = HashSet::new();
    old_states.insert(seats.as_string());
    // sprint(seats.as_string());

    // let mut new = seats.step();
    let sol1 = loop {
        seats.step();
        if !old_states.insert(seats.as_string()) {
            break seats.count_occupied();
        }
    };

    let mut seats = SeatState { seats: data };
    let mut old_states = HashSet::new();
    old_states.insert(seats.as_string());
    // sprint(seats.as_string());

    // let mut new = seats.step();

    let sol2 = loop {
        seats.step2();
        if !old_states.insert(seats.as_string()) {
            break seats.count_occupied();
        }
    };
    (Some(sol1), Some(sol2))
}
