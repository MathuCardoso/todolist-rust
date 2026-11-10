# todo-app

Aplicação de lista de tarefas (to-do list) em Rust, com persistência em JSON.

## Recursos

- Cada tarefa possui: **ID**, **nome**, **descrição**, **data de criação** e **estado de conclusão**.
- Persistência automática em `~/.local/share/todo-app/tasks.json`.
- Funciona via CLI: digite `todo` em qualquer diretório.

## Comandos

| Comando | Descrição |
| --- | --- |
| `todo add <nome> [descrição]` | Adiciona uma nova tarefa |
| `todo list` | Lista todas as tarefas |
| `todo show <id>` | Exibe os detalhes de uma tarefa |
| `todo done <id>` | Marca uma tarefa como concluída |
| `todo remove <id>` | Remove uma tarefa |
| `todo help` | Exibe a ajuda |

## Instalação

```sh
cargo build --release
cp target/release/todo ~/.local/bin/todo
```

## Exemplo

```sh
todo add "Comprar pão" "Pães da padaria do canto"
todo list
todo done 1
```

## Dados

Os dados são salvos em `~/.local/share/todo-app/tasks.json` (ou `$XDG_DATA_HOME/todo-app/tasks.json`).