<div align="center">

# 🐧 Simulador de Sistema de Arquivos Unix em Rust

**Gerenciamento de usuários, grupos, diretórios e permissões via linha de comando (CLI).**

[![Rust](https://img.shields.io/badge/Rust-2021_Edition-ED3311?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Build-Cargo-F05032?style=for-the-badge&logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo/)
[![Status](https://img.shields.io/badge/Status-Concluído-2ea44f?style=for-the-badge)]()

[Sobre o Projeto](#-sobre-o-projeto) • [Arquitetura](#-arquitetura-e-módulos) • [Como Executar](#-como-executar) • [Diário Técnico](./docs/README.md)

</div>

---

## 📖 Sobre o Projeto

Este projeto é uma aplicação de linha de comando desenvolvida em **Rust** que simula o comportamento lógico de um sistema de arquivos e controle de acesso baseado em sistemas operacionais **Unix/Linux**.

O foco principal do desenvolvimento foi aplicar conceitos fundamentais de Sistemas Operacionais — como identificação por `UID`/`GID`, hierarquia de diretórios e cálculo de permissões octais (`rwx`) — aliados às garantias de segurança de memória (*ownership* e *borrowing*) da linguagem Rust.

> [!NOTE]
> **Nota sobre a organização do repositório:** Este repositório mantém a arquitetura **modular original** em pastas e arquivos separados para facilitar a leitura e manutenção. Para detalhes sobre como o projeto foi adaptado para um único arquivo (`main.rs`) para entrega acadêmica e os desafios enfrentados com o compilador, consulte o nosso **[Diário de Bordo Técnico](./docs/README.md)**.

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
```

### Resumo das Entidades

| Módulo | Estrutura Principal | Responsabilidade Técnica |
| :--- | :--- | :--- |
| **`Usuario`** | `uid: u16`, `grupos_ids: Vec<u16>` | Representa um usuário do sistema e rastreia os GIDs aos quais pertence. |
| **`Grupo`** | `gid: u16`, `membros_uids: Vec<u16>` | Agrupa usuários através de seus UIDs, permitindo vínculo bidirecional. |
| **`Diretorio`** | `arquivos: Vec<Arquivo>` | Atua como contêiner para arquivos, controlando adição, remoção e listagem. |
| **`Arquivo`** | `permissoes: (Permissao, Permissao, Permissao)` | Armazena metadados (nome, tamanho em bytes, UID, GID) e exibe detalhes via `stat()`. |
| **`Permissao`** | `leitura`, `escrita`, `execucao` (`bool`) | Calcula o valor octal (ex: `755`, `644`) para Dono, Grupo e Outros. |

---

## 💻 Como Executar

### Pré-requisitos
* Ter o ambiente **Rust** e o gerenciador **Cargo** instalados ([Guia oficial de instalação](https://www.rust-lang.org/pt-BR/tools/install)).

### Passo a passo
1. **Clone o repositório:**
   ```bash
   git clone [https://github.com/seu-usuario/seu-repositorio.git](https://github.com/seu-usuario/seu-repositorio.git)
   cd seu-repositorio
   ```

2. **Compile e inicie o programa:**
   ```bash
   cargo run
   ```

### Exemplo de Uso (Saída no Terminal)

```text
=== MENU PRINCIPAL ===
1. Menu dos usuários.
2. Menu dos grupos.
3. Menu dos diretórios.
4. Menu dos arquivos.
0. Finalizar o programa.
> 4

=== ARQUIVOS ===
1. Criar arquivo
2. Listar arquivos
3. Ver stat
4. Alterar permissão
5. Remover arquivo
0. Voltar
> 3
Nome do arquivo: relatorio.txt
Arquivo: relatorio.txt
Tamanho: 1024 bytes
Permissoes:(664 / rw-rw-r--)
Uid:1000
Gid:100
```

---

## 🔮 Melhorias Futuras (Roadmap)

- [ ] **Persistência de Dados:** Salvar e carregar o estado de usuários, grupos e arquivos em formato JSON ou binário (`serde`).
- [ ] **Diretórios Aninhados:** Permitir que um diretório contenha subdiretórios de forma recursiva.
- [ ] **Validação de Acesso:** Verificar efetivamente se o `UID` logado possui permissão de escrita/leitura antes de modificar um arquivo.

---

## 👥 Autores

| Desenvolvedor | GitHub | 
| :--- | :--- |
| **Matheus Brito** | [@MathBrito0](https://github.com/MathBrito0) |
| **Helen Midori** | [@komodospit](https://github.com/komodospit) |
