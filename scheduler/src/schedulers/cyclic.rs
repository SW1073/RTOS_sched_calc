use crate::task::Task;
use super::CheckSchedulable;
use super::lcm;

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
     * Afegeix una nova tasca al planificador
     */
    pub fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        self.tasks.push(Task::new(computing_time, deadline, period));
        Ok(())   
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
    fn is_schedulable(&mut self) -> bool {
        // Check utilization factor
        println!("# Busquem el factor d'utilitzacio");
        let u = self.get_utilization();
        println!("El factor d'utilitzacio U = {u}");
        if u > 1.0 {
            println!("! Com que U > 1.0, el sistema no es planificable.");
            return false
        };
        println!("Com que U <= 1.0, continuem mirant condicions.");

        // Find the hyperperiod
        let hyper_period = self.get_hyperperiod();
        println!("# L'hiperperiode H = {hyper_period}");

        // Secondary period
        println!("# A continuació, trobem el periode secundari:");
        // Find max computing time
        let max_c = self.get_max_computing_time();
        println!("El temps de comput màxim és: {max_c}");
        // Find min deadline time
        let min_d = self.get_min_deadline();
        println!("El deadline mínim és: {min_d}");
        if max_c >= (min_d as f64) {
            println!("! Com que el temps de comput màxim és major o igual que el mínim deadline, no es possible planificar aquest sistema actualment.");
            return false;
        }
        println!("Com que el temps de comput màxim és menor que el mínim deadline, és possible trobar frames secuandaris en el rang.");

        println!("De fet, gracies a la equivalencia H = k*Ts, sabem que:");
        let kd = hyper_period/min_d;
        let kc = (hyper_period as f64)/max_c;
        println!("H = k*Ts = {kd} * {min_d} = {hyper_period}");
        println!("H = k*Ts = {kc} * {max_c} = {hyper_period}");

        // TODO: Cal trobar un layout de les tasques sobre el hyper_period i els secondary period.

        // return true if every check before was ok
        true
    }
    
}

