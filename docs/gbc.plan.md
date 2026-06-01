# Roadmap — Emulador de Game Boy Color

## Visão Geral

| Fase | Objetivo | Marco |
|--------|--------|--------|
| 1 | Loader de ROM | Carregar ROMs e ler cabeçalhos |
| 2 | Memory Bus | Sistema de memória funcionando |
| 3 | CPU Básica | Executar instruções simples |
| 4 | Testes de CPU | Passar nos testes básicos |
| 5 | Controle de Fluxo | Loops, chamadas e retornos |
| 6 | CPU Completa | Passar no `cpu_instrs.gb` |
| 7 | Timers | Timer funcionando |
| 8 | Interrupts | Sistema de interrupções |
| 9 | Renderização Inicial | Exibir tiles |
| 10 | Background | Renderizar cenários |
| 11 | Boot Sequence | Logo Nintendo aparece |
| 12 | Sprites | Personagens aparecem |
| 13 | MBC1 | Jogos maiores iniciam |
| 14 | Primeiro Jogo | Jogo totalmente jogável |
| 15 | Recursos GBC | Jogos exclusivos de GBC |
| 16 | - | Compatibilidade avançada |
| 17 | Áudio | Sons e músicas |
| 18 | Otimização | Compatibilidade e performance |

---

# Fase 1 — Loader de ROM

## Objetivo

Carregar e interpretar ROMs.

## Implementar

- [x] Leitura de arquivos `.gb`
- [x] Leitura de arquivos `.gbc`
- [x] Parser do cabeçalho
- [x] Exibição de informações da ROM

## Estrutura Inicial

```go
type Cartridge struct {
    ROM []byte
}
```

## Marco

- [x] Carregar qualquer ROM
- [x] Exibir informações do cartucho

---

# Fase 2 — Memory Bus

## Objetivo

Centralizar acessos à memória.

## Implementar

```go
Read(addr uint16) byte
Write(addr uint16, value byte)
```

## Componentes

- [ ] ROM
- [ ] VRAM
- [ ] WRAM
- [ ] HRAM
- [ ] IO Registers

## Marco

- [ ] CPU acessando toda memória através do Bus

---

# Fase 3 — CPU Básica

## Objetivo

Executar as primeiras instruções.

## Registradores

- [ ] AF
- [ ] BC
- [ ] DE
- [ ] HL
- [ ] SP
- [ ] PC

## Primeiros OpCodes

- [ ] NOP
- [ ] LD
- [ ] INC
- [ ] DEC

## Marco

- [ ] Executar programas simples sem travamentos

---

# Fase 4 — Testes de CPU

## Objetivo

Validar operações matemáticas.

## Implementar

- [ ] ADD
- [ ] ADC
- [ ] SUB
- [ ] SBC
- [ ] AND
- [ ] OR
- [ ] XOR
- [ ] CP

## Testes

- [ ] ROMs de teste da comunidade

## Marco

- [ ] Passar nos testes básicos da ALU

---

# Fase 5 — Controle de Fluxo

## Objetivo

Executar programas reais.

## Implementar

- [ ] JP
- [ ] JR
- [ ] CALL
- [ ] RET
- [ ] RST

## Conceitos

- [ ] Stack
- [ ] Saltos condicionais
- [ ] Endereçamento

## Marco

- [ ] Programas com loops funcionando

---

# Fase 6 — CPU Completa

## Objetivo

Implementar todas as instruções.

## Implementar

- [ ] 256 OpCodes principais
- [ ] 256 OpCodes CB-Prefixed

## Testes

- [ ] cpu_instrs.gb

## Marco

- [ ] Passar completamente em cpu_instrs.gb

---

# Fase 7 — Timers

## Objetivo

Implementar temporização interna.

## Registradores

- [ ] DIV
- [ ] TIMA
- [ ] TMA
- [ ] TAC

## Marco

- [ ] Testes de timer aprovados

---

# Fase 8 — Interrupts

## Objetivo

Implementar eventos do sistema.

## Tipos

- [ ] VBlank
- [ ] LCD
- [ ] Timer
- [ ] Serial
- [ ] Joypad

## Registradores

- [ ] IME
- [ ] IE
- [ ] IF

## Marco

- [ ] Boot ROM executando corretamente

---

# Fase 9 — Renderização Inicial

## Objetivo

Desenhar os primeiros pixels.

## Implementar

- [ ] SDL2
- [ ] Framebuffer
- [ ] Renderização de Tiles

## Resolução

```text
160x144
```

## Marco

- [ ] Exibir tiles da VRAM

---

# Fase 10 — Background

## Objetivo

Renderizar cenários.

## Implementar

- [ ] Tile Maps
- [ ] Scroll
- [ ] Paletas

## Marco

- [ ] Tela completa renderizada

---

# Fase 11 — Boot Sequence

## Objetivo

Mostrar a sequência de inicialização.

## Sincronização

- [ ] CPU
- [ ] PPU
- [ ] Timers

## Marco

- [ ] Logo Nintendo aparece corretamente

---

# Fase 12 — Sprites

## Objetivo

Renderizar objetos móveis.

## Implementar

- [ ] OAM
- [ ] Prioridade
- [ ] Flip Horizontal
- [ ] Flip Vertical

## Marco

- [ ] Sprites aparecem corretamente

---

# Fase 13 — MBC1

## Objetivo

Suportar ROMs maiores.

## Implementar

- [ ] ROM Banking
- [ ] RAM Banking

## Marco

- [ ] Jogos comerciais iniciam

---

# Fase 14 — Primeiro Jogo Jogável

## Objetivo

Executar um jogo completo.


## Marco

- [ ] Completar uma partida sem falhas graves

---

# Fase 15 — Recursos do Game Boy Color

## Objetivo

Adicionar suporte específico do GBC.

## Implementar

- [ ] VRAM Bank 0
- [ ] VRAM Bank 1
- [ ] WRAM Banking
- [ ] Color Palettes
- [ ] HDMA
- [ ] Double Speed Mode

## Marco

- [ ] Jogos exclusivos de GBC iniciam

---

# Fase 16 — compatibilidade avançada

## Objetivo

Validar compatibilidade avançada.

## Corrigir

- [ ] Bugs gráficos
- [ ] Bugs de timing
- [ ] Bugs de interrupções

## Marco

- [ ] Jogar normalmente
- [ ] Salvar progresso
- [ ] Carregar save

---

# Fase 17 — Áudio

## Objetivo

Implementar APU.

## Canais

- [ ] Square 1
- [ ] Square 2
- [ ] Wave
- [ ] Noise

## Marco

- [ ] Música e efeitos funcionando

---

# Fase 18 — Otimização

## Objetivo

Aumentar compatibilidade e performance.

## Melhorias

- [ ] Profiling
- [ ] Testes automatizados
- [ ] Save States
- [ ] Debugger
- [ ] Benchmarks

## Marco

- [ ] Compatibilidade acima de 90%
