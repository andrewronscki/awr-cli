<h1 align="center">
     🤖 <a href="#" alt="site do places"> AWR CLI</a>
</h1>

<h3 align="center">
    ⚙️ A CLI para padronizar a criação e evolução de nossos projetos. 💗
</h3>

<div align="center">
<p align="center">
<a><img alt="GitHub package.json version" src="https://img.shields.io/github/package-json/v/andrewronscki/places-serverside"></a>
<a><img alt="GitHub" src="https://img.shields.io/github/license/andrewronscki/places-serverside"></a>
<a><img alt="GitHub contributors" src="https://img.shields.io/github/contributors/andrewronscki/places-serverside"></a>
<a><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/andrewronscki/places-serverside"></a>
</p>
</div>

Tabela de conteúdos
=================
<!--ts-->
   * [Sobre o projeto](#-sobre-o-projeto)
   * [Funcionalidades](#%EF%B8%8F-funcionalidades)
   * [Como executar o projeto](#-como-executar-o-projeto)
     * [Pré-requisitos](#pré-requisitos)
     * [Rodando/Criando um executável novo para a CLI](#-rodandocriando-um-executável-novo-para-a-cli)
   * [Tecnologias](#-tecnologias)
     * [CLI](#cli-rust)
   * [Estrutura do Template do Projeto](#-estrutura-do-template-do-projeto)
   * [Autor](#-autor)
   * [Licença](#-licença)
<!--te-->


## 💻 Sobre o projeto

🤖 AWR CLI - é uma forma de criar e evoluir projetos utilizando o padrão adotado pela AWR.

É uma Interface de Linha de Comando que permite criar novos projetos em NestJS e futuramente em Go Lang utilizando Clean Archtecture e DDD.

---

## ⚙️ Funcionalidades

- [x] NestJS:
  - [x] Criar templete
```bash
awr nest new hello-world
```

---

## 🚀 Como executar o projeto

### Pré-requisitos

Antes de começar, você vai precisar ter instalado em sua máquina a seguinte ferramenta:
[Rust](https://rustup.rs/).
Além disto é bom ter um editor para trabalhar com o código como [VSCode](https://code.visualstudio.com/)

#### 🎲 Rodando/Criando um executável novo para a CLI

```bash

# Clone este repositório
$ git clone git@github.com:andrewronscki/awr-cli.git

# Acesse a pasta do projeto no terminal/cmd
$ cd awr-cli

# Execute a CLI com o seguinte comando
$ cargo run nest new hello-world

# Será criado uma pasta com o nome hello-world e nele irá conter o template do projeto
# Na pasta target/debug terá sido criado um arquivo executavel chamado AWR, rodando no windows ele criará um executável chamado AWR.exe

```

---

## 🛠 Tecnologias

As seguintes ferramentas foram usadas na construção do projeto:

#### [](https://github.com/AWR-solucoes-de-pagamentos/AWR-cli)**CLI** ([Rust](https://www.rust-lang.org/pt-BR))
-   **[clap](https://docs.rs/crate/clap/4.0.32)**
-   **[error-chain](https://docs.rs/crate/error-chain/0.12.4)**
-   **[reqwest](https://docs.rs/crate/reqwest/0.11.13)**
-   **[tokio](https://docs.rs/crate/tokio/1.24.1)**
-   **[zip](https://docs.rs/crate/zip/0.6.3)**

> Veja o arquivo  [Cargo.toml](https://github.com/AWR-solucoes-de-pagamentos/AWR-cli/blob/main/Cargo.toml)

---

## 🛠 Estrutura do Template do Projeto
A estrutura do template do projeto segue este modelo:
- O sistema deve ser desenvolvido utilizando os ensinamentos do Clean Architecture e Domain-Driven Design, separando as camadas em domain, data, infra e presentation;
- O sistema deve ser agrupado por módulos independentes;
- O desenvolvimento dos casos de uso deve ser orientado a testes (TDD).

<div align="center">
  <img alt="Arquitetura da Api" src="./arquitetura-software.png">
</div>

---

## 🦸 Autor

<a href="https://andrewronscki.com">
 <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/32884775?v=4" width="100px;" alt=""/>
 <br />
 <sub><b>André Wronscki Ricardo</b></sub></a> <a href="https://andrewronscki.com" title="André Wronscki">🚀</a>
 <br />
 <br />


[![Linkedin Badge](https://img.shields.io/badge/-André-blue?style=flat-square&logo=Linkedin&logoColor=white&link=https://www.linkedin.com/in/andr%C3%A9-wronscki-ricardo-13694bb7/)](https://www.linkedin.com/in/andr%C3%A9-wronscki-ricardo-13694bb7/)
[![Gmail Badge](https://img.shields.io/badge/-andrewronscki@gmail.com-c14438?style=flat-square&logo=Gmail&logoColor=white&link=mailto:andrewronscki@gmail.com)](mailto:andrewronscki@gmail.com)

---

## 📝 Licença

Este projeto esta sobe a licença [MIT](./LICENSE.md).
