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
            if solver::solve(&mut next) {
                Some(next)
            } else {
                None
            }
        })
    })
}

/*
/// Selecciona la celda vacía con menos candidatos (heurística MRV).
pub fn first_choice(board: &Board) -> Option<(usize, usize, u16)> {
    let mut best: Option<(usize, usize, u16)> = None;
    let mut best_count = (SIZE + 1) as u32;

    for r in 0..SIZE {
        for c in 0..SIZE {
            if board.grid[r][c] == 0 {
                let mask = board.candidates_mask(r, c);
                let cnt = mask.count_ones();
                if cnt == 0 {
                    return None; // inconsistencia
                }
                if cnt < best_count {
                    best_count = cnt;
                    best = Some((r, c, mask));
                    if cnt == 1 {
                        return best; // mejor caso
                    }
                }
            }
        }
    }
    best
}
*/
