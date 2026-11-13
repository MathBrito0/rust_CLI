<div align="center">

# 🐧 Simulador de Sistema de Arquivos Unix em Rust

**Gerenciamento de usuários, grupos, diretórios e permissões via linha de comando (CLI).**

[![Rust](https://img.shields.io/badge/Rust-Version_1.85-ED3311?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Build-Cargo-F05032?style=for-the-badge&logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo/)
[![Status](https://img.shields.io/badge/Status-Concluído-2ea44f?style=for-the-badge)]()

[Sobre o Projeto](#-sobre-o-projeto) • [Arquitetura](#-arquitetura-e-módulos) • [Como Executar](#-como-executar) • [Diário Técnico](./docs/README.md)

</div>

---

## 📖 Sobre o Projeto

Este projeto é uma aplicação de linha de comando desenvolvida em **Rust** que simula o comportamento lógico de um sistema de arquivos e controle de acesso baseado em sistemas operacionais **Unix/Linux**.

O foco principal do desenvolvimento foi aplicar conceitos fundamentais de Sistemas Operacionais — como identificação por `UID`/`GID`, hierarquia de diretórios e cálculo de permissões octais (`rwx`) — aliados às garantias de segurança de memória (*ownership* e *borrowing*) da linguagem Rust.

> [!NOTE]
> **Nota sobre a versão do repositório:** Este repositório mantém a arquitetura **modular original** em pastas e arquivos separados para facilitar a leitura e manutenção. Para detalhes sobre como o projeto foi adaptado para um único arquivo (`main.rs`) para entrega acadêmica e os desafios enfrentados com o compilador, consulte o nosso **[Diário de Bordo Técnico](./docs/README.md)**.

---

## 🏗️ Arquitetura e Módulos

O sistema foi projetado separando as regras de negócio (`modelos`) da interface de interação com o usuário (`menus`):

```text
src/
├── main.rs                 # Ponto de entrada e inicialização dos vetores em memória
├── modelos/                # Entidades e regras de negócio (Structs & Impls)
│   ├── usuario.rs          # Gestão de usuários e vínculos com grupos
│   ├── grupo.rs            # Gestão de grupos e listas de membros
│   ├── diretorio.rs        # Contêiner hierárquico que armazena arquivos
│   ├── arquivo.rs          # Metadados, tamanho, dono e exibição estilo `stat`
│   └── permissao.rs        # Lógica booleana (r, w, x) e conversão para octal
└── menus/                  # Camada de interação via terminal (CLI)
    ├── menu_principal.rs   # Roteador central dos submenus
    ├── menu_usuarios.rs    # CRUD de usuários e helpers de leitura
    ├── menu_grupos.rs      # CRUD de grupos e associações
    ├── menu_diretorios.rs  # Criação e edição de diretórios
    └── menu_arquivos.rs    # Criação, alteração de permissões e status de arquivos
