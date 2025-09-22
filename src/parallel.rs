use crate::board::{Board, SIZE};
use rayon::prelude::*;
use crate::solver;

/// Resolver Sudoku en paralelo: divide la búsqueda inicial en ramas distintas
/// y las explora en paralelo usando Rayon.
pub fn solve_parallel(board: &Board) -> Option<Board> {
    // Encontrar la primera celda vacía
    let (r, c) = match board.find_empty() {
        Some(pos) => pos,
        None => return Some(board.clone()), // ya está resuelto
    };

    // Obtener candidatos para esa celda
    let mask = board.candidates_mask(r, c);
    let candidates = Board::mask_to_vec(mask);

    // Probar cada candidato en paralelo
    candidates.into_par_iter().find_map_any(|val| {
        let mut next = board.clone();
        next.grid[r][c] = val;
        if solver::solve(&mut next) {
            Some(next)
        } else {
            None
        }
    })
}

/// Selecciona la celda vacía con menos candidatos (heurística MRV).
fn first_choice(board: &Board) -> Option<(usize, usize, u16)> {
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
