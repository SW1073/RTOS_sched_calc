use scheduler::task::Task;

#[derive(Debug)]
enum SchedulerType {
    RateMonotonic,
    DeadlineMonotonic,
}

#[derive(Debug)]
struct MonotonicScheduler {
    // With associated priority
    tasks: Vec<(Option<usize>,Task)>,
    shced_type : SchedulerType,
}

impl MonotonicScheduler {

    /**
     * Constructora default
     */
    fn new(shced_type: SchedulerType) -> Self {
        MonotonicScheduler {
            tasks: vec![],
            shced_type,
        }
    }

    /**
     * Afegeix una nova tasca al planificador
     */
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        self.tasks.push((None, Task::new(computing_time, deadline, period)));
        Ok(())   
    }

    /**
     * Retorna si el planificador es plausible i planificable
     */
    fn is_planable(&mut self) -> bool {
        self.equal_multipliers();
        self.assign_priorities();

        let sc1 = self.check_sc1();
        let sc2 = self.check_sc2();
        let rta = self.check_rta();

        return sc1 || sc2 || rta;
    }

    /**
     * Iguala els multiplicadors de totes les tasques del sistema
     */
    fn equal_multipliers(&mut self) {
        let max = self.tasks.iter().map(|a| a.1.get_multiplier()).max().unwrap_or(0);
        for t in self.tasks.iter_mut() {
            t.1.set_multiplier(max);
        }
    }

    /**
     * Assigna les prioritats corresponents segons el tipus de planificador
     */
    fn assign_priorities(&mut self) {
        // Sort task by priority depending on scheduler type
        match self.shced_type {
            SchedulerType::RateMonotonic => self.tasks.sort_by(|a,b| a.1.get_period().cmp(&b.1.get_period())),
            SchedulerType::DeadlineMonotonic => self.tasks.sort_by(|a,b| a.1.get_deadline().cmp(&b.1.get_deadline())),
        };

        // Set priorities based on their position in the ordered vector
        let mut i = self.tasks.len();
        for t in self.tasks.iter_mut() {
            t.0 = Some(i);
            i-=1;
        }
    }

    /**
     * Check the sufficient condition 1 over self.
     */
    fn check_sc1 (&self) -> bool {
        let u_total: f64 = self.tasks.iter().map(|t|t.1.get_utilization()).sum();
        let n: f64 = self.tasks.len() as f64;
        return u_total <=  n*(((2f64).powf(1f64/n)) - 1f64);
    }

    /**
     * Check the sufficient condition 2 over self.
     */
    fn check_sc2(&self) -> bool {
        let sc2: f64 = self.tasks.iter().map(|t|(t.1.get_utilization())+1.0).product();
        return sc2 <= 2.0;
    }

    /**
     * Check the response time analysis of self,
     * and return wether it succeded or not.
     */
    fn check_rta(&self) -> bool {
        println!("===== RTA check =====");
        let mut prev_tasks: Vec<(usize,usize)> = vec![]; // Detalls de les tasks amb mes prioritat que la actual
        for t in self.tasks.iter() {
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
                if (t.0.unwrap_or(0) == self.tasks.len()) && (w <= d) {
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

fn main() {
    let mut sched = MonotonicScheduler::new(SchedulerType::RateMonotonic);
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();

    println!("{}", sched.is_planable());
    
}
