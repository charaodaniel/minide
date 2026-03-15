# MinIDE

MinIDE é um editor de código leve e modular focado em simplicidade, performance e personalização.

O objetivo do projeto é criar um editor moderno inspirado em ferramentas como Visual Studio Code, porém muito mais leve e eficiente, ideal para computadores com poucos recursos.

O projeto terá duas versões principais:

* **MinIDE CLI** — editor de código para terminal (TUI)
* **MinIDE GUI** — interface gráfica baseada em Tauri

Ambas as versões compartilharão o mesmo núcleo de funcionalidades.

---

# Objetivos do projeto

* Criar um editor de código leve
* Rodar bem em computadores com hardware limitado
* Ter interface simples e intuitiva
* Permitir extensões e customizações
* Integrar ferramentas de desenvolvimento modernas

---

# Arquitetura do projeto

O projeto está dividido em módulos:

```
minide/
├ core/      # núcleo do editor
├ cli/       # versão terminal
├ gui/       # versão gráfica (Tauri)
├ themes/    # temas de cores
├ icons/     # ícones de arquivos (Nerd Fonts)
└ plugins/   # sistema de extensões
```

### Core

O módulo **core** contém a lógica principal do editor:

* gerenciamento de arquivos
* buffer de texto
* integração com git
* sistema de temas
* suporte a plugins

Tanto o CLI quanto a GUI utilizam esse núcleo.

---

# Versões do editor

## MinIDE CLI

Editor de terminal inspirado em ferramentas como:

* Helix
* Neovim
* Nano

Características planejadas:

* interface TUI
* navegação rápida por arquivos
* syntax highlight
* integração com Git
* suporte a Nerd Fonts
* tema Catppuccin

---

## MinIDE GUI

Interface gráfica baseada em **Tauri**.

Características planejadas:

* editor visual moderno
* suporte a temas
* explorador de arquivos
* integração com Git
* terminal integrado
* suporte a extensões

---

# Temas

O MinIDE terá suporte a temas externos.

Tema inicial:

* Catppuccin (Mocha / Macchiato)

Os temas serão carregados a partir da pasta:

```
themes/
```

---

# Ícones

O editor suportará ícones baseados em **Nerd Fonts** para identificar arquivos no explorador.

Exemplo:

```
 main.py
 index.html
 style.css
 database.sql
```

---

# TODO / Roadmap

## Fase 1 — Base do projeto

* [x] Estruturar Cargo Workspace
* [x] Criar núcleo do editor (core)
* [x] Implementar buffer de texto
* [x] Implementar abertura e salvamento de arquivos
* [x] Implementar carregamento de temas
* [x] Implementar sistema de ícones

---

## Fase 2 — Editor CLI

* [ ] Criar interface TUI
* [ ] Implementar navegação de arquivos
* [ ] Implementar syntax highlight
* [ ] Implementar atalhos de teclado
* [ ] Implementar busca de texto
* [ ] Implementar integração básica com Git

---

## Fase 3 — Editor GUI

* [ ] Criar interface com Tauri
* [ ] Integrar Monaco Editor
* [ ] Criar explorador de arquivos
* [ ] Implementar sistema de abas
* [ ] Integrar terminal
* [ ] Integrar Git

---

## Fase 4 — Plugins

* [ ] Criar sistema de extensões
* [ ] API para plugins
* [ ] carregamento dinâmico de extensões

---

## Fase 5 — Qualidade de vida

* [ ] suporte a múltiplos temas
* [ ] suporte a atalhos customizados
* [ ] suporte a LSP (Language Server Protocol)
* [ ] suporte a formatação automática
* [ ] suporte a autocomplete

---

# Contribuições

Este projeto ainda está em fase inicial. Contribuições, sugestões e ideias são bem-vindas.

---

# Licença

Este projeto será distribuído sob licença open source (a definir).
