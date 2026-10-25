struct Solution;

static NUMBER_OF_DAYS: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

struct Date {
    month: i32,
    day: i32,
}

impl Date {
    pub fn parse_days(date: String) -> Self {
        let (month, day) = date.split_once('-').unwrap();
        Self {
            month: month.parse().unwrap(),
            day: day.parse().unwrap(),
        }
    }

    fn date_to_day_of_year(self) -> i32 {
        let mut total_days = self.day;
        for m in 1..self.month as usize {
            total_days += NUMBER_OF_DAYS[m];
        }
        total_days
    }
}

struct PersonLive {
    arrive: Date,
    departure: Date,
}

impl PersonLive {
    pub fn correcponding_days(self, other: PersonLive) -> i32 {
        let overlap_arrive = std::cmp::max(
            self.arrive.date_to_day_of_year(),
            other.arrive.date_to_day_of_year(),
        );

        let overlap_departure = std::cmp::min(
            self.departure.date_to_day_of_year(),
            other.departure.date_to_day_of_year(),
        );

        if overlap_arrive <= overlap_departure {
            return overlap_departure - overlap_arrive + 1;
        }
        0
    }

    pub fn dates_to_days_of_year(self) -> (i32, i32) {
        (
            self.arrive.date_to_day_of_year(),
            self.departure.date_to_day_of_year(),
        )
    }
}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let alice = PersonLive {
            arrive: Date::parse_days(arrive_alice),
            departure: Date::parse_days(leave_alice),
        };

        let bob = PersonLive {
            arrive: Date::parse_days(arrive_bob),
            departure: Date::parse_days(leave_bob),
        };

        alice.correcponding_days(bob)
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::solutions::count_days_spent_together::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::count_days_together(
                "08-15".into(),
                "08-18".into(),
                "08-16".into(),
                "08-19".into()
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::count_days_together(
                "10-01".into(),
                "10-31".into(),
                "11-01".into(),
                "12-31".into()
            ),
            0
        );
    }
}
