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
#[derive(Debug)]
pub struct Task {
    computing_time: f64,
    deadline: usize,
    period: usize,
    multiplier: f64,
}

/**
 * Constant que defineix la maxima precisio que retenim quan cambiem la base del temps.
 * Preferiblement, ha de ser una potència de 10.
 */
const MAX_MULT: f64 = 10000_f64;

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
            multiplier: utils::find_mult(computing_time),
        }
    }

    // ========== INTERACCIO AMB EL MULTIPLIER ==========
    /**
     * Retorna el factor multiplicador de la tasca
     */
    pub fn get_multiplier(&self) -> usize {
        self.multiplier as usize
    }

    pub fn set_multiplier(&mut self, mult: usize) {
        self.multiplier = mult as f64;
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

    // ========== GETTERS AMB MULTIPLIER ==========
    /**
     * Retorna el temps de comput de la task * multiplier
     */
    pub fn get_computing_time_mult(&self) -> usize {
        (self.computing_time * self.multiplier) as usize
    }

    /**
     * Retorna el deadline de la task * multiplier
     */
    pub fn get_deadline_mult(&self) -> usize {
        self.deadline * self.multiplier as usize
    }

    /**
     * Retorna el periode de la task * multiplier
     */
    pub fn get_period_mult(&self) -> usize {
        self.period * self.multiplier as usize
    }

}

/**
 * Módul de funcions utilitaries
 */
mod utils {
    use super::MAX_MULT;
    // ========== FUNCIONS PRIVADES ==========
    /**
     * Troba el mínim multiplier de base 10 pel que f deixa de tenir
     * decimals. El multiplier tindrà valor 1.0 <= m <= MAX_MULT.
     */
    pub fn find_mult(f: f64) -> f64 {
        let mut mult = 1.0;

        while ((f * mult).ceil() != (f * mult)) && (mult < MAX_MULT) {
            mult *= 10.0;
        }

        mult
    }
}
