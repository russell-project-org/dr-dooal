## How to use the IO driver

What it does : It reads in a file to initialise the solver.

### File format

```
Suppose you have a linear program below

min 2 * x1 + 3 * x2
such that 
x1 + x2 <= 8
0.5 * x1 + 2 * x2 >= 6
2 * x1 + 5 * x2 = 20
x1 >= 0, x1 <= 0

The file format will be based loosely on DIMACS CNF formula

c This will be the comment
c of the file. It will be ignored during parsing.
c Note that variable 'v' line must always be before objective function line
v 1 -1 <-- 1 for >=0 and -1 for <= 0, no unbounded allowed
o 2 3 <--- 'o' stands for objective function, 2 and 3 are coefficients of the linear function
e 1 1 leq 8
e 0.5 2 geq 6
e 2 5 eq 20
v 1 -1 <-- 1 for >=0 and -1 for <= 0, 0 for unbounded
f <-- Termination buffer

```

If you have any questions, please raise an issue. Thanks.