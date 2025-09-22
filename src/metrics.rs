use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Contador global de llamadas recursivas al solver
static CALLS: AtomicUsize = AtomicUsize::new(0);

/// Reinicia el contador de llamadas
pub fn reset_calls() {
    CALLS.store(0, Ordering::Relaxed);
}

/// Incrementa el contador de llamadas
pub fn inc_calls() {
    CALLS.fetch_add(1, Ordering::Relaxed);
}

/// Obtiene el número total de llamadas
pub fn get_calls() -> usize {
    CALLS.load(Ordering::Relaxed)
}

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
