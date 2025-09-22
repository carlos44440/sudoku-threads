use crate::board::Board;

mod solver;
mod metrics;
mod board;
mod parallel;

fn main() {
    env_logger::init();


    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Uso: cargo run --release -- <path_to_16x16_board.txt>");
        println!("Formato: 16 filas, cada fila 16 valores separados por espacios o sin espacios. Use 0 o . para vacío, 1-9 y A-G para 10-16.");
        return;
    }


    let path = &args[1];


    match Board::from_file(path) {
        Ok(mut board) => {
            println!("Tablero cargado:");
            board.print();


            println!("Aplicando propagación de restricciones (REDUCE)...");
            if !board.reduce_constraints() {
                println!("Inconsistencia detectada durante la propagación de restricciones.");
                return;
            }


            println!("Tablero después de REDUCE:");
            board.print();


            println!("Intentando resolver con backtracking (secuencial)... Esto puede tardar en 16x16 difíciles");
            let start = std::time::Instant::now();
            if solver::solve(&mut board) {
                let dt = start.elapsed();
                println!("Solución encontrada en: {:?}", dt);
                board.print();
            } else {
                println!("No se encontró solución (o es demasiado costoso).");
            }
        }
        Err(e) => {
            println!("Error cargando tablero: {}", e);
        }
    }
}
