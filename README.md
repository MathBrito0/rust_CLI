# 🐧 Simulador de Sistema de Arquivos estilo Unix em Rust

[![Rust](https://img.shields.io/badge/Rust-ED3311?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Concluído-success?style=for-the-badge)](https://github.com/seu-usuario/seu-repositorio)

Projeto desenvolvido em dupla com o objetivo de simular funcionalidades fundamentais de um sistema operacional estilo Unix (como gerenciamento de usuários, grupos, diretórios, arquivos e permissões de acesso). 

O projeto foi escrito em **Rust**, aproveitando a segurança de tipos e o sistema estrito de gerenciamento de memória da linguagem.

---

## 🛠️ Arquitetura do Projeto

Embora o código tenha passado por um processo de consolidação para atender a requisitos acadêmicos específicos de submissão, a arquitetura original foi pensada de forma estritamente modular:

* **`modelos/`**: Contém as estruturas de dados principais (`Structs` e seus métodos):
  * `usuario.rs`: Gerenciamento de UIDs e vínculos com grupos.
  * `grupo.rs`: Gerenciamento de GIDs e listas de membros.
  * `diretorio.rs`: Estrutura de diretórios contendo vetores de arquivos.
  * `arquivo.rs`: Metadados do arquivo, tamanhos e donos.
  * `permissao.rs`: Controle de permissões (Leitura, Escrita, Execução) e conversão para formato octal (estilo chmod).
* **`menus/`**: Camada de interface de linha de comando (CLI) interativa para manipulação de cada entidade.

---

## 🚀 Funcionalidades

- **Usuários & Grupos:** Criação, listagem e associação de usuários a grupos do sistema.
- **Diretórios:** Criação, remoção e navegação/edição do conteúdo de diretórios.
- **Arquivos:** Criação de arquivos dentro de diretórios específicos, consulta de metadados (`stat`), alteração de permissões e remoção.

---

## 💻 Como Executar

Certifique-se de ter o [Rust e o Cargo](https://www.rust-lang.org/tools/install) instalados em sua máquina.

1. Clone o repositório:
   ```bash
   git clone [https://github.com/seu-usuario/seu-repositorio.git](https://github.com/seu-usuario/seu-repositorio.git)
   cd seu-repositorio
