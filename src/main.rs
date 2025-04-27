use std::fs; // Solo necesitas fs para leer directorios y archivos
use std::io; // Para manejar los posibles errores de I/O

fn imprimir_contenido_archivos(directorio: &str) -> io::Result<()> {
    // Intentar abrir el directorio
    let entradas = fs::read_dir(directorio)?;

    // Iterar sobre las entradas del directorio
    for entrada in entradas {
        let entrada = entrada?; // Obtener la entrada
        let ruta = entrada.path(); // Obtener la ruta completa del archivo

        // Verificar si es un archivo (no una subcarpeta)
        if ruta.is_file() {
            println!("Contenido de archivo: {:?}", ruta);
            
            // Leer el contenido del archivo
            let contenido = fs::read_to_string(&ruta)?;
            println!("{}", contenido);  // Imprimir el contenido del archivo
        }
    }
    
    Ok(())
}

fn main() {
    let directorio = "./"; // Aquí puedes poner el path de tu directorio
    if let Err(e) = imprimir_contenido_archivos(directorio) {
        eprintln!("Error al leer los archivos: {}", e);
    }
}
