use scheduler::{
    schedulers::{
        // Schedulers available
        cyclic::CyclicScheduler,
        monotonic::{
            deadline::DeadlineMonotonicScheduler,
            rate::RateMonotonicScheduler,
        },
        edf::EarliestDeadlineFirstScheduler,

        // Results the schedulers is_planable() can return
        SchedulabilityResult::{
            Schedulable as Schedulable,
            NotSchedulable as NotSchedulable,
            Undetermined as Undetermined,
        },

        // Interface que ens asegura que un scheduler implementa is_schedulable() i add_task()
        SchedulerInterface,
    },

    // Funcionalitats per a manegar el logs
    log::Log,
};
// Per a fer input fàcil
use input_macro::input;

// Per a fer exit(-1) quan hi hagi un error.
use std::process::exit;

/**
 * Imprimeix el log en cas que hi hagi
 */
fn print_if_log(log: Option<Log>) {
    match log {
        Some(l) => {println!("Log:"); l.print_log()},
        None => println!("No s'ha proporcionat Log."),
    }
}

fn main() {
    // Input Scheduler Type
    println!("Schedulers disponibles:");
    println!("1. C   -> Cíclic");
    println!("2. RM  -> Rate Monotnic");
    println!("3. DM  -> Deadline Monotonic");
    println!("4. EDF -> Eadrliest Deadline First");
    let sched_type = input!("Quin scheduler vols utilitzar?\n").to_lowercase();
    let mut sched = match sched_type.trim() {
        "1" | "c" => Box::new(CyclicScheduler::new()) as Box<dyn SchedulerInterface>,
        "2" | "rm" => Box::new(RateMonotonicScheduler::new()) as Box<dyn SchedulerInterface>,
        "3" | "dm" => Box::new(DeadlineMonotonicScheduler::new()) as Box<dyn SchedulerInterface>,
        "4" | "edf" => Box::new(EarliestDeadlineFirstScheduler::new()) as Box<dyn SchedulerInterface>,
        _ => {println!("Scheduler invàlid"); exit(-1);},
    };

    // Input Number of Tasks
    let num_tasks = input!("Quantes tasques vols afegir a l'scheduler?\n").parse::<usize>().unwrap();

    // Input and add such tasks
    let mut computing_time: f64;
    let mut deadline: usize;
    let mut period: usize;
    let mut input_line: String;
    println!("Introdueix el computing_time, deadline i periode separats per espais per a cada tasca a continuació:");
    for _n in 0..num_tasks {
        println!("-- Tasca {_n} --");
        input_line = input!("-> ");
        let input_line_split: Vec<&str> = input_line.split(' ').collect();
        computing_time = input_line_split[0].parse::<f64>().unwrap();
        deadline = input_line_split[1].parse::<usize>().unwrap();
        period = input_line_split[2].parse::<usize>().unwrap();
        sched.add_task(computing_time, deadline, period).unwrap();
    }

    // Check schedulability and print results
    match sched.is_schedulable() {
        Schedulable(log) => {
            println!("El sistema SI es planificable");
            print_if_log(log);
        },
        NotSchedulable(log) => {
            println!("El sistema NO es planificable.");
            print_if_log(log);
        },
        Undetermined(log) => {
            println!("No s'ha pogut detrminar si el sistema es planificable.");
            print_if_log(log);
        }
    };

}
