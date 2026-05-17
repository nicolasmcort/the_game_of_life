use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let width = 50;
    let height = 50;
    let padding_width = width + 2;
    let size = padding_width * (height + 2);

    // Inicializamos dos buffers con ceros
    // vec![valor; tamaño] es como un malloc + memset
    let mut grid_a = vec![0u8; size];
    let mut grid_b = vec![0u8; size];

    // Semilla aleatoria con baja densidad (~15% de celdas vivas)
    let mut rng = rand::thread_rng();
    for y in 1..=height {
        for x in 1..=width {
            if rng.gen_bool(0.15) {
                grid_a[y * padding_width + x] = 1;
            }
        }
    }

    let stdin = io::stdin();
    let mut generacion = 0;

    // Bucle de generaciones
    loop {
        // Imprimir el grid en consola
        print_grid(width, height, &grid_a);
        println!("--- Generación {} ---", generacion);

        // Calcular la lógica de la siguiente generación y obtener estadísticas del estado actual
        // Pasamos grid_a como lectura y grid_b como escritura mutua
        let (poblacion, nacimientos, muertes) = update_grid(width, height, &grid_a, &mut grid_b);

        // Imprimir métricas de la simulación inmediatamente abajo del tablero
        println!("--------------------------------------------------");
        println!("Población Actual: {} celdas vivas", poblacion);
        println!("Nacimientos en este turno: {} | Muertes: {}", nacimientos, muertes);
        println!("--------------------------------------------------");

        // Preguntar al usuario si quiere continuar (ahora viendo todos los datos en pantalla)
        print!("Presiona Enter para continuar a la siguiente generación, o escribe 'q' para salir: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("q") {
            println!("Simulación terminada por el usuario.");
            break;
        }

        // "Swap" de memoria. Rust intercambia los dueños de los datos
        std::mem::swap(&mut grid_a, &mut grid_b);

        generacion += 1;

        // Control de velocidad de la simulación.
        // 100 milisegundos por generación. 
        thread::sleep(Duration::from_millis(100));
    }
}


fn print_grid(w: usize, h: usize, grid: &[u8]) {
    let p_w = w + 2;
    for y in 1..=h {
        for x in 1..=w {
            let idx = y * p_w + x;
            if grid[idx] == 1 {
                print!("█");
            } else {
                print!("·");
            }
        }
        println!();
    }
}


// Modificado únicamente para contar y retornar (Población_Total, Nacimientos, Muertes)
fn update_grid(w: usize, h: usize, actual: &[u8], siguiente: &mut [u8]) -> (u32, u32, u32) {
    let p_w = w + 2; // Ancho con padding
    
    let mut total_vivas = 0;
    let mut nacimientos = 0;
    let mut muertes = 0;

    for y in 1..=h {
        for x in 1..=w {
            let idx = y * p_w + x;

            // Suma de los 8 vecinos (gracias al padding no hay desbordamiento)
            let vecinos = actual[idx - p_w - 1] + actual[idx - p_w] + actual[idx - p_w + 1] +
                          actual[idx - 1]         +             actual[idx + 1] +
                          actual[idx + p_w - 1] + actual[idx + p_w] + actual[idx + p_w + 1];

            // Reglas de Conway
            siguiente[idx] = match (actual[idx], vecinos) {
                (1, 2) | (1, 3) => {
                    total_vivas += 1;
                    1 // Vive
                },
                (0, 3) => {
                    total_vivas += 1;
                    nacimientos += 1;
                    1 // Nace
                },
                (1, _) => {
                    muertes += 1;
                    0 // Muere por sobrepoblación o soledad
                },
                _ => 0, // Sigue muerto
            };
        }
    }

    // Retorna las estadísticas calculadas en este turno
    (total_vivas, nacimientos, muertes)
}