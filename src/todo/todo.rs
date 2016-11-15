use todo::Priority;
use term;

#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Todo {
    pub id: u64,
    pub name: String,
    pub details: String,
    pub done: bool,
    pub priority: Priority,
}

impl Todo {
    pub fn show(&self) {
        let mut t = term::stdout().unwrap();

        if self.done {
            t.attr(term::Attr::Dim).unwrap();
            write!(t, "✔ ").unwrap();
        } else {
            write!(t, "  ").unwrap();
        }

        match self.priority {
            Priority::Critical => {
                t.fg(term::color::BRIGHT_MAGENTA).unwrap();
                write!(t, "C").unwrap();
                t.reset().unwrap();
            },
            Priority::High => {
                t.fg(term::color::BRIGHT_RED).unwrap();
                write!(t, "H").unwrap();
                t.reset().unwrap();
            },
            Priority::Medium => {
                t.fg(term::color::BRIGHT_YELLOW).unwrap();
                write!(t, "M").unwrap();
                t.reset().unwrap();
            },
            Priority::Low => {
                t.fg(term::color::BRIGHT_GREEN).unwrap();
                write!(t, "L").unwrap();
                t.reset().unwrap();
            },
            Priority::Unknown => {
                t.fg(term::color::BRIGHT_CYAN).unwrap();
                write!(t, "?").unwrap();
                t.reset().unwrap();
            },
        }

        write!(t, " #{} {}", self.id, self.name).unwrap();
        t.reset().unwrap();
    }

    pub fn done(&self, d: bool) -> Self {
        let mut t = self.clone();
        t.done = d;
        t
    }
}
