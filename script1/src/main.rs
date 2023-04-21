use num::integer::{lcm, gcd};

#[derive(Debug)]
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

    /**
     * Retorna el temps de comput de la task
     */
    fn get_computing_time(&self) -> f64 {
        self.computing_time
    }

    /**
     * Retorna el deadline de la task
     */
    fn get_deadline(&self) -> usize {
        self.deadline
    }

    /**
     * Retorna el periode de la task
     */
    fn get_period(&self) -> usize {
        self.period
    }

    /**
     * Retorna el factor d'utilitzacio de la tasca
     */
    fn get_utilization(&self) -> f64 {
        self.computing_time/(self.period as f64)
    }
}

#[derive(Debug)]
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

    /**
     * Retorna si la tasca es planificable o no
     */
    fn is_schedulable(&self) -> bool {
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
        println!("# A continuació, trobem el periode secondari:");
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

        let mult: usize = CyclicScheduler::get_multiplier(max_c);

        // ⩝𝑖: 2𝑇𝑠 − gcd(𝑇𝑠, 𝑇𝑖) ≤ 𝐷𝑖
        // With both values of 𝑇𝑠
        println!("# Trobem el frame secundari:");
        let tsd = min_d;
        let tsc = (max_c * mult as f64) as usize;
        let mut i = 1;
        for t in self.tasks.iter() {
            println!("Tasca numero {i}. ");
            if 2*tsd - gcd(tsd, t.get_period()) > t.get_deadline() {
                return false; 
            }
            if 2*tsc - gcd(tsc, t.get_period()*mult) > t.get_deadline()*mult {
                return false; 
            }
            i += 1;
        }

        // return true if every check before was ok
        true
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

    /**
     * Retorna multiple de 10 mes petit necesari per a que quan el multipliquem per f, el resulat
     * no tingui decimals.
     */
    fn get_multiplier(f: f64) -> usize {
        let mut mult = 1.0;
        while (f * mult).ceil() != (f * mult) {
            mult *= 10.0;
        }
        mult as usize
    }
}

fn main() {
    let mut cs = CyclicScheduler::new();
    cs.add_task(1.0, 10, 10).unwrap();
    cs.add_task(1.0, 10, 10).unwrap();
    cs.add_task(6.5, 10, 10).unwrap();
    dbg!(&cs);

    match cs.is_schedulable() {
        true => println!("El sistema es planificable"),
        false => println!("El sistema no es planificable"),
    }
}

// ==== [ TESTS ] ====
#[test]
fn general_test_1() {
    let mut my_tasks = CyclicScheduler::new();
    my_tasks.add_task(5.0, 20, 20).unwrap();
    my_tasks.add_task(5.0, 20, 20).unwrap();
    my_tasks.add_task(10.0, 40, 40).unwrap();
    assert!(my_tasks.is_schedulable());
}

#[test]
fn general_test_2() {
    let mut my_tasks = CyclicScheduler::new();
    my_tasks.add_task(2.0, 6, 6).unwrap();
    my_tasks.add_task(2.0, 8, 8).unwrap();
    my_tasks.add_task(8.0, 24, 24).unwrap();
    assert!(!my_tasks.is_schedulable());
}

#[test]
fn general_test_3() {
    let mut my_tasks = CyclicScheduler::new();
    my_tasks.add_task(2.0, 6, 6).unwrap();
    my_tasks.add_task(2.0, 8, 8).unwrap();
    my_tasks.add_task(4.0, 24, 24).unwrap();
    my_tasks.add_task(4.0, 24, 24).unwrap();
    assert!(!my_tasks.is_schedulable());
}

#[test]
fn general_test_4() {
    let mut my_tasks = CyclicScheduler::new();
    my_tasks.add_task(5.0,  20, 20).unwrap();
    my_tasks.add_task(10.0, 20, 20).unwrap();
    my_tasks.add_task(10.0, 40, 40).unwrap();
    assert!(my_tasks.is_schedulable());
}

