#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

use std::collections::{HashSet, VecDeque};

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1.max(capacity_2) {
        return None;
    }

    let gcd = |mut a: u8, mut b: u8| {
        while b > 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    };

    if goal != 0 && gcd(capacity_1, capacity_2) == 0 {
        return None;
    }

    if goal != 0 && !goal.is_multiple_of(gcd(capacity_1, capacity_2)) {
        return None;
    }

    let start_state = match start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };

    let is_invalid_state = |state: (u8, u8)| match start_bucket {
        Bucket::One => state.0 == 0 && state.1 == capacity_2,
        Bucket::Two => state.1 == 0 && state.0 == capacity_1,
    };

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start_state);
    queue.push_back((start_state, 1u8));

    while let Some(((a, b), moves)) = queue.pop_front() {
        if a == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: b,
            });
        }
        if b == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: a,
            });
        }

        let neighbors = [
            (capacity_1, b),
            (a, capacity_2),
            (0, b),
            (a, 0),
            {
                let transfer = a.min(capacity_2 - b);
                (a - transfer, b + transfer)
            },
            {
                let transfer = b.min(capacity_1 - a);
                (a + transfer, b - transfer)
            },
        ];

        for state in neighbors.iter().copied() {
            if is_invalid_state(state) {
                continue;
            }
            if visited.insert(state) {
                queue.push_back((state, moves.saturating_add(1)));
            }
        }
    }

    None
}