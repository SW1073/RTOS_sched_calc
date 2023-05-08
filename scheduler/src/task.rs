/**
 * === Módul de tasca ===
 * En aquesta classe es defineixen
 * tots els paràmetres d'una tasca
 * genèrica.
 *
 */

/**
 * Data model d'una tasca en si
 */
#[derive(Debug, Clone)]
pub struct Task {
    computing_time: f64,
    deadline: usize,
    period: usize,
}

/**
 * Funcions de tasca
 */
impl Task {
    // ========== CREADORA ==========
    /**
     * Constructora default
     */
    pub fn new(computing_time: f64, deadline: usize, period: usize) -> Self {
        Task {
            computing_time,
            deadline,
            period,
        }
    }

    // ========== GETTERS ==========
    /**
     * Retorna el factor d'utilitzacio de la tasca
     */
    pub fn get_utilization(&self) -> f64 {
        self.computing_time/(self.period as f64)
    }

    /**
     * Retorna el temps de comput de la task
     */
    pub fn get_computing_time(&self) -> f64 {
        self.computing_time
    }

    /**
     * Retorna el deadline de la task
     */
    pub fn get_deadline(&self) -> usize {
        self.deadline
    }

    /**
     * Retorna el periode de la task
     */
    pub fn get_period(&self) -> usize {
        self.period
    }

    /**
     * Divideix la tasca en 2. Es modifica a si mateixa i en retorna la nova tasca, amb computing_time/2
     */
    pub fn divide_task(&mut self) -> Task {
        self.computing_time /= 2.0;
        self.clone()
    }
}
