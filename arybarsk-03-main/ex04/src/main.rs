use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

struct Time {
    hours: u32,
    minutes: u32,
}

enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl FromStr for Time
{
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let timebytes = s.as_bytes();

        if timebytes.len() != 5
            { return Err(TimeParseError::InvalidLength); }
        
        if timebytes[2] != b':'
            { return Err(TimeParseError::MissingColon); }
        
        if !u8::is_ascii_digit(&timebytes[0]) || !u8::is_ascii_digit(&timebytes[1])
            || !u8::is_ascii_digit(&timebytes[3]) || !u8::is_ascii_digit(&timebytes[4])
            { return Err(TimeParseError::InvalidNumber); }
        
        let hours = (timebytes[0] - b'0') as u32 * 10 + (timebytes[1] - b'0') as u32;
        let minutes = (timebytes[3] - b'0') as u32 * 10 + (timebytes[4] - b'0') as u32;
    
        if minutes > 59 || hours > 23
            { return Err(TimeParseError::InvalidNumber); }
        
        Ok(Time {hours, minutes})
    }
}

impl Display for Time
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.hours == 1 && self.minutes == 1
            { write!(formatter, "{} hour, {} minute", self.hours, self.minutes) }
        else if self.hours == 1
            { write!(formatter, "{} hour, {} minutes", self.hours, self.minutes) }
        else if self.minutes == 1
            { write!(formatter, "{} hours, {} minute", self.hours, self.minutes) }
        else
            { write!(formatter, "{} hours, {} minutes", self.hours, self.minutes) }
    }
}

impl Display for TimeParseError
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            TimeParseError::MissingColon => write!(formatter, "missing ':'"),
            TimeParseError::InvalidLength => write!(formatter, "invalid length"),
            TimeParseError::InvalidNumber => write!(formatter, "invalid number"),  
        }
    }
}

impl Debug for Time
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        formatter.debug_struct("TimeParseError")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}

impl Debug for TimeParseError
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        formatter.debug_struct("TimeParseError")
            .finish()
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}