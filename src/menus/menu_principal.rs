// Menu inicial que leva aos submenus. Usa os 4 outros menus e é usado pelo "main.rs"


use crate::menus::menu_usuarios::{ler_numero, menu_usuarios};
use crate::modelos::usuario::Usuario;

pub fn menu_pincipal(usuarios: &mut Vec<Usuario>) {
   
    loop {
        println!("=== MENU PRINCIPAL ===");
        println!("1. Menu dos usuários");
        println!("0. Finalizar o programa.");

        let opcao = ler_numero();

        match opcao {
            1 => menu_usuarios(usuarios),
            0 => break,
            _ => println!("Essa opção é inválida."),

        }
    }
}