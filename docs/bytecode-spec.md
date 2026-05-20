# Specifica del Bytecode ZLang

## 4. VM e runtime

### 4.1. Modello

- **VM stack-based**
- **Registri interni**:
  - `IP` (instruction pointer)
  - `SP` (stack pointer)
  - `FP` (frame pointer)
- **Segmenti**:
  - `Code`: bytecode
  - `Heap`: oggetti (str, list, map, closure)
  - `Stack`: valori + frame chiamate

### 4.2. Formato bytecode

- **Struttura modulo compilato**:
  - `Header`: magic `ZBC0`, versione
  - Tabella costanti
  - Tabella simboli (funzioni, variabili globali)
  - Sezione codice (array di istruzioni)

- **Istruzione**:
  - 1 byte opcode
  - 0–8 byte operandi (dipende dall’opcode)

### 4.3. Set di istruzioni base (estratto)

- **Stack**:
  - `PUSH_CONST idx`
  - `POP`
- **Variabili**:
  - `LOAD_LOCAL idx`
  - `STORE_LOCAL idx`
  - `LOAD_GLOBAL idx`
  - `STORE_GLOBAL idx`
- **Operazioni**:
  - `ADD`, `SUB`, `MUL`, `DIV`, `MOD`
  - `EQ`, `NEQ`, `LT`, `GT`, `LE`, `GE`
  - `AND`, `OR`, `NOT`
- **Controllo**:
  - `JMP addr`
  - `JMPIFFALSE addr`
  - `CALL func_idx, argc`
  - `RET`
- **Strutture**:
  - `NEW_LIST n`
  - `NEW_MAP n`
  - `GET_INDEX`
  - `SET_INDEX`
- **Syscall**:
  - `SYS_CALL id, argc` (per sys., net., ecc.)
