<div align="center">

# 🦆 Bongo Duck — Seu Desktop Pet de Produtividade

<img src="logo.png" alt="Bongo Duck Logo" width="180"/>

### Um companheiro virtual para desktop com Pomodoro integrado, animações vetoriais e alertas sonoros.

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-blue.svg)](https://github.com/Luirafa2020/Bongo-Duck/releases)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/Luirafa2020/Bongo-Duck?color=brightgreen)](https://github.com/Luirafa2020/Bongo-Duck/releases)

</div>

---

## 📖 Sobre o Projeto

O **Bongo Duck** é um desktop pet leve e personalizável feito em **Rust** com a biblioteca gráfica **egui/eframe**. Inspirado no clássico *Bongo Cat*, ele fica flutuando na sua tela enquanto você estuda ou trabalha, ajudando você a manter o foco através da técnica **Pomodoro**.

A janela é transparente, sem bordas e configurada para ficar **sempre no topo** (*Always on Top*), acompanhando seu fluxo de trabalho sem atrapalhar suas tarefas.

---

## ✨ Funcionalidades

- 🍅 **Temporizador Pomodoro**: Ciclos configurados de **25 minutos de Foco** e **5 minutos de Pausa**.
- ⌨️ **Animação Bongo Cat**: O pato digita alternando as patinhas no teclado e no mouse durante o tempo de foco.
- 😴 **Modo Sesta (Descanso)**: Durante a pausa, ele fecha os olhos para dormir com efeito animado de `Zzz...` e gotinha no bico.
- 🥰 **Interação e Carinho**: Clique nele para fazer carinho! Ele solta corações flutuantes e frases de incentivo.
- 🔊 **Alerta Sonoro (`quack.mp3`)**: Toca um alerta sonoro exclusivo sempre que um ciclo Pomodoro termina.
- 🎨 **Gráficos 100% Vetoriais (SVG)**: Renderização nítida em qualquer resolução (Full HD, Quad HD e 4K).
- 🖱️ **Controle Dinâmico de Tamanho**: Passe o mouse sobre o pato e role o *scroll* para aumentar ou diminuir o tamanho dele em tempo real.
- 📌 **Arraste Livre**: Clique e arraste o patinho para posicioná-lo em qualquer canto do monitor.

---

## 🎭 Poses do Pato

| Parado (Idle) | Digitando (Foco) | Carinho (Clique) | Dormindo (Pausa) |
| :---: | :---: | :---: | :---: |
| <img src="pato.svg" width="110"/> | <img src="pato1.svg" width="110"/> | <img src="pato3.svg" width="110"/> | <img src="pato4.svg" width="110"/> |
| *Aguardando início* | *Trabalhando a todo vapor* | *Recebendo carinho* | *Descanso merecido* |

---

## 📥 Downloads (Binários Prontos)

Baixe a versão mais recente diretamente na aba de [**Releases**](https://github.com/Luirafa2020/Bongo-Duck/releases):

### 🐧 Linux (AppImage)
Compatível com Fedora, Ubuntu, Debian, Arch Linux e qualquer distribuição moderna:
1. Baixe o arquivo **`BongoDuck-x86_64.AppImage`**.
2. Dê permissão de execução:
   ```bash
   chmod +x BongoDuck-x86_64.AppImage