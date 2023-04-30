// Log per a manegar els logs que pugui crear la execució del programa,
// i treure'ls per pantalla quan l'usuari vulgui. En substitució del 

use colored::Colorize;

enum LogEntry {
    Info(String),
    Event(String),
    Error(String),
}

pub struct Log {
    events: Vec<LogEntry>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            events: vec![],
        }
    }

    pub fn add_info(&mut self, event: String) {
        self.events.push(LogEntry::Info(event));
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(LogEntry::Event(event));
    }

    pub fn add_error(&mut self, event: String) {
        self.events.push(LogEntry::Error(event));
    }

    pub fn print_log(&self) {
        for e in self.events.iter() {
            match e {
                LogEntry::Info(t) => println!("{}", t.green()),
                LogEntry::Event(t) => println!("{}", t.blue()),
                LogEntry::Error(t) => println!("{}", t.red()),
            }
        }
    }
}