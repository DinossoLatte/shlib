use std::iter::Iterator;

/// Trait para los comandos
/// 
/// Este trait debe ser implementado por Command, pero puede implementarse de otras 
/// formas si se desea.
/// 
/// 'execute' - Esta función será llamada cuando en el shell se introduzca el trigger
/// 'get_trigger' - Esta función debe retornar el trigger, o el texto
/// por el que se conocerá este comando, del comando.
pub trait CommandTrait {
    fn execute(&self, args: Vec<String>) -> Result<String, String>;
    fn get_trigger(&self) -> &String;
}

#[derive(Clone)]
pub struct Command {
    pub trigger: String,
    pub function: fn(&Command, args: Vec<String>) -> Result<String, String>
}

impl CommandTrait for Command {
    /// execute
    /// 
    /// Esta función será llamada cuando se introduzca el String emitido por get_trigger
    /// Debe retornar un Result con Strings, que será el resultado al terminar el comando.
    /// Se puede no incluir una respuesta, introduciendo Ok(String::from("")) o String::new().
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        return (self.function)(self, args);
    }

    /// get_trigger
    /// 
    /// Esta función retornará el comando con el que responderá este objeto. En el caso de que 
    /// se introduzca el trigger en el shell, se llamará a la función 'execute'
    fn get_trigger(&self) -> &String {
        return &self.trigger;
    }
}

#[derive(Clone, Debug)]
// El objeto T en general debe implementar CommandTrait
pub struct Commands<T: CommandTrait + Clone> {
    pub command_list: Vec<T>
}

impl Commands<Command> {
    pub fn initialize(array: &[Command]) -> Commands<Command> {
        // Creamos el mapa con cada componente 
        let commands = Commands {
            command_list: Vec::from(array)
        };
        return commands;
    }

    // El bool retornado será el resultado, es decir si se encuentra algún trigger
    pub fn execute_command(&self, command_str: String, args: Vec<String>) -> Result<String, String> {
        // Primero detectamos el elemento que se ha querido llamar
        let filtered = self.command_list
            // Convertimos la lista en iterador
            .iter()
            // Comprobamos que exista algún elemento
            .find(|command| command.get_trigger().clone() == command_str);
        // ¿Existe algún comando con el trigger?
        if filtered.is_some() {
            // Si es el caso, ejecuta el execute del comando
            // Es necesario clonar el objeto al deber retornarlo después.
            return filtered.cloned().unwrap().execute(args);
        }
        return Err(String::from("Command not found"));
    }

    pub fn get_command(&self, command_str: String) -> Option<&Command> {
        // Iteramos por los comandos
        self.command_list.iter().find(|command| command.get_trigger().clone() == command_str)
    }
}