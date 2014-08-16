#[deriving(Eq, PartialEq, Show)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: u8,
}

impl Date {
    /// precondition: date is a valid, legal date
    pub fn iter(&self, day: Day) -> DateIterator {
        let mut iter = DateIterator { date: *self, day: day };
        iter.next_back();
        iter
    }

    pub fn days_in_month(&self) -> u8 {
        match self.month {
            September => 30, // Thirty days has September,
            April | June | November => 30, // April, June and November.
            January | March | May | July | August | October | December =>
                31, // All the rest have thirty-one,
            February => { // Saving February alone,
                let mut days = 28; // Which has twenty-eight, rain or shine.
                if self.year.is_leap_year() { days += 1 } // And on leap years, twenty-nine.
                days
            },
        }
    }
}

pub struct DateIterator {
    date: Date,
    day: Day,
}

impl Iterator<(Date, Day)> for DateIterator {
    fn next(&mut self) -> Option<(Date, Day)> {
        let ref mut date = self.date;
        if date.day == date.days_in_month() {
            date.day = 1;
            if date.month.next() == Some(January) {
                date.year.next();
            }
        } else {
            date.day += 1;
        }
        Some((*date, self.day.next().unwrap()))
    }
}

impl DoubleEndedIterator<(Date, Day)> for DateIterator {
    fn next_back(&mut self) -> Option<(Date, Day)> {
        let ref mut date = self.date;
        if date.day == 1 {
            if date.month.next_back() == Some(December) {
                date.year.next_back();
            }
            date.day = date.days_in_month();
        } else {
            date.day -= 1;
        }
        Some((*date, self.day.next_back().unwrap()))
    }
}

#[deriving(Eq, PartialEq, Show)]
pub struct Year(i32);

impl Year {
    pub fn is_leap_year(&self) -> bool {
        match *self { Year(year) => // A leap year occurs
            year % 4 == 0 && ( // on any year evenly divisible by 4,
            year % 100 != 0 || // but not on a century
            year % 400 == 0) // unless it is divisible by 400.
        }
    }
}

impl Iterator<Year> for Year {
    fn next(&mut self) -> Option<Year> {
        {
            let Year(ref mut year) = *self;
            *year += 1;
        }
        Some(*self)
    }
}

impl DoubleEndedIterator<Year> for Year {
    fn next_back(&mut self) -> Option<Year> {
        {
            let Year(ref mut year) = *self;
            *year -= 1;
        }
        Some(*self)
    }
}

#[deriving(Eq, FromPrimitive, PartialEq, Show)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Iterator<Month> for Month {
    fn next(&mut self) -> Option<Month> {
        let month = FromPrimitive::from_u8(*self as u8 + 1).unwrap_or(January);
        *self = month;
        Some(month)
    }
}

impl DoubleEndedIterator<Month> for Month {
    fn next_back(&mut self) -> Option<Month> {
        let month = FromPrimitive::from_i8(*self as i8 - 1).unwrap_or(December);
        *self = month;
        Some(month)
    }
}

#[deriving(Eq, PartialEq, Show)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Iterator<Day> for Day {
    fn next(&mut self) -> Option<Day> {
        let day = match *self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        };
        *self = day;
        Some(day)
    }
}

impl DoubleEndedIterator<Day> for Day {
    fn next_back(&mut self) -> Option<Day> {
        let day = match *self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        };
        *self = day;
        Some(day)
    }
}

euler_problem!(b"a4a042cf4fd6bfb47701cbc8a1653ada", w, {
    static min_date: Date = Date { year: Year(1900), month: January, day: 1 };
    static min_day: Day = Monday;
    static start_date: Date = Date { year: Year(1901), month: January, day: 1 };
    static end_date: Date = Date { year: Year(2001), month: January, day: 1 };

    let count = min_date.iter(min_day)
        .skip_while( |&(date, _)| date != start_date )
        .take_while( |&(date, _)| date != end_date )
        .filter( |&(date, day)| day == Sunday && date.day == 1)
        .count();
    write!(w, "{}", count)
})
