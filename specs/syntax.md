# Unnamed Language

NOTES:
- Basic, like Lua
- Runs Inside a VM, like Lua/Java
- Is meant to be a learning project for later
- Is meant to execute exposed functions from the Host language
- Don't have a name yet

## Types

Booleans Literals

- `true` 
- `false`

Int and Float

- Only decimal
- '.' for decimal separation

Strings

- Enclosed in '"'

Nil

- `nil`, though I would like to make it strongly typed without requiring null values I don't know how yet

## Expressions

Arithmetic

- left_operand + right_operand (also for -, * and /)
- -operand (unary, for negation)

Comparison

- <, <=, >, >=, == and !=

Different types always evaluate to `false`

Logic

- not
- and (returns left operand if false, right operand if true)
- or (returns right operand if false, left operand if true)

## Statements

- expressions ending in ';'

## Variables

- `let var_name := value;`
- `let` is the keyword for assigning the variable
- `:=` is the assign operator
- must always be initialized

## Control flow

- `if (...) { ... } else { ... }`
- `while (...) { ... }`
- `for (i in [n..m;s]) { ... }`
  - n is starting value
  - m is ending value
  - s is value to add to n until n >= m, non inclusive
  - in the future I would like to change `[n..m;s]` for any array 

## Functions

- `fun function_name(arg_name : arg_type, ...) -> return_type { ... }`, user-defined functions
  - usage: `function_name(args)`
- exposed functions usage: `@function_name(args)`

I'm following Lox's design a lot, in there (and Lua) functions are first class. They can be passed around as values to functions or as return values. I don't really have a need for it right now I believe, especially because I want to do some simple type checking. 

## Classes, or more precisely, bundles of types and functions

I don't have a need right now for it, however, if the need arises, I would like to make it similar to rust: having Structs and Implementations, instead of classes and inheritance.
