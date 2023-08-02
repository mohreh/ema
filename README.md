# ema Programming Language

This project is an ast interpreter for a dynamic programming language called ema with s-expression base syntax (like lisp) and semantically like javascript and python.

## Features

- simple syntax (s-expressions)
- functional (all functions are closures in ema)
- first class functions (static scope)
- oop support (class based) (support for inhertace)
- everything is an expression
- in every block (scope) last evaluated expression is result
- lambda functions (IILE support)
- Namespaces and modules
- repl (intractive programming environment for Read-Evaluate-Print-Loop)

## Installation

install the language from git repository:

```bash
cargo install --git https://github.com/mohreh/ema.git
```

or:

```bash
git clone https://github.com/mohreh/ema.git
cd ema
cargo install --path .
```

### Run program from file

```bash
ema main.ema
```

ema examples provided in ema_example directory. you can run code and see the result:

```bash
ema ema_example/<name>.ema
```

## Syntax Examples

almost every example of syntax below provided in ema_example directory.

#### Basics:

_see basics.ema inside ema_example directory_
We create a new block with begin keyword.
For define a variable we use "var" keyword. "set" keyword will lookup for variable and change its value.
variable y in inner scope is differ from y from outer scope. because we change value of x in inner scope value of x will change to 100.

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

Also because everything in ema language is an expression, you can define a variable and set scope result to it, value of x will be last evaluated expression inside scope.

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

Note that var and set expression will return value we try to set to a variable, print expression and empty blocks returns _nil_

#### Define Functions:

_see define_functions.ema inside ema_example directory_

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

##### callback functions and IILEs

callback result is equals to (5 + 7) ^ 2 = 144, and IILE result (Immediately-invokes lambda expression) equals to 4 ^ 2 = 16.

```scheme
(begin
    (def on_click (callback x y)
        (callback (+ x y))
    )

    (print "callback result = " (on_click (lambda data (* data data)) 7 5))
    (print "ILLEs result = " ((lambda (x) (* x x)) 4))
)
```

#### While loop:

_see while.ema inside ema_example directory_

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

#### for loop:

for loop is syntactic sugar for while loop, two below code is equvelant to each other.

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

this code calculate the result of 5!

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

#### switch expression:

```scheme
(switch
    (<cond1> <block2>)
    ...
    (<cond1> <block2>)
    (else <alternate>)
)
```

Fibonacci:

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

_see class.ema inside ema_example directory_
In the expamle. we have class call Point witch doesn't inherit from any class, and has two functions calc and constructor. constructor function will call when we create and instance with new key word _(var p (new Point 10 20))_ and it sets provided values to instance and will available with prop keyword _(prop self x)_.
The Point3D class inherit from Point class, and we can call parent methods with super keyword _((prop (super Point3D) constructor) self x y)_ we use inside Point3D calc method.
see pTwoD, sTwoD, pThreeD and sThreeD for super and prop examples:

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
sThreeS eq to 10.
