use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Mide el tiempo de ejecución de una función y retorna su duración
pub fn measure_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let dt = start.elapsed();
    (result, dt)
}

/// Calcula la eficiencia de paralelismo
/// `t_seq`: tiempo de ejecución secuencial
/// `t_par`: tiempo de ejecución paralelo
/// `num_cores`: número de cores usados
pub fn parallel_efficiency(t_seq: Duration, t_par: Duration, num_cores: usize) -> f64 {
    let t_seq_s = t_seq.as_secs_f64();
    let t_par_s = t_par.as_secs_f64();
    if t_par_s == 0.0 || num_cores == 0 {
        return 0.0;
    }
    let efficiency = t_seq_s / (t_par_s * num_cores as f64);
    efficiency.min(1.0) // máximo 1.0
}
