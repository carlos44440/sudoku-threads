# Sudoku Solver Paralelo en Rust

Este proyecto implementa un **solver de Sudoku 16x16** en Rust, capaz de resolver tableros utilizando **backtracking secuencial y paralelo**. Está diseñado para analizar el desempeño de la paralelización mediante métricas de **speedup** y **eficiencia** usando múltiples hilos.

## Objetivo

El objetivo del proyecto es:

- Resolver tableros de Sudoku de manera eficiente.
- Comparar el rendimiento entre el algoritmo secuencial y paralelo.
- Estudiar cómo la cantidad de hilos afecta el tiempo de resolución y la eficiencia.

## Funcionalidades

- Carga tableros de Sudoku desde archivos de texto.
- Aplica **propagación de restricciones** (reduce_constraints) antes del backtracking.
- Resuelve el Sudoku de manera secuencial.
- Resuelve el Sudoku de manera paralela usando `rayon` y un número configurable de hilos.
- Mide tiempos de ejecución, speedup y eficiencia para el algoritmo paralelo.

## Estructura del proyecto

- `src/main.rs`: Punto de entrada del programa.
- `src/board.rs`: Estructura del tablero y funciones auxiliares.
- `src/solver.rs`: Algoritmo de backtracking secuencial.
- `src/parallel.rs`: Algoritmo de backtracking paralelo.
- `src/metrics.rs`: Funciones para medir tiempos y calcular métricas.

## Requisitos

- Rust >= 1.70
- Crates utilizados:
  - `rayon` para paralelización.
  - `env_logger` para logging.
  - `num_cpus` para obtener el número de cores disponibles.

## Comandos para levantar el proyecto

1. **Clonar el repositorio**
   ```bash
   git clone <url-del-repo>
   cd <nombre-del-proyecto>
2. **Compilar y ejecutar en RustRover o desde terminal**
   ```bash
   cargo run --bin workshop1
