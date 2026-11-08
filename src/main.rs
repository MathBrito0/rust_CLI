// Tabalho de autoria da dupla:
// Matheus Brito e Helen Midori

use crate::modelos::usuario::Usuario;
use crate::modelos::grupo::Grupo;
use crate::menus::menu_principal::{menu_principal};


mod modelos;
mod menus;

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();


    // MIDORI --    let mut diretorios: Vec<Diretorio> = Vec::new();

    menu_principal(&mut usuarios, &mut grupos);

}