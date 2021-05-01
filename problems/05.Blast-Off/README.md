# Blast Off
[View problem on NZOI](https://train.nzoi.org.nz/problems/1133)

### Specifications
Memory limit: 128 megabytes

Time limit: 2.0 seconds

---
### Problem Statement
You have N friends playing a new board game which you have created, called Blast Off. In this game, the board consists of a straight path of T tiles, numbered from T−1 to 0, and each player starts off at a random tile. The objective of the game is to reach the goal, tile 0.

Players can move between tiles by using rockets, of which there are R types available. Each rocket has a certain cost, ci, and amount of fuel, f<sub>i</sub>. Each unit of fuel allows the player to move by a single tile. However, the player has no control over where they go – upon using a rocket, the player will always move fi tiles towards tile 0, and the rocket will not stop until it runs out of fuel. If there is still fuel remaining after reaching tile 0, the player is then propelled in the opposite direction, sending them backwards. For example, if a player is currently at tile 10, and uses a rocket with 15 fuel, they will move 10 tiles forwards, to tile 0, and then 5 tiles backwards, ending at tile 5. More formally, if a player is currently at tile x and uses a rocket with f fuel, they will end up at tile |x−f|. Note that the same type of rocket can be used multiple times.

For each of your N friends' starting positions, you would like to know the minimum cost required to reach and stop at the goal, tile 0. It is guaranteed that all players will be able to reach the goal.
##### Input
The first line will contain three space-separated integers, R, the number of rocket types, N, the number of players, and T, the number of tiles on the board.

The next R lines each contain two space-separated integers c<sub>i</sub>, f<sub>i</sub>, indicating that the i-th rocket costs c<sub>i</sub> and has f<sub>i</sub> fuel.

The next N lines each contain a single integer, s<sub>i</sub>, representing the starting tile of the i-th player.
##### Output
You should output N lines, each containing a single integer. The i-th of these integers is the minimum cost required for player i to reach the goal.
##### Constraints
- 1≤R≤50
- 2≤T≤10000
- 1≤N<T
- 1 ≤ f<sub>i</sub>,s<sub>i</sub> < T
- 1 ≤ c<sub>i</sub> ≤ 10000
##### Subtasks
- Subtask 1 (25%): N=1, and R=2
- Subtask 2 (25%): N=1, and c<sub>i</sub>=1 for all i.
- Subtask 3 (35%): c<sub>i</sub>=1 for all i.
- Subtask 4 (15%): No further restrictions apply.

##### Sample Explanation
In the first sample case, the optimal solution is to use rocket types 1,1,2,1, in order. This moves the player between tiles 7→3→1→4→0:

The first rocket moves the player to tile |7−4|=3.
The second rocket moves the player to tile |3−4|=1.
The third rocket moves the player to tile |1−5|=4.
The last rocket moves the player to tile |4−4|=0.
The total cost of this strategy is 1+1+2+1=5, which is the minimum possible.
##### Sample Input 1
```
2 1 20
1 4
2 5
7
```

##### Sample Output 1
```
5
```

##### Sample Input 2
```
3 1 12
1 1
1 2
1 6
10
```

##### Sample Output 2
```
3
```

##### Sample Input 3
```
4 3 25
3 4
2 6
7 10
3 15
1
17
20
```

##### Sample Output 3
```
10
8
10
```