// struct Usuario com lista de id dos grupos;
// Usa os métodos new, "adiciona_grupo", "remove_grupo", "listar_grupos".
// É usado pelo "menu_usuarios.rs" e pelo "main.rs".


pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    pub grupos_ids: Vec<u16>,
}


impl Usuario {
    pub fn new(nome: String, uid: u16) -> Self {
        Usuario {
            nome: nome,
            uid: uid,
            grupos_ids: Vec::new(),
        }
    }
    
    pub fn adiciona_grupo(&mut self, uid: u16) {
            self.grupos_ids.push(uid);
        }

    pub fn listar_grupos(&self) {

        if self.grupos_ids.len() == 0 {
            println!("Nenhum grupo encontrado.");

        } else {

            for n in &self.grupos_ids {
            println!("{}", n);
            }

        }
    }

    pub fn remove_grupo(&mut self, gid: u16) {
        for n in 0..self.grupos_ids.len() {
            if self.grupos_ids[n] == gid {
                self.grupos_ids.remove(n);
                break;
            }
        }
    }
}