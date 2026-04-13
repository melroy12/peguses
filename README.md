# Peguses

A simple programming language written in Rust.

## Features

### Data Types
- **Integers**: 64-bit signed integers (`i64`)
- **Booleans**: `true` and `false`
- **Strings**: Text enclosed in double quotes with escape sequence support

### Operators

**Arithmetic Operators:**
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Modulo: `%`
- Unary negation: `-`

**String Operators:**
- Concatenation: `+` (for strings)

**Comparison Operators:**
- Equal: `==`
- Not equal: `!=`
- Less than: `<`
- Greater than: `>`
- Less than or equal: `<=`
- Greater than or equal: `>=`

**Logical Operators:**
- AND: `&&`
- OR: `||`
- NOT: `!`

### Statements

**Variable Declaration:**
```
let x = 10;
let is_valid = true;
```

**Variable Assignment:**
```
x = x + 1;
```

**Print Statement:**
```
print x;
print 42;
```

**If-Else Statement:**
```
if x > 10 {
    print 1;
} else {
    print 0;
}
```

**While Loop:**
```
let counter = 5;
while counter > 0 {
    print counter;
    counter = counter - 1;
}
```

**For Loop:**
```
for i in 0..10 {
    print i;
}
```

**Break Statement:**
```
for i in 0..10 {
    if i == 5 {
        break;
    }
    print i;
}
```

**Continue Statement:**
```
for i in 0..10 {
    if i % 2 == 0 {
        continue;
    }
    print i;  // Only prints odd numbers
}
```

**String Literals:**
```
let greeting = "Hello, World!";
let message = "Line 1\nLine 2";  // Escape sequences: \n, \t, \r, \\, \"
print greeting;
```

**String Concatenation:**
```
let first = "Hello";
let second = "World";
let combined = first + " " + second;
print combined;  // Prints: Hello World
```

### Comments
Single-line comments are supported using `//`:
```
// This is a comment
let x = 10;  // This is also a comment
```

## Example Program

```peguses
// Fibonacci sequence
let n1 = 0;
let n2 = 1;
let count = 10;

while count > 0 {
    print n1;
    let temp = n1 + n2;
    n1 = n2;
    n2 = temp;
    count = count - 1;
}
```

## Building and Running

```bash
cd peguses_lang
cargo build
cargo run
```

## Project Structure

- `src/main.rs` - Entry point with example programs
- `src/lexer.rs` - Tokenizes source code
- `src/token.rs` - Token definitions
- `src/parser.rs` - Parses tokens into an AST
- `src/ast.rs` - Abstract Syntax Tree definitions
- `src/interpreter.rs` - Executes the AST 