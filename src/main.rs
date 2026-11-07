// Tabalho de autoria da dupla:
// Matheus Brito e Helen Midori

use crate::modelos::usuario::Usuario;

mod modelos;

fn main() {
    let mut user1 = Usuario::new("Caio".to_string(), 2);
    println!("{} {}", user1.nome, user1.grupos_ids.len());

    user1.adiciona_grupo(10);
    println!("{} {}", user1.nome, user1.grupos_ids.len());
}