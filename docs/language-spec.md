# Specifica del Linguaggio ZLang

## 1. Identità del linguaggio

- **Nome linguaggio**: ZLang
- **Estensione file**: `.zlang`
- **Ruolo**:
  - Linguaggio di sistema nativo ZDOS
  - Scripting, daemon, orchestrazione tool, client/nodi blockchain
- **Target**:
  - Interprete + VM a bytecode
  - Portabile su Linux/Termux, x86 vecchi, ARM

## 2. Specifica del linguaggio

### 2.1. Tipi e valori

- **Tipi primitivi**:
  - `int` (64 bit)
  - `float` (64 bit)
  - `bool` (true / false)
  - `str` (UTF-8)
  - `bytes`
  - `list` (eterogenea)
  - `map` (chiave `str` → valore generico)
  - `func`
- **Literals**:

```zlang
let a = 42
let pi = 3.14
let ok = true
let name = "High"
let data = 0xDEADBEEF
let arr = [1, 2, 3]
let cfg = { "id": "node-1", "port": 8080 }
```

### 2.2. Variabili e assegnazione

- **Dichiarazione**:

```zlang
let x = 10
let msg: str = "hello"
```

- **Riassegnazione**:

```zlang
x = x + 1
```

### 2.3. Funzioni

- **Definizione**:

```zlang
func add(a: int, b: int) -> int {
    return a + b
}

func log_system(msg: str) {
    sys.log("core", msg)
}
```

- **Funzioni anonime**:

```zlang
let f = func(x: int) -> int {
    return x * 2
}
```

### 2.4. Controllo di flusso

```zlang
if x > 5 {
    sys.log("test", "x > 5")
} else {
    sys.log("test", "x <= 5")
}

for i in 0..10 {
    sys.log("loop", "i=" + str(i))
}

while x < 100 {
    x = x + 10
}
```

### 2.5. Moduli

- **Dichiarazione modulo**:

```zlang
module chain.node
```

- **Import**:

```zlang
import sys
import net
import chain.util
```

### 2.6. Errori ed eccezioni

```zlang
func risky() {
    if something_wrong {
        throw "bad state"
    }
}

func main() {
    try {
        risky()
    } catch err {
        sys.log("err", "caught: " + err)
    }
}
```

## 3. Grammatica (EBNF minimale)

```ebnf
Program      = { Statement } ;

Statement    = VarDecl | Assign | FuncDecl | IfStmt | ForStmt | WhileStmt
             | ImportStmt | ModuleStmt | ExprStmt | ThrowStmt | TryCatch ;

VarDecl      = "let" Identifier [ ":" Type ] "=" Expression ;
Assign       = Identifier "=" Expression ;
FuncDecl     = "func" Identifier "(" [ ParamList ] ")" [ "->" Type ] Block ;
ParamList    = Param { "," Param } ;
Param        = Identifier ":" Type ;

IfStmt       = "if" Expression Block [ "else" Block ] ;
ForStmt      = "for" Identifier "in" Expression ".." Expression Block ;
WhileStmt    = "while" Expression Block ;

ImportStmt   = "import" Identifier { "." Identifier } ;
ModuleStmt   = "module" Identifier { "." Identifier } ;

ThrowStmt    = "throw" Expression ;
TryCatch     = "try" Block "catch" Identifier Block ;

Block        = "{" { Statement } "}" ;

ExprStmt     = Expression ;

Expression   = LogicOr ;
LogicOr      = LogicAnd { "||" LogicAnd } ;
LogicAnd     = Equality { "&&" Equality } ;\nEquality     = Relational { ("==" | "!=") Relational } ;
Relational   = Additive { ("<" | ">" | "<=" | ">=") Additive } ;
Additive     = Multiplicative { ("+" | "-") Multiplicative } ;
Multiplicative = Unary { ("*" | "/" | "%") Unary } ;
Unary        = [ "!" | "-" ] Primary ;
Primary      = Literal
             | Identifier
             | "(" Expression ")"
             | Primary "." Identifier
             | Primary "(" [ ArgList ] ")" ;

ArgList      = Expression { "," Expression } ;

Literal      = IntLiteral | FloatLiteral | StringLiteral
             | BoolLiteral | ListLiteral | MapLiteral ;

ListLiteral  = "[" [ Expression { "," Expression } ] "]" ;
MapLiteral   = "{" [ StringLiteral ":" Expression { "," StringLiteral ":" Expression } ] "}" ;

Type         = "int" | "float" | "bool" | "str" | "bytes" | "list" | "map" | "func" ;

Identifier   = Letter { Letter | Digit | "_" } ;
```
