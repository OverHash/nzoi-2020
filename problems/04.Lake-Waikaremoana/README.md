# Lake Waikaremoana
[View problem on NZOI](https://train.nzoi.org.nz/problems/1125)

### Specifications
Memory limit: 128 megabytes

Time limit: 1.5 seconds

---
### Problem Statement
There is a circuit tramping track around Lake Waikaremoana. It consists of N huts numbered clockwise from 1 to N, and N trails that connect adjacent huts. The i-th trail connects hut i and the next hut clockwise (so trail 1 connects huts 1 and 2, trail 2 connects huts 2 and 3, and trail N connects huts N and 1). Trampers can pick any hut as the starting location, and walk the circuit either clockwise or anticlockwise all the way back to the starting hut. The circuit is a multi-day tramp; each day the trampers walk along one trail, then sleep at the adjoining hut.

Trampers need to carry all their food with them. They start the tramp with N units of food, and at the end of each day they eat one unit of food (so during the first day they carry N units of food, during the second day they carry N−1 units of food, and during the last day they carry 1 unit of food).

Each trail has a difficulty rating. The actual difficulty of walking the trail depends on the amount of food carried, and is given by the formula difficulty rating × amount of food carried. The overall difficulty of the tramp is the sum of the actual difficulties for walking each trail.

Naturally, we would like to pick the starting hut and the walking direction (clockwise/anticlockwise) so that the tramp is as easy as possible. Given the difficulty ratings of the trails, what is the smallest overall difficulty that can be achieved?
##### Input
The first line will consist of a single integer, N, the number of huts (1≤N≤100000).

The second line will consist of N space-separated integers, the difficulty ratings trails 1 to N. Each difficulty rating will be in the range 0 to `100000`.
##### Output
Output a single integer, the smallest overall difficulty that can be achieved by choosing the starting hut and the walking direction (clockwise/anticlockwise).

Note: the result might not fit in a 32-bit integer.
##### Subtasks
- Subtask 1 (15%): 1≤N≤1000, and it is guaranteed that the smallest overall difficulty can be achieved by starting from hut 1 and going clockwise
- Subtask 2 (15%): 1≤N≤1000, and it is guaranteed that the smallest overall difficulty can be achieved by going clockwise
- Subtask 3 (20%): 1≤N≤1000
- Subtask 4 (50%): 1≤N≤100000

##### Sample Explanation
In the first sample case, the best option is to start from hut 1 and go clockwise. During the first day, trail 1 is taken from hut 1 to hut 2 (difficulty rating: 20, amount of food carried: 4, actual difficulty: 80). The overall difficulty is `20×4 + 10×3 + 50×2 + 100×1 = 310`.

In the second sample case, the best option is to start from hut 4 and go anticlockwise. During the first day, trail 3 is taken from hut 4 to hut 3 (difficulty rating: 60). The overall difficulty is `60×5 + 70×4 + 80×3 + 90×2 + 80×1 = 1080`. This case will not appear in subtasks 1 or 2.
##### Sample Input 1
```
4
20 10 50 100
```

##### Sample Output 1
```
310
```

##### Sample Input 2
```
5
80 70 60 80 90
```

##### Sample Output 2
```
1080
```
