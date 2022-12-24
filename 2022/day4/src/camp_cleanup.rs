trait Overlapping {
    fn overlaps(&self, other: Self) -> bool;
    fn partial_overlaps(&self, other: Self) -> bool;
}

impl Overlapping for (u64, u64) {
    fn overlaps(&self, other: Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn partial_overlaps(&self, other: Self) -> bool {
        self.0 <= other.0 && (self.1 <= other.1 && self.1 >= other.0)
    }
}

struct AssignmentPair {
    first: (u64, u64),
    second: (u64, u64),
}

impl From<String> for AssignmentPair {
    fn from(line: String) -> Self {
        let assignments = line
            .split(',')
            .map(|assignment| {
                let range = assignment.split('-').collect::<Vec<_>>();
                (
                    range[0].parse::<u64>().unwrap(),
                    range[1].parse::<u64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        AssignmentPair {
            first: assignments[0],
            second: assignments[1],
        }
    }
}

impl AssignmentPair {
    pub fn contains_overlapping_range(&self) -> bool {
        self.first.overlaps(self.second) || self.second.overlaps(self.first)
    }

    pub fn contains_partial_overlapping_range(&self) -> bool {
        self.first.partial_overlaps(self.second) || self.second.partial_overlaps(self.first)
    }
}

pub fn find_fully_contained_amount() {
    let lines = common::get_lines!();
    let assignments = lines
        .into_iter()
        .map(|line| AssignmentPair::from(line.unwrap()))
        .collect::<Vec<_>>();

    let overlapping = assignments
        .iter()
        .filter(|assignment| assignment.contains_overlapping_range())
        .count();

    let partial_overlapping = assignments
        .iter()
        .filter(|assignment| {
            assignment.contains_partial_overlapping_range()
                || assignment.contains_overlapping_range()
        })
        .count();

    println!("Overlapping: {overlapping}");
    println!("Partial overlapping: {partial_overlapping}");
}
