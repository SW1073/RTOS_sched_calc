// Log per a manegar els logs que pugui crear la execució del programa,
// i treure'ls per pantalla quan l'usuari vulgui. En substitució del println!()

use colored::Colorize;

#[derive(Debug, Clone)]
enum LogEntry {
    Info(String),
    Event(String),
    Error(String),
}

#[derive(Debug, Clone)]
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
    
    pub fn append_to_last_entry(&mut self, addition: String) {
        let last_entry_mut_ref = self.events.last_mut().unwrap();
        match last_entry_mut_ref {
            LogEntry::Info(c) => { *last_entry_mut_ref = LogEntry::Info(format!("{c}{addition}"))},
            LogEntry::Event(c) => { *last_entry_mut_ref = LogEntry::Event(format!("{c}{addition}"))},
            LogEntry::Error(c) => { *last_entry_mut_ref = LogEntry::Error(format!("{c}{addition}"))},
        }
    }

    pub fn append_log(&mut self, log_to_append: Log) {
        self.events.append(&mut log_to_append.get_events().clone());
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

    fn get_events(&self) -> &Vec<LogEntry> {
        &self.events
    }
}