mod solver;
mod metrics;
mod board;
mod parallel;

use num_cpus;
use crate::board::Board;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Uso: cargo run --release -- <path_to_16x16_board.txt>");
        println!("Formato: 16 filas, cada fila 16 valores separados por espacios o sin espacios. Use 0 o . para vacío, 1-9 y A-G para 10-16.");
        return;
    }

    let path = &args[1];

    // Declarar variables fuera para que sean accesibles después
    let mut board: Board;
    let mut t_seq = std::time::Duration::ZERO;

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

            println!("Intentando resolver con backtracking (secuencial)...");
            let (res, t) = metrics::measure_time(|| solver::solve(&mut b));
            t_seq = t; // guardar tiempo secuencial
            if res {
                println!("Solución encontrada secuencialmente en: {:?}", t_seq);
                b.print();
            } else {
                println!("No se encontró solución secuencialmente.");
            }
            board = b; // guardar tablero para paralelización
        }
        Err(e) => {
            println!("Error cargando tablero: {}", e);
            return;
        }
    };

    println!("Intentando resolver en paralelo con Rayon...");
    let num_cores = num_cpus::get();

    let (solution, t_par) = metrics::measure_time(|| parallel::solve_parallel(&board));

    if let Some(sol) = solution {
        println!("Solución encontrada en paralelo en: {:?}", t_par);
        sol.print();

        // Calcular eficiencia usando t_seq
        let eff = metrics::parallel_efficiency(t_seq, t_par, num_cores);
        println!("Eficiencia paralela: {:.2}%", eff * 100.0);
    } else {
        println!("No se encontró solución en paralelo.");
    }
}
