// Menu inicial que leva aos submenus. Usa os 4 outros menus e é usado pelo "main.rs"


use crate::menus::menu_usuarios::{ler_numero, menu_usuarios};
use crate::menus::menu_grupos::{menu_grupos};
use crate::modelos::usuario::Usuario;
use crate::modelos::grupo::Grupo;


pub fn menu_principal(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
   
    loop {
        println!("=== MENU PRINCIPAL ===");
        println!("1. Menu dos usuários.");
        println!("2. Menu dos grupos.");
        println!("0. Finalizar o programa.");

        let opcao = ler_numero();

        match opcao {
            1 => menu_usuarios(usuarios, grupos),
            2 => menu_grupos(grupos, usuarios),
            
            3 => println!("pasta 'arquivos' em desenvolvimento."),  // MIDORI -- troca esse print pela função 'menu_arquivos(diretorios)'
            4 => println!("pasta 'diretórios' em desenvolvimento."),  // MIDORI -- troca esse print pela função 'menu_diretorios(diretorios)'
           
            0 => break,
            _ => println!("Essa opção é inválida."),

        }
    }
}