use crate::board::{Board};
use crate::solver;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::{Arc};

/// Resolver Sudoku en paralelo usando k hilos
/// `num_threads` controla la cantidad de hilos a usar
pub fn solve_parallel(board: &Board, num_threads: usize) -> Option<Board> {
    let board = Arc::new(board.clone());

    // Crear thread pool con k hilos
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("No se pudo crear el thread pool");

    // Ejecutar solución dentro del pool
    pool.install(|| {
        // Encontrar la primera celda vacía
        let (r, c) = match board.find_empty() {
            Some(pos) => pos,
            None => return Some((*board).clone()), // ya resuelto
        };

        // Obtener candidatos para esa celda
        let mask = board.candidates_mask(r, c);
        let candidates = Board::mask_to_vec(mask);

        // Explorar cada candidato en paralelo
        candidates.into_par_iter().find_map_any(|val| {
            let mut next = (*board).clone();
            next.grid[r][c] = val;
            if !next.reduce_constraints() {  // aplicar propagación antes de backtracking
                return None;
            }
            if solver::solve(&mut next) {
                Some(next)
            } else {
                None
            }
        })
    })
}
