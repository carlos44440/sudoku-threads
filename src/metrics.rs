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

/// Calcula el speedup de paralelización
/// `t_seq`: tiempo de ejecución secuencial
/// `t_par`: tiempo de ejecución paralelo
pub fn parallel_speedup(t_seq: Duration, t_par: Duration) -> f64 {
    let t_seq_s = t_seq.as_secs_f64();
    let t_par_s = t_par.as_secs_f64();
    if t_par_s == 0.0 {
        return 0.0;
    }
    t_seq_s / t_par_s
}

/// Ejecuta una función paralela usando `k` threads y mide su duración
/// `f`: función que ejecuta el algoritmo paralelo
/// `num_threads`: número de threads a usar
/// Retorna (resultado de la función, tiempo de ejecución)

pub fn measure_parallel_with_threads<F, R>(f: F, num_threads: usize) -> (R, Duration)
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    use rayon::ThreadPoolBuilder;

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("No se pudo crear el thread pool");

    pool.install(|| measure_time(f))
}
