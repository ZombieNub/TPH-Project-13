# TPH-Project-13
Implementation of TPH Project 13 with 3 algorithms.

This implements the 13th project listed under Programming Challenges (Easy) on the TPH repository: https://github.com/the-programmers-hangout/programming-resources/blob/master/ideas.md
To find a good stock, three different algorithms are used:
1) Compare each stock to every other stock and find which stocks have the maximum difference (O(n^2))
2) Sort the stocks by value and pick the lowest and highest stocks that occur sequentially (O(n * log(n))
3) Greedy algorithm contributed by discord user D_Drake#1521 (O(n))

Algorithm 3 is not properly implemented, as it has the possibiltiy to pick two stocks that don't occur sequentially. Future strategies may be implemented.
