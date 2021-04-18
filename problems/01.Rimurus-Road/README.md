# Rimuru's Road
[View problem on NZOI](https://train.nzoi.org.nz/problems/1113)

### Specifications
Memory limit: 64 megabytes

Time limit: 0.5 seconds

---
### Problem Statement
Rewind to the 17th century in the country of Tempest: the road from Tempest to the Sorcerer's Dynasty has just been completed. As a passionate fan of transport infrastructure projects, you start journeying from one end of the road to the other, starting from Tempest. At Tempest, which is the starting point (0km), there are three types of facilities available: a bar, an outpost, and an inn. From this starting point, there is a bar located every A kilometres on the road, an outpost every B kilometres, and an inn every C kilometres.

However, the weather's quite hot today. At some point during your journey, you lose track of how far you are from a facility, and your phone's dead too. You only know how far along the road you currently are, and the values A, B, and C from above. How far from the closest facility are you from your current point?

##### Input
The first line will contain three space-separated integers A, B, and C (1≤A,B,C≤1000000000).

The second line will contain an integer D, the distance from Tempest to your current point on the road in kilometres (0<D≤1000000000).

##### Output
The absolute distance to the closest facility in kilometres, regardless of whether it is a bar, outpost, or inn.

If there is more than one facility that is the closest and are of different types (e.g. an outpost and a bar), output this text on the line after the absolute distance: `can't make up my mind`

##### Subtasks
Subtask 1 (60%): 1≤A,B,C,D≤500

Subtask 2 (40%): 1≤A,B,C,D≤1000000000

##### Sample Explanation
In the first sample case, you are currently 29km away from Tempest. The closest bar is at the 30km mark (6 × 5km); the closest outpost is 28km (2 × 14km); the closest inn is 33km (3 × 11km). Both the bar and the outpost (two different types of facilities) are 1km away from your current location, so on the next line is the message `can't make up my mind`

In the second sample case, you are 35km away from Tempest. The closest bar is 46km away from Tempest; the two closest outposts are 30km and 40km; the closest inn is 42km. There are two of the same type of facility (two outposts) to choose from that are the closest, being 5km away from your location.

##### Sample Input 1
```
5 14 11
29
```

##### Sample Output 1
```
1
can't make up my mind
```

##### Sample Input 2
```
23 10 21
35
```

##### Sample Output 2
```
5
```