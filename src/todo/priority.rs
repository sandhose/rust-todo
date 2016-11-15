use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
use std::cmp::Ordering;
use clap::ArgMatches;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn to_u8(&self) -> u8 {
        match *self {
            Priority::Unknown => 0,
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }

    fn from_u8(v: u8) -> Self {
        match v {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            4 => Priority::Critical,
            _ => Priority::Unknown,
        }
    }

    pub fn from_matches(m: &ArgMatches) -> Self {
        if m.is_present("critical") { Priority::Critical }
        else if m.is_present("high") { Priority::High }
        else if m.is_present("medium") { Priority::Medium }
        else if m.is_present("low") { Priority::Low }
        else if let Some(e) = m.value_of("pr") {
            match e {
                "critical" | "crit" | "c" => Priority::Critical,
                "high" | "h" => Priority::High,
                "medium" | "m" => Priority::Medium,
                "low" | "l" => Priority::Low,
                _ => Priority::Unknown,
            }
        }
        else { Priority::Unknown }

    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Priority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Priority) -> Ordering {
        self.to_u8().cmp(&other.to_u8())
    }
}


impl Encodable for Priority {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_u8(self.to_u8())
    }
}

impl Decodable for Priority {
    fn decode<D: Decoder>(d: &mut D) -> Result<Priority, D::Error> {
        Ok(Priority::from_u8(d.read_u8()?))
    }
}
