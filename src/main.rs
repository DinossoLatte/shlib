extern crate termion;

pub mod graphics;
pub mod command;

pub fn interpreter(commands: command::Commands<command::Command>) {
    let mut term = graphics::init();
    loop {
        let string_res = graphics::prompt(&mut term);
        if (&string_res).is_err() {
            panic!("Error attempting to get line, {}", string_res.unwrap_err());
        } else {
            // Extraemos el resultado, ya que su tiempo de vida se perderá
            let string = string_res.unwrap();
            // Eliminamos caracteres innecesarios
            let mut args_str = string.trim().split(" ");
            // Obtenemos el comando o "", que seguramente no tenga asignado ningún comando
            // Además, al llamar a nth extraemos el primer elemento del comando del iterador
            let command = args_str.nth(0).unwrap_or("");
            // Convertimos los argumentos a String, para guardarlos en el heap
            let args = args_str.map(|str| String::from(str)).collect();
            // Ahora necesitamos extraer los argumentos del comando)
            if command == "exit" {
                break;
            } else {
                let res = commands.execute_command(String::from(command), args);
                match res {
                    Ok(text) => {
                        graphics::slow_type(&text);
                    },
                    Err(text) => {
                        graphics::slow_type(&text);
                    }
                }
            }
        }
    }
}

// Se trata de una shel básica para probar el funcionamiento
#[test]
fn test() {
    let commands = command::Commands::initialize(&[
        command::Command {
            trigger: String::from("hw"),
            function: |command: &command::Command, args: Vec<String>| {
                graphics::slow_type("Hello World!");
                return Ok(String::from(""));
            }
        },
        command::Command {
            trigger: String::from("test"),
            function: |command: &command::Command, args: Vec<String>| {
                graphics::slow_type("You pressed test");
                return Ok(String::from(""));
            }
        },
        command::Command {
            trigger: String::from("arg"),
            function: |command: &command::Command, args: Vec<String>| {
                graphics::slow_type(&format!("The first argument is {}", args[0]));
                return Ok(String::from(""));
            }
        },
        command::Command {
            trigger: String::from("reverse"),
            function: |command: &command::Command, args: Vec<String>| {
                let input = &args[0];
                // Invertimos el input
                let res: String = input.chars().rev().collect();
                return Ok(String::from(res));
            }
        }]);
    interpreter(commands);   
}
