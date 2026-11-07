// Tabalho de autoria da dupla:
// Matheus Brito e Helen Midori

use crate::modelos::usuario::Usuario;
use crate::modelos::grupo::Grupo;


mod modelos;

fn main() {
    let mut user1 = Usuario::new("Caio".to_string(), 2);
    let mut user2 = Usuario::new("Julia".to_string(), 3);
    println!("Atributos do primeiro usuário: Nome - {}, ID - {} e Qntd grupos - {}", user1.nome, user1.uid, user1.grupos_ids.len());
    println!("Atributos do segundo usuário: Nome - {}, ID - {} e Qntd grupos - {}", user2.nome, user2.uid, user2.grupos_ids.len());


    user1.listar_grupos();
    user2.listar_grupos();

    user1.adiciona_grupo(10);
    user1.adiciona_grupo(20);
    println!("-------- {} {} ---------", user1.nome, user1.grupos_ids.len());

    user1.remove_grupo(20);
    println!("-------- {} {} ---------", user1.nome, user1.grupos_ids.len());

    println!("Atributos do primeiro usuário: Nome - {}, ID - {} e Qntd grupos - {}", user1.nome, user1.uid, user1.grupos_ids.len());
    println!("Atributos do segundo usuário: Nome - {}, ID - {} e Qntd grupos - {}", user2.nome, user2.uid, user2.grupos_ids.len());

}