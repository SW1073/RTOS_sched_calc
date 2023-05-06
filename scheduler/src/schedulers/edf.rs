use itertools::Itertools;
use num::integer::lcm;
use crate::{
    log::Log,
    task::Task,
    schedulers::{
        SchedulabilityResult,
        CheckSchedulable,
        AddTaskCapabilities,
        SchedulerInterface,
    }
};

pub struct EarliestDeadlineFirstScheduler {
    tasks: Vec<Task>,
}

impl EarliestDeadlineFirstScheduler {
    /**
     * Constructora default
     */
    pub fn new() -> Self {
        EarliestDeadlineFirstScheduler {
            tasks: vec![],
        }
    }

    /**
     * Afegeix una nova tasca al planificador
     */
    pub fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        // Error checking
        if period < deadline { return Err(String::from("Period < Deadline")); }
        if computing_time < 0.0 { return Err(String::from("Computing Time < 0")) }

        self.tasks.push(Task::new(computing_time, deadline, period));
        Ok(())   
    }
 
    /**
     * Checks if all the tasks have same values for T and D. If one of them does not, the function returns false.
     */
    fn t_eq_d(&self) -> bool {
        let tasks_t_eq_d = self.tasks.iter().filter(|t| t.get_period() == t.get_deadline()).collect::<Vec<&Task>>().len();
        // Si la llargada del vector de tasques es igual al nombre de tasques on D=T, retornem true
        // Si en canvi, el nombre de tasques on D=T es menor que la llargada del vector original, retornem false
        tasks_t_eq_d == self.tasks.len()
    }

    /**
     * Retorna el factor d'utilització de totes les tasques del planificador
     */
    fn get_utilization(&self) -> f64 {
        self.tasks.iter().map(|t|t.get_utilization()).sum()
    }

    /**
     * Returns the hyperperiod
     */
    fn get_hyperperiod(&self) -> usize {
        let mut h = 1;
        for taski in self.tasks.iter() {
            h = lcm(h, taski.get_period());
        }
        return h;
    }

    /**
     * Checks if self scheduler meets the processor demand criterion.
     */
    fn check_pdc(&self) -> (bool, Log) {
        let mut log = Log::new();

        // Processor demand criterion: In any interval, 
        // the computation demanded by the task set must 
        // be no greater than the available time. 
        let u: f64 = self.get_utilization();
        let h = self.get_hyperperiod();
        let l_star: f64 = self.tasks.iter()
            .map(|t| (t.get_period() - t.get_deadline()) as f64 * t.get_utilization())
            .sum::<f64>() / (1_f64-u);
        let min_h_l_star = (h as f64).min(l_star);

        log.add_info(format!("Utilization Factor: {u:.2}"));
        log.add_info(format!("Hyperperiod: {h}"));
        log.add_info(format!("L*: {l_star:.2} "));
        if min_h_l_star == l_star { log.add_info(format!("L* <= Hyperperiod. Usem L*")); }
        else { log.add_info(format!("Hyperperiod <= L*. Usem Hyperperiod")); }

        // Form vector or all the absolute deadlines and the associated tasks.
        let mut abs_deadlines: Vec<usize> = vec![];
        for t in self.tasks.iter() {
            let mut l = t.get_deadline();
            while l <= min_h_l_star.round() as usize {
                abs_deadlines.push(l);
                l += t.get_period();
            }
        }
        abs_deadlines.sort();
        abs_deadlines = abs_deadlines.iter().unique().map(|e|*e).collect();
        log.add_info(format!("Els deadlines aboluts son: {abs_deadlines:?}"));

        // Check for every L if g(0,L) <= L
        for l in abs_deadlines {
            let mut g0l: f64 = 0.0;
            for tsk in self.tasks.iter() {
                g0l += ( (l + tsk.get_period() - tsk.get_deadline()) / tsk.get_period() ) as f64 * tsk.get_computing_time();
            }
            log.add_info(format!("g(0,{l}) = {g0l:.2} <= {l}??"));
            // Trobem si es compleix g(0,L) <= l
            if g0l > (l as f64) {
                log.add_error(format!("g(0,L) > L"));
                return (false, log);
            }
            log.append_to_last_entry(format!(" --> SI"));
        }

        // If the  previous check did not return, the pdc succeeded
        (true, log)
    }
}

impl CheckSchedulable for EarliestDeadlineFirstScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        // if ForAll Tasks Di = Ti, (Utilization <= 1.0) is sufficient
        // else if: ForAll Tasks Di < Ti, (PDC) is required
        let mut log = Log::new();

        if self.t_eq_d() { // ForAll Task_i: D_i=T_i
            log.add_event(format!("Totes les tasques tenen el periode igual al deadline"));
            log.add_info(format!("Només cal que el factor d'utilització sigui <= 1.0"));
            let u = self.get_utilization();
            log.add_info(format!("Utilization factor = {u}"));
            if self.get_utilization() > 1.0 {
                log.add_error(format!("U > 1.0!"));
                return SchedulabilityResult::NotSchedulable(Some(log));
            }
            log.add_event(format!("U <= 1.0. El sistema és planificable"));
            return SchedulabilityResult::Schedulable(Some(log));
        }
        else {
            log.add_event(format!("Hi ha tasques amb periode menor que deadline"));
            log.add_info(format!("Cal mirar el PDC"));

            let (result, log_pdc) = self.check_pdc();
            log.append_log(log_pdc);
            if result {
                log.add_event(format!("Es compleix el PDC. El sistema és planificable"));
                return  SchedulabilityResult::Schedulable(Some(log));
            }
            log.add_error(format!("No es compleix el PDC. No es pot garantir la planificabilitat d'aquest sistema"));
            return SchedulabilityResult::Undetermined(Some(log));
        }
    }
}

impl AddTaskCapabilities for EarliestDeadlineFirstScheduler {
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        // Error checking
        if period < deadline { return Err(String::from("Period < Deadline")); }
        if computing_time < 0.0 { return Err(String::from("Computing Time < 0")) }

        // Really adding the task
        self.tasks.push(Task::new(computing_time, deadline, period));
        Ok(()) 
    }
}

impl SchedulerInterface for EarliestDeadlineFirstScheduler {}
