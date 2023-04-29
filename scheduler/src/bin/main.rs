use scheduler::schedulers::{
    cyclic::CyclicScheduler,
    CheckSchedulable
};

fn print_is_schedulable(sched: &mut impl CheckSchedulable) {
    match sched.is_schedulable() {
        true => println!("El sistema SI es planificable"),
        false => println!("El sistema NO es planificable"),
    };
}

fn main() {
    let mut sched = CyclicScheduler::new();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(8.0, 40, 40).unwrap();

    print_is_schedulable(&mut sched);

}
