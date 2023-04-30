pub use super::CheckSchedulable;
pub use crate::task::Task;

// Exportem els crates necesaris
pub mod deadline;
pub mod rate;

trait GetTasksMut {
    fn get_tasks_mut(&mut self) -> std::slice::IterMut<'_, (Option<usize>, Task)>;
}

trait GetTasks {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)>;
}

/**
 * Iguala els multiplicadors de totes les tasques del sistema
 */
trait EqualMultipliers : GetTasksMut{
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
trait AssignPriorities {
    fn assign_priorities(&mut self); // No default implementation
}

/**
 * Check the sufficient condition 1.
 */
trait CheckSC1 : GetTasks {
    fn check_sc1 (&mut self) -> bool {
        let u_total: f64 = self.get_tasks().iter().map(|t|t.1.get_utilization()).sum();
        let n: f64 = self.get_tasks().len() as f64;
        return u_total <=  n*(((2f64).powf(1f64/n)) - 1f64);
    }
}

/**
 * Check the sufficient condition 2 over self.
 */
trait CheckSC2 : GetTasks {
    fn check_sc2(&self) -> bool {
        let sc2: f64 = self.get_tasks().iter().map(|t|(t.1.get_utilization())+1.0).product();
        return sc2 <= 2.0;
    }
}

/**
 * Check the response time analysis of self,
 * and return wether it succeded or not.
 */
trait CheckRTA : GetTasks {
    fn check_rta(&self) -> bool {
        println!("===== RTA check =====");
        let mut prev_tasks: Vec<(usize,usize)> = vec![]; // Detalls de les tasks amb mes prioritat que la actual
        for t in self.get_tasks().iter() {
            println!("----- Task a evaluar: {} -----", t.0.unwrap());
            let d = t.1.get_deadline_mult();
            let mut w; // El W(n) actual
            let mut prev_w = 0; // El W(n-1)
            let mut prev_ws: Vec<usize> = vec![]; // Els W calculats previament
            loop {
                // El calcul principal del RTA
                w = t.1.get_computing_time_mult() + prev_tasks.iter().map(|tsk|((prev_w/tsk.0)+(if prev_w%tsk.0 == 0{0}else{1}))*tsk.1).sum::<usize>();
                print!("Current w: {} | ", w);
                // La tasca mes prioritaria només necesita w <= d
                if (t.0.unwrap_or(0) == self.get_tasks().len()) && (w <= d) {
                    prev_tasks.push((t.1.get_period_mult(), t.1.get_computing_time_mult()));
                    println!();
                    break;
                } 

                print!("W:{w} <= D:{d}? --> ");
                // El sistema no es planificable, no complim el RTA
                if w > d {
                    println!("NO! W és més gran D! RTA falla");
                    return false; 
                } else { print!("SI "); }

                // Ens ha sortit 2 cops el mateix valor, aquesta tasca compleix el RTA
                if prev_ws.contains(&w) {
                    prev_tasks.push((t.1.get_period_mult(), t.1.get_computing_time_mult()));
                    println!("W apareix 2 cops, pasem a seguent task!");
                    break;
                } 
                // Necesari per fer calculs a la seguent iteracio
                prev_w = w;
                prev_ws.push(w);

                println!();
            }
            // Si hem arribat aqui, la tasca actual compleix el RTA
        }
        // Si hem arribat aqui, totes les tasques comleixen el RTA i el sistema SI que es planificable
        println!("Totes les tasques compleixen el RTA!!");
        true
    }
}
