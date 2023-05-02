// Exposem els diferents schedulers per a que puguin ser utilitzats fora del crate.
pub mod cyclic;
pub mod monotonic;
pub mod edf;

// Import global per a que tots els membres d'aquest modul puguin usar lcm i gcd.
pub use num::integer::{lcm, gcd};

// Import global per a que tots els membres d'aquest modul puguin usar els resultats d'schedulability
pub use crate::SchedulabilityResult;


// Traits que ens proporcionen la interfície del planificador

/**
 * Supertrait que agrupa tots els traits de la interfície
 */
pub trait SchedulerInterface: CheckSchedulable + AddTaskCapabilities {}

/**
 * Trait queassgura que qui l'implementi, te un mètode per a poder
 * comprovar la seva planificabilitat i per a afegir tasques.
 */
pub trait CheckSchedulable {
    fn is_schedulable(&mut self) -> SchedulabilityResult;
}

/**
 * Afegur una nova tasca al planificador. Sense implementació default.
 */
pub trait AddTaskCapabilities {
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String>;
}
