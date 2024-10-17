## General info
* anything enclosed in `()` is a comment
* anything enclosed in `~~` is a string
* anything prefaced by `.` is an operation 
* anything else is an integer
* words are separated by spaces or newlines; tabs are NOT SUPPORTED

## Supported operations

### Operators on integers
For any operator where order matters, eg. subtraction, multiplication, etc., we
always use the top of the stack as the "second" number. IE. if the stack
contains `a b` where `b` is the top of the stack, `.-` will push `a - b`. For
example `3 2 .-` will push the result `1` to the stack; not `-1`.

* `.+` pops two numbers from the stack and pushes their sum
* `.-` pops two numbers from the stack and pushes their difference
* `.*` pops two numbers from the stack and pushes their multiple
* `./` pops two numbers from the stack and pushes the quotient of the second
  divided by the first.
* `.mod` pops two numbers from the stack and pushes the remainder of the second
  divided by the first.
* `.=?` pops two numbers from the stack and pushes `1` if they are equal, `0`
  otherwise
* `.>?` pops two numbers from the stack and pushes `1` if the second one is
  larger than the first one, `0` otherwise

### Stack operations
* `.dup` push a copy of the top word 
* `.swap` swaps the two top values on the stack 

### Control flow operations
* `.cjump` pops two numbers off the stack. if the second number is not `0` the
  program jumps by as many words as the first number indicates. eg. in 
  `1 -3 .cjump` the number `1` indicates that a jump should happen and the
  number `-3` indicates that the program should jump three words back. If the
  program for instance is `... 5 6 .* 1 -3 .cjump` we will jump back to the
  word `.*`.

### I/O operations
* `.print` pop a word from the stack and print it to terminal
* `.newline` output a newline character to terminal; does not alter the stack.

