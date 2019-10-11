# `lambash`

Run the shell with `cargo run`.

```
\f.((\x.x) date) -R -u f

-p (λf.(((((λx.x) date) -R) -u) f))
-> (λf.(((date -R) -u) f))
-η ((date -R) -u)
Fri, 11 Oct 2019 22:28:09 +0000
```

### λ Normalization

```sh
(\x.x) 1
-p ((λx.x) 1)
-> 1
-η 1

\x.f x
-p (λx.(f x))
-> (λx.(f x))
-η f
```

A POSIX-like shell written for and using lambda calculus and it's derivatives.

```sh
# Application (foreground call).
(λx.x) echo 1
1
(\x.y) 2
y: command not found

# Abstraction (background call).
\.ping localhost

# TODO
λx.x
λx.y

```

### Next Steps

```
# This is a comment.

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
