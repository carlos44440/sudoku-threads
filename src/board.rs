use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub const SIZE: usize = 16; // Tamaño del tablero
pub const BLOCK: usize = 4; // Tamaño de los bloques

#[derive(Clone, Debug)] // Permite imprimir y clonar el grid.
pub struct Board {
    pub grid: [[u8; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self { grid: [[0u8; SIZE]; SIZE] }
    }

    /// Cargar tablero desde archivo. Cada línea puede tener valores separados por espacios,
    /// o bien un string con 16 caracteres (sin espacios). Use 0 o '.' para vacías.
    /// Valores permitidos: 1-9 y A-G / a-g (A->10 ... G->16).
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let mut board = Board::new();


        for (r, line_res) in reader.lines().enumerate() {
            if r >= SIZE { break; }
            let line = line_res?;
            // Separar los valores de la linea por espacios vacios
            let tokens: Vec<&str> = line.split_whitespace().collect();
            // Si la separacion fue exitosa rellenar el grid
            if tokens.len() == SIZE {
                for c in 0..SIZE {
                    board.grid[r][c] = parse_token(tokens[c])?;
                }
            // Si no fue exitosa intentar sin espacios
            } else {
                let chars: Vec<char> = line.chars().filter(|ch| !ch.is_whitespace()).collect();
                if chars.len() != SIZE {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          format!("Linea {}: esperaba {} valores, obtuvo {}", r+1, SIZE, chars.len())));
                }
                for c in 0..SIZE {
                    board.grid[r][c] = parse_char(chars[c])?;
                }
            }
        }


        Ok(board)
    }
    pub fn print(&self) {
        // Recorrer grid
        for r in 0..SIZE {
            for c in 0..SIZE {
                let v = self.grid[r][c];
                if v == 0 {
                    print!(" .");
                } else if v <= 9 {
                    print!(" {:}", v);
                } else {
                    // Transformar a char con el codigo ASCII
                    let ch = ((v - 10) as u8 + b'A') as char;
                    print!(" {}", ch);
                }
                // Imprimir separador de bloque de columna
                if (c + 1) % BLOCK == 0 && c + 1 != SIZE { print!(" |"); }
            }
            println!();
            // Imprimir separador de bloque de fila
            if (r + 1) % BLOCK == 0 && r + 1 != SIZE {
                for _ in 0..(SIZE + BLOCK - 1) { print!("--"); }
                println!();
            }
        }
    }

    /// Devuelve la primera celda vacía (row,col) o None si está completo
    pub fn find_empty(&self) -> Option<(usize, usize)> {
        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.grid[r][c] == 0 { return Some((r,c)); }
            }
        }
        None
    }

    /// Calcula la máscara de candidatos para la celda (row,col). Cada bit i representa el valor i+1.
    pub fn candidates_mask(&self, row: usize, col: usize) -> u16 {
        if self.grid[row][col] != 0 { return 0; }
        // Máscara que marca los numeros ya usados
        let mut used: u32 = 0;

        // Marcar los numeros ya usados en la fila
        for c in 0..SIZE {
            let v = self.grid[row][c];
            if v != 0 { used |= 1u32 << (v as u32 - 1); }
        }
        // Marcar los numeros ya usados en la columna
        for r in 0..SIZE {
            let v = self.grid[r][col];
            if v != 0 { used |= 1u32 << (v as u32 - 1); }
        }
        // Marcar los numeros ya usados en el bloque
        let br = (row / BLOCK) * BLOCK;
        let bc = (col / BLOCK) * BLOCK;
        for r in br..br+BLOCK {
            for c in bc..bc+BLOCK {
                let v = self.grid[r][c];
                if v != 0 { used |= 1u32 << (v as u32 - 1); }
            }
        }

        // Máscara que marca todos los numeros del 1 al 16
        let all_mask = (((1u32 << SIZE) - 1) & 0xffff) as u16;
        // Contenedor que marca los numeros ya usados en la fila, columna y bloque
        let used_mask = (used & 0xffff) as u16;

        // Máscara que marca los candidatos posibles para la celda
        all_mask & (!used_mask)
    }


    /// Convierte máscara a vector de valores 1..=SIZE
    pub fn mask_to_vec(mask: u16) -> Vec<u8> {
        let mut v = Vec::new();
        for i in 0..SIZE {
            if (mask & (1u16 << i)) != 0 {
                v.push((i + 1) as u8);
            }
        }
        v
    }


    /// Propagación simple de restricciones: asigna celdas que tienen un solo candidato repetidamente.
    /// Devuelve false si detecta inconsistencia (celda sin candidatos).
    pub fn reduce_constraints(&mut self) -> bool {
        loop {
            let mut changed = false;
            for r in 0..SIZE {
                for c in 0..SIZE {
                    // Busca los candidatos de cada celda vacia
                    if self.grid[r][c] == 0 {
                        let mask = self.candidates_mask(r, c);
                        if mask == 0 {
                            return false; // Retorna false, en caso de no encontrar candidatos.
                        }
                        // En caso de que haya un solo candidato, lo asigna a la celda.
                        if mask.count_ones() == 1 {
                            let val = Board::mask_to_vec(mask)[0];
                            self.grid[r][c] = val;
                            changed = true;
                        }
                    }
                }
            }
            if !changed { break; } // En caso de que no se realizaron cambios, termina.
            // En caso de que se realizaron cambios, sigue con el loop.
        }
        true
    }
}

// Parsea los tokens
fn parse_token(tok: &str) -> Result<u8, Error> {
    if tok == "." || tok == "0" { return Ok(0); }
    if tok.len() == 1 {
        return parse_char(tok.chars().next().unwrap());
    }
    // poder aceptar números >9 escritos como decimal (ej 10..16)
    if let Ok(n) = tok.parse::<u8>() {
        if n <= SIZE as u8 { return Ok(n); }
    }
    Err(Error::new(ErrorKind::InvalidData, format!("Token inválido: {}", tok)))
}

// Parsea los chars, transforma los char a digit.
fn parse_char(ch: char) -> Result<u8, Error> {
    if ch == '.' || ch == '0' { return Ok(0); }
    if ch.is_ascii_digit() {
        let v = ch.to_digit(10).unwrap() as u8;
        if v <= SIZE as u8 { return Ok(v); }
    }
    if ch.is_ascii_alphabetic() {
        let up = ch.to_ascii_uppercase();
        let idx = (up as u8).wrapping_sub(b'A');
        // A => 10, B => 11, ... G => 16
        if idx < 7 {
            return Ok(10 + idx as u8);
        }
    }
    Err(Error::new(ErrorKind::InvalidData, format!("Char inválido: {}", ch)))
}
