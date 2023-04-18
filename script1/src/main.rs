struct Task {
    computing_time: f64,
    deadline: usize,
    period: usize,
}

impl Task {

    /**
     * Constructora default
     */
    fn new(computing_time: f64, deadline: usize, period: usize) -> Self {
        Task {
            computing_time,
            deadline,
            period,
        }
    }
}

struct CyclicScheduler {
    tasks: Vec<Task>,
}

impl CyclicScheduler {

    /**
     * Constructora default
     */
    fn new() -> Self {
        CyclicScheduler {
            tasks: vec![],
        }
    }

    /**
     * Afegeix una nova tasca al planificador
     */
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        self.tasks.push(Task::new(computing_time, deadline, period));
        Ok(())   
    }
}

fn main() {
    let mut cs = CyclicScheduler::new();
    cs.add_task(1.0, 10, 10).unwrap();
}
