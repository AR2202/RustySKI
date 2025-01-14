# RustySKI

### Interpreter for the SKI combinator calculus

https://en.wikipedia.org/wiki/SKI_combinator_calculus
## USAGE
start REPL with `cargo run`

Enter a SKI expression (a string consisting of S,K,I)

to exit, enter `quit`

```
S
K
I
please enter a SKI expression, or enter 'quit' to exit:
```
Enter a SKI expression to reduce it to a simpler SKI expression.

The combinators are applied from left to right. Parentheses are only necessary where required to change the evaluation order, but permitted anywhere.

## Examples

### Identity function

`IK` reduces to `K`

`II` reduces to `I`

`(II)K` reduces to `K` (the parentheses are not required here)

### Representing booleans 
`true` can be represented as `K`

`false` can be represented as `KI` 

`a and b` can then be represented as `a b false ` which corresponds to `a b KI`

Let's validate this representation:

`true and true` should be `true` 

`true and true` => `true true false` => `KK(KI)` => `K` => `true`

`true and false` should be `false` 

`true and false`=>`true false false`=>`K(KI)(KI)` => `KI` => `false`

`false and true` should be `false`

`false and true`=>`false true false`=>`(KI)K(KI)`=>`I(KI)`=>`KI` => `false`

`false and false` should be `false`

`false and false`=> `false false false`=>`KI(KI)(KI)`=> `KI`=> `false`

`a or b` can then be represented as `a true b ` which corresponds to `a K b`

Let's validate this representation:

`true or true` should be `true` 

`true or true` => `true true true` => `KKK` => `K` => `true`

`true or false` should be `true` 

`true or false`=>`true true false`=>`KK(KI)` => `K` => `true`

`false or true` should be `true`

`false or true`=>`false true true`=>`(KI)KK`=>`IK`=>`K` => `true`

`false or false` should be `false`

`false or false`=> `false true false`=>`KIK(KI)`=> `KI`=> `false`

`not a` can then be represented as `a false true ` which corresponds to `a (KI) K`

`not true` should be `false` 

`not true` => `true false true` => `K(KI)K` => `KI` => `false`

`a xor b` can be represented as `(a and (not b)) or ((not a) and b)` which corresponds to `(a (b (KI) K) (KI)) K ((a (KI)K) b (KI))`