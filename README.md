# `lambash`

A POSIX-like shell written for and using lambda calculus and it's derivatives.

[Introductory Paper](http://www.cs.yale.edu/homes/hudak/CS201S08/lambda.pdf)

```
# This is a comment.

# Abstraction (function)
λx.x
λx.x

# Application
λx.x 1
1

\x.x 2
2

# Definition (syntactic)
id := \x.x

# Assignment (semantic)
ipi = \x.x 3.1415

# Equality
1 ≡ 1
λx.x ≡ λx.x
ipi ≡ id 3.1415
ipi ≢ 3.1415

# Primitives
### Boolean
true = true
true ≠ false

### Number
0
1.0
3.1415
271e-2

### Character
'f'
‘β’

### String
"foo"
“Γον”

### Void / Null
null = void

### None () / Tuple
()
(1, 2)

### Array
[]
[1, 2]

# POSIX

### Path (all unbound variables will be treated as a path)
foo.rb
./folder/file.ext
/somewhere/else
https://nixpulvis.com

### Operations
write foo.rb 3.1415
()

read foo.rb
3.1415

execp cat foo.rb
3.1415
```
