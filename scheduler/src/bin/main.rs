use scheduler::schedulers::{
    cyclic::CyclicScheduler,
    monotonic::{
        deadline::DeadlineMonotonicScheduler,
        rate::RateMonotonicScheduler,
    },
    SchedulabilityResult::{
        Schedulable as Schedulable,
        NotSchedulable as NotSchedulable,
        Undetermined as Undetermined,
    },
    CheckSchedulable
};

fn print_is_schedulable(sched: &mut dyn CheckSchedulable) {
    match sched.is_schedulable() {
        Schedulable => println!("El sistema SI es planificable"),
        NotSchedulable(reason) => println!("El sistema NO es planificable. Raó: {reason}"),
        Undetermined => println!("No s'ha pogut detrminar si el sistema es planificable.")
    };
}

fn main() {
    // Input Scheduler Type

    // Input Number of Tasks

    // Check schedulability
    let mut sched = CyclicScheduler::new();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(30.0, 40, 40).unwrap();

    print_is_schedulable(&mut sched);

}
