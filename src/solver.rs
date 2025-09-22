use crate::board::{Board, SIZE};

/// Solver secuencial por backtracking con heurística MRV (minimum remaining values).
pub fn solve(board: &mut Board) -> bool {
    // Primero, intentar propagar, en caso de que una celda esta vacia y no se encuentren
    // candidatos posibles (Tablero invalido), retorna false.
    if !board.reduce_constraints() { return false; }
    // Si no hay vacías, está resuelto
    if board.find_empty().is_none() { return true; }
    // MRV: elegir la celda vacía con menor número de candidatos
    let mut best: Option<(usize, usize, u16)> = None; // (r,c,mask)
    let mut best_count = (SIZE + 1) as u32;
    for r in 0..SIZE {
        for c in 0..SIZE {
            if board.grid[r][c] == 0 {
                let mask = board.candidates_mask(r, c);
                let cnt = mask.count_ones();
                if cnt == 0 { return false; } // inconsistencia
                if cnt < best_count {
                    best_count = cnt;
                    best = Some((r, c, mask));
                    if cnt == 1 { break; }
                }
            }
        }
    }
    let (r, c, mask) = match best {
        None => return true, // no vacías
        Some(t) => t,
    };
    // Probar cada candidato (orden ascendente es razonable)
    let candidates = Board::mask_to_vec(mask);
    for val in candidates {
        let mut next = board.clone();
        next.grid[r][c] = val;
        if solve(&mut next) {
            // copiar solución al tablero de entrada
            *board = next;
            return true;
        }
    }
    false
}
