mod solver;
mod metrics;
mod board;
mod parallel;

use crate::board::Board;
use num_cpus;

fn main() {
    env_logger::init();

    let path = "./src/grid16x16.txt";

    // Variables para guardar tablero y tiempo secuencial
    let board: Board;
    let t_seq;

    // Cargar tablero
    match Board::from_file(path) {
        Ok(mut b) => {
            println!("Tablero cargado:");
            b.print();

            println!("Aplicando propagación de restricciones (REDUCE)...");
            if !b.reduce_constraints() {
                println!("Inconsistencia detectada durante la propagación de restricciones.");
                return;
            }

            println!("Tablero después de REDUCE:");
            b.print();

            // Resolver secuencialmente
            println!("Resolviendo con backtracking secuencial...");
            let (res, t) = metrics::measure_time(|| solver::solve(&mut b));
            t_seq = t;
            if res {
                println!("Solución secuencial encontrada en: {:?}", t_seq);
                b.print();
            } else {
                println!("No se encontró solución secuencialmente.");
            }
            board = b;
        }
        Err(e) => {
            println!("Error cargando tablero: {}", e);
            return;
        }
    };

    // Número máximo de cores disponibles
    let num_cores = num_cpus::get();
    println!("\nNúmero de cores disponibles: {}", num_cores);
    println!("Probando paralelización variando el número de hilos (1..=num_cores)\n");

    // Iterar sobre k = 1..=num_cores
    for k in 1..=num_cores {
        let (solution, t_par) = metrics::measure_parallel_with_threads(
            || parallel::solve_parallel(&board, k),
            k,
        );

        if let Some(sol) = solution {
            let speedup = metrics::parallel_speedup(t_seq, t_par);
            let efficiency = metrics::parallel_efficiency(t_seq, t_par, k);

            println!("=== {} hilos ===", k);
            println!("Tiempo paralelo: {:?}", t_par);
            println!("Speedup: {:.2}", speedup);
            println!("Eficiencia: {:.2}%", efficiency * 100.0);
            // Puedes imprimir la solución solo para k = num_cores si quieres
            if k == num_cores {
                println!("Solución final:");
                sol.print();
            }
        } else {
            println!("No se encontró solución usando {} hilos.", k);
        }
    }
}
