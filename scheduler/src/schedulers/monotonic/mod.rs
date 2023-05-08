pub use crate::{
    task::Task,
    log::Log,
    schedulers::CheckSchedulable,
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
trait CheckSC1 : GetTasks {
    fn check_sc1 (&mut self) -> (bool, Log) {
        // Càlculs
        let u_total: f64 = self.get_tasks().iter().map(|(_priority, task)| task.get_utilization()).sum();
        let n: f64 = self.get_tasks().len() as f64;
        let right_side = n*(((2f64).powf(1f64/n)) - 1f64);
        let result = u_total <= right_side;
        // Log
        let mut log = Log::new();
        log.add_info(format!("{u_total:.2} <= {right_side:.2} ??"));
        return (result, log);
    }
}

/**
 * Check the sufficient condition 2 over self.
 */
trait CheckSC2 : GetTasks {
    fn check_sc2(&mut self) -> (bool, Log) {
        // Càlculs
        let sc2: f64 = self.get_tasks().iter().map(|(_priority, task)| (task.get_utilization())+1.0).product();
        let result = sc2 <= 2.0;
        // Log
        let mut log = Log::new();
        log.add_info(format!("{sc2:.2} <= 2.0 ??"));
        return (result, log);
    }
}

/**
 * Check the response time analysis of self,
 * and return wether it succeded or not.
 */
trait CheckRTA : GetTasks {
    fn check_rta(&mut self) -> (bool, Log) {
        // Log local i temporal, que posteriorment serà aefgit al log de l'scheduler
        let mut log = Log::new();

        let mut prev_tasks: Vec<(f64,f64)> = vec![]; // Detalls de les tasks amb mes prioritat que la actual
        for (priority, task) in self.get_tasks() {
            log.add_event(format!("----- Prioritat de la Task a evaluar: {} -----", priority.unwrap()));
            let d = task.get_deadline();
            let mut w; // El W(n) actual
            let mut prev_ws: Vec<f64> = vec![]; // Els W calculats previament
            loop {
                // El calcul principal del RTA
                w = task.get_computing_time() +
                    prev_tasks.iter().map(|(t_period, t_computing_time)|
                                          ((prev_ws.last().unwrap_or(&0f64) / t_period).ceil()) * t_computing_time).sum::<f64>();

                log.add_info(format!(" W:{w:.2} <= D:{d:.2}??"));

                // La tasca mes prioritaria només necesita w <= d
                if (priority.unwrap_or(0) == self.get_tasks().len()) && (w <= d as f64) { // Es la primers tasca, no cal comprovar res més
                    prev_tasks.push((task.get_period() as f64, task.get_computing_time()));
                    log.append_to_last_entry(format!(" --> SI"));
                    break;
                } 

                if w > d as f64 { // El sistema no es planificable, no complim el RTA
                    log.add_error(format!("NO! W és més gran que D! RTA falla"));
                    return (false, log); 
                } else { log.append_to_last_entry(format!(" --> SI")); }

                if prev_ws.contains(&w) { // Ens ha sortit 2 cops el mateix valor, aquesta tasca compleix el RTA
                    prev_tasks.push((task.get_period() as f64, task.get_computing_time()));
                    log.append_to_last_entry(format!(" W apareix 2 cops, pasem a seguent task!"));
                    break;
                } 

                // Necesari per fer calculs a la seguent iteracio
                prev_ws.push(w);
            }
            // Si hem arribat aqui, la tasca actual compleix el RTA
        }
        // Si hem arribat aqui, totes les tasques comleixen el RTA i el sistema SI que es planificable
        log.add_event(format!("Totes les tasques compleixen el RTA!"));
        (true, log)
    }
}

trait GetUtilizationFactor : GetTasks {
    fn get_utilization(&self) -> f64 {
        self.get_tasks().iter().map(|(_priority, task)| task.get_utilization()).sum()
    }
}
