use range_check;


/// A named day of the week.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Weekday {

    /// **Monday**, the first day of the week.
    ///
    /// The name comes from the Anglo-Saxon *monandaeg*, meaning “moon’s day”.
    Monday,

    /// **Tuesday**, the second day of the week.
    ///
    /// Named after the Norse god *Tyr*.
    Tuesday,

    /// **Wednesday**, the third day of the week.
    ///
    /// Named after the Norse god *Odin*.
    Wednesday,

    /// **Thursday**, the fourth day of the week.
    ///
    /// Named after the Norse god *Thor*.
    Thursday,

    /// **Friday**, the fifth day of the week.
    ///
    /// Named after the Norse goddess *Frigg*.
    Friday,

    /// **Saturday**, the sixth day of the week.
    ///
    /// The name comes from the Latin *dies Saturni*, meaning “Saturn’s day”.
    Saturday,

    /// **Sunday**, the seventh and last day of the week.
    ///
    /// The name comes from the Latin *dies solis*, meaning “sun’s day”.
    Sunday,
}

impl Weekday {

    /// Returns the number of days this weekday is from Monday, with Monday as
    /// 1 day, Tuesday as 2 days, and so on until Sunday as 7 days.
    ///
    /// ```rust
    /// use datetime::cal::unit::Weekday;
    ///
    /// assert_eq!(Weekday::Saturday.days_from_monday_as_one(), 6);
    /// assert_eq!(Weekday::Sunday.days_from_monday_as_one(), 7);
    /// ```
    pub fn days_from_monday_as_one(&self) -> i8 {
        use self::Weekday::*;

        match *self {        Monday   => 1,  Tuesday => 2,
            Wednesday => 3,  Thursday => 4,  Friday  => 5,
            Saturday  => 6,  Sunday   => 7,
        }
    }

    /// Returns the weekday based on a number, with Sunday as Day 0, Monday as
    /// Day 1, and so on.
    ///
    /// ```rust
    /// use datetime::cal::unit::Weekday;
    ///
    /// assert_eq!(Weekday::from_zero(4), Ok(Weekday::Thursday));
    /// assert!(Weekday::from_zero(7).is_err());
    /// ```
    pub fn from_zero(weekday: i8) -> range_check::Result<Weekday, i8> {
        use self::Weekday::*;

        Ok(match weekday {
            0 => Sunday,     1 => Monday,    2 => Tuesday,
            3 => Wednesday,  4 => Thursday,  5 => Friday,
            6 => Saturday,
            n => return Err(range_check::Error::new(n, 0..7)),
        })
    }

    /// Returns the weekday based on a number, with Monday as Day 1, and Sunday
    /// as Day 7.
    ///
    /// ```rust
    /// use datetime::cal::unit::Weekday;
    ///
    /// assert_eq!(Weekday::from_one(4), Ok(Weekday::Thursday));
    /// assert!(Weekday::from_one(0).is_err());
    /// ```
    pub fn from_one(weekday: i8) -> range_check::Result<Weekday, i8> {
        use self::Weekday::*;

        Ok(match weekday {   1 => Monday,    2 => Tuesday,
            3 => Wednesday,  4 => Thursday,  5 => Friday,
            6 => Saturday,   7 => Sunday,
            n => return Err(range_check::Error::new(n, 1..8)),
        })
    }
}
