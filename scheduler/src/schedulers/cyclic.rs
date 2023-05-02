use crate::SchedulabilityResult;
use crate::log::Log;
use crate::task::Task;
use super::{
    CheckSchedulable,
    AddTaskCapabilities,
    SchedulerInterface, 
    lcm,
};

#[derive(Debug)]
pub struct CyclicScheduler {
    tasks: Vec<Task>,
}

impl CyclicScheduler {

    /**
     * Constructora default
     */
    pub fn new() -> Self {
        CyclicScheduler {
            tasks: vec![],
        }
    }

    /**
     * Retorna el factor d'utilitzacio de l'scheduler
     */
    fn get_utilization(&self) -> f64 {
        self.tasks.iter().map(|t| t.get_utilization()).sum()
    }

    /**
     * Retorna l'hiperperiode de totes les tasks de l'scheduler
     */
    fn get_hyperperiod(&self) -> usize {
        let mut h = 1;
        for t in self.tasks.iter() {
            h = lcm(h, t.get_period());
        }
        return h;
    }

    /**
     * Retorna el temps de comput maxim entre totes les tasques
     */
    fn get_max_computing_time(&self) -> f64 {
        return self.tasks.iter().map(|t| t.get_computing_time()).max_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap_or(0.0);
    }

    /**
     * Retorna el deadline mes petit entre totes les tasques
     */
    fn get_min_deadline(&self) -> usize {
        return self.tasks.iter().map(|t| t.get_deadline()).min_by(|a, b| a.cmp(&b)).unwrap_or(0);
    }
}

/**
 * Implementació del common trait IsSchedulable, que conté la funció is_schedulable()
 */
impl CheckSchedulable for CyclicScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        // Log per a guardar els events que ocorren
        let mut log = Log::new();

        // Check utilization factor
        let u = self.get_utilization();
        log.add_info(format!("El factor d'utilitzacio U = {u:.2}"));
        if u > 1.0 {
            log.add_error(format!("U > 1.0"));
            return SchedulabilityResult::NotSchedulable(Some(log));
        };
        log.add_event(format!("Com que U <= 1.0, continuem mirant condicions"));

        // Find the hyperperiod
        let hyper_period = self.get_hyperperiod();
        log.add_info(format!("L'hiperperiode H = {hyper_period}"));

        // Secondary period
        log.add_event(format!("# A continuació, trobem el periode secundari:"));

        // Find max computing time
        let max_c = self.get_max_computing_time();
        log.add_info(format!("El temps de comput màxim és: {max_c}"));
        // Find min deadline time
        let min_d = self.get_min_deadline();
        log.add_info(format!("El deadline mínim és: {min_d}"));
        if max_c >= (min_d as f64) {
            log.add_event(format!("Com que el temps de comput màxim és major o igual que el mínim deadline, no es possible planificar aquest sistema actualment."));
            log.add_error(format!("Minimum Deadline <= Maximum Computing Time"));
            return SchedulabilityResult::NotSchedulable(Some(log)); // TODO: substitute by task partitioning
        }
        log.add_event(format!("Com que el temps de comput màxim és menor que el mínim deadline, és possible trobar frames secuandaris en el rang."));

        log.add_info(format!("De fet, gracies a la equivalencia H = k*Ts, sabem que:"));
        let kd = hyper_period/min_d;
        let kc = (hyper_period as f64)/max_c;
        log.add_info(format!("H = k*Ts = {kd:.2} * {min_d:.2} = {hyper_period}"));
        log.add_info(format!("H = k*Ts = {kc:.2} * {max_c:.2} = {hyper_period}"));

        // TODO: Cal trobar un layout de les tasques sobre el hyper_period i els secondary period.

        // return true if every check before was ok
        SchedulabilityResult::Schedulable(Some(log))
    }
    
}

impl AddTaskCapabilities for CyclicScheduler {
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        // Error checking
        if period < deadline { return Err(String::from("Period < Deadline")); }
        if computing_time < 0.0 { return Err(String::from("Computing Time < 0")) }

        // Really adding the task
        self.tasks.push(Task::new(computing_time, deadline, period));
        Ok(()) 
    }
}

impl SchedulerInterface for CyclicScheduler {}
