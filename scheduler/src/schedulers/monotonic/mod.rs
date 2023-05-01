pub use super::CheckSchedulable;
pub use crate::{
    task::Task,
    log::Log,
};

// Exportem els moduls necesaris
pub mod deadline;
pub mod rate;

trait GetTasksMut {
    fn get_tasks_mut(&mut self) -> std::slice::IterMut<'_, (Option<usize>, Task)>;
}

trait GetTasks {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)>;
}

trait LogFunctionalities {
    fn log_append(&mut self, log_to_append: Log);
}

/**
 * Iguala els multiplicadors de totes les tasques del sistema
 */
trait EqualMultipliers : GetTasksMut {
    fn equal_multipliers(&mut self) {
        let max = self.get_tasks_mut().map(|a| a.1.get_multiplier()).max().unwrap_or(0);
        for t in self.get_tasks_mut() {
            t.1.set_multiplier(max);
        }
    }
}

/**
 * Asigna prioritats a totes les tasques del sistema
 */
trait AssignPriorities : GetTasksMut + GetTasks {
    fn sort_n_assign(&mut self) {
        self.sort_tasks();
        self.assign_priorities();
    }

    fn assign_priorities(&mut self) {
        // Set priorities based on their position in the ordered vector
        let mut i = self.get_tasks().len();
        for t in self.get_tasks_mut() {
            t.0 = Some(i);
            i-=1;
        }
    }

    // Dependent on every implementation
    fn sort_tasks(&mut self);
}

/**
 * Check the sufficient condition 1.
 */
trait CheckSC1 : GetTasks + LogFunctionalities {
    fn check_sc1 (&mut self) -> bool {
        let mut log = Log::new();
        let u_total: f64 = self.get_tasks().iter().map(|t|t.1.get_utilization()).sum();
        let n: f64 = self.get_tasks().len() as f64;
        let right_side = n*(((2f64).powf(1f64/n)) - 1f64);
        log.add_info(format!("{u_total:.2} <= {right_side:.2}??"));
        self.log_append(log);
        return u_total <= right_side;
    }
}

/**
 * Check the sufficient condition 2 over self.
 */
trait CheckSC2 : GetTasks + LogFunctionalities {
    fn check_sc2(&mut self) -> bool {
        let sc2: f64 = self.get_tasks().iter().map(|t|(t.1.get_utilization())+1.0).product();
        let mut log = Log::new();
        log.add_info(format!("{sc2:.2} <= 2.0 ??"));
        self.log_append(log);
        return sc2 <= 2.0;
    }
}

/**
 * Check the response time analysis of self,
 * and return wether it succeded or not.
 */
trait CheckRTA : GetTasks + LogFunctionalities {
    fn check_rta(&mut self) -> bool {
        // Log local i temporal, que posteriorment serà aefgit al log de l'scheduler
        let mut log = Log::new();

        let mut prev_tasks: Vec<(usize,usize)> = vec![]; // Detalls de les tasks amb mes prioritat que la actual
        for t in self.get_tasks() {
            log.add_event(format!("----- Task a evaluar: {} -----", t.0.unwrap()));
            let d = t.1.get_deadline_mult();
            let mut w; // El W(n) actual
            let mut prev_w = 0; // El W(n-1)
            let mut prev_ws: Vec<usize> = vec![]; // Els W calculats previament
            loop {
                // El calcul principal del RTA
                w = t.1.get_computing_time_mult() + prev_tasks.iter().map(|tsk|((prev_w/tsk.0)+(if prev_w%tsk.0 == 0{0}else{1}))*tsk.1).sum::<usize>();
                log.add_info(format!("--- Current w: {} --- ", w));
                // La tasca mes prioritaria només necesita w <= d
                if (t.0.unwrap_or(0) == self.get_tasks().len()) && (w <= d) { // Es la primers tasca, no cal comprovar res més
                    prev_tasks.push((t.1.get_period_mult(), t.1.get_computing_time_mult()));
                    break;
                } 

                log.add_info(format!(" W:{w} <= D:{d}?"));
                if w > d { // El sistema no es planificable, no complim el RTA
                    log.add_error(format!("NO! W és més gran que D! RTA falla"));
                    self.log_append(log);
                    return false; 
                } else { log.append_to_last_entry(format!(" SI")) }

                if prev_ws.contains(&w) { // Ens ha sortit 2 cops el mateix valor, aquesta tasca compleix el RTA
                    prev_tasks.push((t.1.get_period_mult(), t.1.get_computing_time_mult()));
                    log.append_to_last_entry(format!(" W apareix 2 cops, pasem a seguent task!"));
                    break;
                } 

                // Necesari per fer calculs a la seguent iteracio
                prev_w = w;
                prev_ws.push(w);
            }
            // Si hem arribat aqui, la tasca actual compleix el RTA
        }
        // Si hem arribat aqui, totes les tasques comleixen el RTA i el sistema SI que es planificable
        log.add_event(format!("Totes les tasques compleixen el RTA!"));
        self.log_append(log);
        true
    }
}
