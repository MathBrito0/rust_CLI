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
    
    pub fn adiciona_grupo(&mut self, gid: u16) {
            self.grupos_ids.push(gid);
        }
}


