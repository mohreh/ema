# ema Programming Language

This project implements an abstract syntax tree (AST) interpreter for a dynamic programming language called ema. The language has an s-expression syntax similar to Lisp, and semantically resembles JavaScript and Python.

## Features

- S-expression syntax for code representation
- Functional programming paradigm (all functions are first-class closures)
- Lexical scoping rules for static variable binding
- Object-oriented programming with class-based inheritance
- Everything is an expression that evaluates to a value
- Implicit returns from code blocks (last expression value bubbles up)
- First-class lambda functions and immediately invoked function expressions
- Namespaces and modules for organization
- REPL (Read-Eval-Print-Loop) for interactive programming

## Installation

Install the language compiler from the Git repository:

```bash
cargo install --git https://github.com/mohreh/ema.git
```

Or clone locally and install:

```bash
git clone https://github.com/mohreh/ema.git
cd ema
cargo install --path .
```

### Run an ema program by passing the file path:

```bash
ema main.ema
```

Sample ema code is provided in the ema_example directory. Run an example:

```bash
ema ema_example/<name>.ema
```

## REPL Usage

[![asciicast](https://asciinema.org/a/mBBZElKinHP5G6CedCIa1JNtx.svg)](https://asciinema.org/a/mBBZElKinHP5G6CedCIa1JNtx)

## Language Examples

Most syntax examples below have corresponding code in the ema_example directory.

#### Basics:

See _basics.ema_
Code blocks are created with the _begin_ keyword. _var_ declares variables, _set_ assigns to them. Variables in inner scopes are distinct from outer scopes. Mutating a variable in an inner scope does affect the outer scope value.

```scheme
(begin
    (var x 10)
    (var y (+ 10 2))
    (print "before change x = " x)

    (begin
        (var y 5)
        (set x 100)
        (print "inner block y = " y)
    )

    (print "outer block y = " y)
    (print "after change x = " x)
)
```

Since everything is an expression, variables can be declared and assigned in a single expression. The block value bubbles up.

```scheme
(begin
    (var x (begin
        (var z 10)
        (var y 20)
        (+ z y)
    ))
    (print x)
)
```

_var_ and _set_ evaluate to the value they operate on. Empty blocks and _print_ evaluate to nil.

#### Define Functions:

See _define_functions.ema_

```scheme
(begin
    (var square (lambda x (* x x)))
    (print "5 ^ 2 = " (square 5))

    (def abs (val)
        (if (< val 0)
            (- 0 val)
            val
        )
    )

    (def sum (x y z)
        (+ (+ x y) z)
    )

    (print "abs(-10) = " (abs -10))
    (print "sum(2, 3, 5) = " (sum 2 3 5))
)
```

##### Callbacks and immediately invoked lambda expressions:

```scheme
(begin
    (def on_click (callback x y)
        (callback (+ x y))
    )

    (print "callback result = " (on_click (lambda data (* data data)) 7 5))
    (print "ILLEs result = " ((lambda (x) (* x x)) 4))
)
```

#### Loops:

See _while.ema_

```scheme
(begin
    (var counter 0)
    (var result 0)
    (while (< counter 10)
        (begin
            (set result (+ result 2))
            (set counter (+ counter 1))
         )
    )
    (print "result: " result)
    (print "counter: " counter)
)
```

For loops desugar to while loops:

```scheme
(for <init>
    <condition>
    <modifier>
    <body>
)
(begin
    <init>
    (while <condition>
        (begin
            <body>
            <modifier>
        )
    )
)
```

Calculating factorials for 5:

```scheme
(begin
    (var y 1)
    (for (var x 1)
        (<= x 5)
        (++ x)
        (set y (* y x))
    )

    (print y)
)
```

#### Switch Expression:

A _switch_ expression provides multi-way branching and desugars into nested _if_ expressions:

```scheme
(switch
    (<cond1> <block2>)
    ...
    (<condN> <blockN>)
    (else <alternate>)
)
```

Each (condition expression) pair is evaluated in order. When a condition evaluates to true, its corresponding expression branch is executed and the switch returns.
For example:

```scheme
(switch
  ((> n 5) "greater than 5")
  ((< n 5) "less than 5")
  (else "equal to 5"))
```

Desugars to:

```scheme
(if (> n 5)
  "greater than 5"
  (if (< n 5)
    "less than 5"
    "equal to 5"))
```

The else clause provides a default case if no condition matches.
Switch statements allow cleaner conditional logic compared to nested ifs. Multiple possible branches can be enumerated in a declarative way.

Fibonacci implementation:

```scheme
(begin
  (def fibo (x)
      (switch ((= x 0) 0)
              ((= x 1) 1)
              (else (+ (fibo (- x 1)) (fibo (- x 2))))
      )
  )

)
```

#### Class expression:

Classes in ema allow object-oriented programming with inheritance.
A class declaration looks like:

```scheme
(class <name> <parent>
  <body>
)
```

_<name>_ is the name of the new class. _<parent>_ specifies the parent class to inherit from, or _nil_ if there is no parent.
The class body contains function definitions that become methods on instances of the class.
A special _constructor_ method will be invoked when creating new instances with the _new_ keyword:

```scheme
(var <instance_name> (new <class_name> <args>))
```

This calls the _constructor_ method, passing _self_ and the provided _args_.
Within class methods, _self_ refers to the instance. Properties can be accessed via:

```scheme
(prop self <property_name>)
```

To call a parent class method, use super:

```scheme
((prop (super <class_name>) <method_name>) self args)
```

or:

```scheme
((prop (super <instance_name>) <method_name>) self args)
```

This looks up and calls the <method_name> on the parent.

See _class.ema_
An example:
Point3D inherits from Point, calls parent constructor, defines a _z_ property and updates calc method.

```scheme
(begin
    (class Point nil
        (begin
            (def constructor (self x y)
                (begin
                    (set (prop self x) x)
                    (set (prop self y) y)
                )
            )


            (def calc (self)
                (+
                    (prop self x)
                    (prop self y)
                )
            )
        )
    )

    (class Point3D Point
        (begin
            (def constructor (self x y z)
                (begin
                    ((prop (super Point3D) constructor) self x y)
                    (set (prop self z) z)
                )
            )


            (def calc (self)
                (+
                    ((prop (super Point3D) calc) self)
                    (prop self z)
                )
            )
        )
    )

    (var p (new Point3D 10 30 50))
    (var s (new Point3D 2 3 5))

    (var pTwoD ((prop (super p) calc) p))
    (var sTwoD ((prop (super Point3D) calc) s))

    (var pThreeD ((prop p calc) p))
    (var sThreeD ((prop Point3D calc) s))

    (print "2D point (10 30) = " pTwoD)
    (print "2D point (2 3) = " sTwoD)
    (print "3D point (10 30 50) = " pThreeD)
    (print "3D point (2 3 5) = " sThreeD)
)
```

pTwoD eq to 40.
sTwoD eq to 5.
pThreeD eq to 90.
sThreeD eq to 10.

#### see linked list implementation in ema_example directory
