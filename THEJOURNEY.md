# Starting My Compiler Journey

I've asked Microsoft Copilot how to start createing a compiler in rust. 
The respons was

## 1. Understanding the Basics of a Compiler

A compiler translates high-level source code (like the Ada-inspired code you've shared) into machine-level instructions, typically assembly or binary code. This involves several stages:

* **Lexical Analysis:** Splitting the source code into tokens.
* **Parsing:** Analyzing the structure based on grammar rules.
* **Semantic Analysis**: Ensuring logical correctness.
* **Code Generation**: Producing assembly or machine code.
* **Optimization** (optional): Improving performance.

## 2. Outline of Our Compiler

Since we're focusing on Rust and ADA95-inspired syntax, we'll design a compiler with:

* **Front-End**: Responsible for lexical analysis, parsing, and semantic checks.
* **Back-End**: Generates 64-bit assembly.

We'll write this compiler in Rust, utilizing its safety and performance features. The target assembly will be x86-64.

## 3. Language Design Inspired by ADA95

The language you described resembles Ada95's syntax:

* **Variable Declaration**: Using ```var```.
* **Integer Operations**: Arithmetic and assignment operations.
* **Output Functionality**: ```writeln``` for printing results.

We'll first define the grammar and rules for this language. For example:
* Keywords like ```var```,```begin```,```end```.
* Supported types, lke ```integer```.
* Statement structures.


## High level grammar

Our language's grammar will be defined using rules that describe valid constructs. We can use a notation like Backus-Naur Form (BNF) or Extended BNF (EBNF) to express these rules.
Here’s a first draft:

### 1. Program Structure

```
Program ::= VariableSection StatementSection
VariableSection ::= "var" VariableDeclarations
StatementSection ::= "begin" Statements "end"
```

### 2. Variable Declaration

```
VariableDeclarations ::= VariableDeclaration ";"
VariableDeclaration ::= IdentifierList ":" Type
IdentifierList ::= Identifier { "," Identifier }
Type ::= "integer" | "string" | "boolean"
```

### 3. Statements

```
Statements ::= Statement {";" Statement}
Statement ::= Assignment | Print
Assignment ::= Identifier ":=" Expression
Print ::= "writeln" "(" StringLiteral { "," Expression } ")"
```

### 4. Expressions

```
Expression ::= Term { ("+" | "-") Term }
Term ::= Factor { ("*" | "/") Factor }
Factor ::= Identifier | IntegerLiteral | "(" Expression ")"
```

### 5. Literals and Identifiers

```
Identifier ::= [a-zA-Z][a-zA-Z0-9]*
IntegerLiteral ::= [0-9]+
StringLiteral ::= '"' [^"]* '"'
```

### Key Syntax Features

* **Variable Declaration**: Variables are declared in the ```var```  section with types specified after a colon (```:```). Multiple variables can be declared together, separated by commas.
    * Example ```var a,b : integer```
* **Assignment**: The ```:=``` operator is used for assinging values to variables.
    * Example ```a := 10;```
* **Print Statement**: ```writeln``` prints text and variable values to the output. It allows concatenation of strings and expressions using commas.
    * Example: ```writeln("The result is: ", result);
* **Expressions**: Support basic arithmetic (```+```,```-```,```*```,```/```) and parentheses for grouping.

## Example in Practice

Here's how the grammar applies

```ada
var
    a, b: integer;
    result: integer;

begin
    a := 10;
    b := 20;
    result := a + b;
    writeln('The result is: ', result);
end
```

#### This program:
1. Declares variables ```a```, ```b``` and ```result``` as ingegers
2. Assigns values to ```a``` and ```b```.
3. Computes the sum of ```a```  and ```b```, storing it in ```result```.
4. Prints the result with a message.


