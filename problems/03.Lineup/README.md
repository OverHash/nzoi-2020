# Lineup
[View problem on NZOI](https://train.nzoi.org.nz/problems/1144)

### Specifications
Memory limit: 128 megabytes

Time limit: 1.5 seconds

---
### Problem Statement
Toddy is the new leader of his school's Rock Paper Scissors club, and tomorrow they're having a friendly competition with a rivalling school! Both clubs have exactly N players of varying skill levels, so every player will get to play a single match against some player from the opposing school.

As Toddy is a strategic mastermind, he has managed to obtain the rivalling school's secret lineup, which tells him the order in which their players will play their matches. Toddy knows the skill levels of every player in both schools, and would like to order his own school's lineup in such a way that maximises the total number of games his school will win. Of course, as Rock Paper Scissors is a game of pure skill, in any match between two players, the player with the higher skill level will always win. It is guaranteed that no two players share the same skill level.

Help Toddy find the maximum number of games his club can win, by arranging his lineup optimally.

##### Input
The first line will contain a single integer, N, the number of players in each school's club (1≤N≤100000).

The second line will contain N space-separated integers, a1,a2,…,aN, the skill levels of the players in Toddy's club, in no particular order (1≤ai,bi≤1000000).

The third line will contain N space-separated integers, b1,b2,…,bN, the skill levels of the players in the rivalling school's club, in the order that they will play their matches.

##### Output
You should output a single integer – the maximum number of games Toddy's club can win.

##### Subtasks
- Subtask 1 (50%): N≤1000
- Subtask 2 (50%): N≤100000

##### Sample Explanation
In the first sample case, Toddy's club can win at most `3` matches, by ordering his lineup as `3,8,5,1` (against his opponent's lineup of `2,6,4,7`). With this lineup, they would win the first three matches, as `3>2`, `8>6`, and `5>4`, but lose the last match, as `1<7`.

In the second sample case, Toddy's club can only win `1` match, by playing their highest skilled player (`7`) first, against the player with skill level `6`.

##### Sample Input 1
```
4
3 1 8 5
2 6 4 7
```

##### Sample Output 1
```
3
```

##### Sample Input 2
```
3
4 5 7
6 8 9
```

##### Sample Output 2
```
1
```
