// Tabalho de autoria da dupla:
// Matheus Brito e Helen Midori

use crate::modelos::usuario::Usuario;
use crate::modelos::grupo::Grupo;
use crate::menus::menu_usuarios::menu_usuarios;


mod modelos;
mod menus;

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    menu_usuarios(&mut usuarios);

}