extern crate termion;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, stdin, Stdout, Stdin, Write};

pub struct Terminal {
    pub stdout: Stdout,
    pub stdin: Stdin,
}

pub fn init() -> Terminal{
    let mut term = Terminal {
        stdout: stdout(),
        stdin: stdin()
    };
    // Reseteamos el cursor
    let _ = write!(term.stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1));
    return term;
}

pub fn prompt(term: &mut Terminal) -> Result<String, String> {
    let stdout = &mut term.stdout;
    let stdin = &mut term.stdin;

    let _ = write!(stdout, "\n > ");
    let _ = stdout.flush(); // Siempre pensamos que el flush funcionará bien.
    let mut string = String::new(); // Este String guardará el comando enviado
    if stdin.read_line(&mut string).is_err() {
        return Err(String::from("Could not read line"));
    }

    return Ok(string);
}

pub fn slow_type(string: &str) {
    // Primero, preparamos el stdout
    let mut stdout = stdout();
    // Después imprimiremos en pantalla, caracter por caracter
    for character in string.chars() {
        // Imprimimos en pantalla, ignoramos el resultado
        let _ = write!(stdout, "{}", character);
        // Hacemos flush, para mostrar el resultado de la impresión
        let _ = stdout.flush();
        // Y ejecutamos la parada del proceso
        sleep(Duration::from_millis(10)); // TODO 5 hardcoded, mejor variable del sistema
    }
}