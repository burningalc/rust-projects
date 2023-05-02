observing the pattern

n = 1

```md
1. 0
```

n = 2

```md
1. o o
2. o x
```

n = 3

```md
1. o o o
2. o x o
3. o x x
```

n = 10

```md
    1 2 3 4 5 6 7 8 9 10

1.  o o o o o o o o o o 10
2.  o x o x o x o x o x 5
3.  o x x x o o o x x x 4
4.  o x x o o o o o x x 6
5.  o x x o x o o o x o 6
6.  o x x o x x o o x o 5
7.  o x x o x x x o x o 4
8.  o x x o x x x x x o 3
9.  o x x o x x x x o o 4
10. o x x o x x x x o x 3
```

observation

1. the bulb will toggle at the (i == their factor) round
   so 1 will toggle at 1st round
   2 will toggle at 1st, 2nd round
   6 will toggle at 1st, 2nd, 3rd, 6th round
2. 1, 4, 9 are on, rest are off
3. common thing for 1, 4, 9 is that the number of their factors is odd, which is why they get is stayed on at the end
4. so the solution to solve this is by finding out how many number between 1..n has odd number of factors
5. we can find this by calculating by either finding out all the perfect square or the square root of n
