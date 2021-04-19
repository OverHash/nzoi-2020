# Paddocks
[View problem on NZOI](https://train.nzoi.org.nz/problems/1120)

### Specifications
Memory limit: 64 megabytes

Time limit: 1.0 seconds

---
### Problem Statement
Plantain is a weed found on farms. If the percentage of plantain is low, the farm is considered healthy - suitable for crop growth the grazing by live stock. A farmer takes readings each year to determine the plantain saturation (as a percentage) of some paddocks on their farm. Your task is to process the plantain saturation data collected from the past three years and determine if a farm is `healthy` or `unhealthy`. If the crops are unhealthy, you must determine if the farmer should resow their land.

- If the average (arithmetic mean) plantain saturation reading is ≥12% for each of the past three years then the farm is deemed to be unhealthy.
- If the farm is unhealthy AND 50% or more of paddock readings for the most recent year have plantain saturation ≥25%, then the farmer should resow their land.
- A farm is healthy for all other situations.

##### Input
The first line will contain a single integer, N, the number of readings taken in the most recent year (1≤N≤200). The second line will contain N space-separated integers, the plantain saturation (as a percentage) for each paddock sampled that year. Each saturation reading will be an integer between 0 and 100 (inclusive).

This input format is repeated for the data collected in the two prior years. N might be different for each year.

##### Output
You should output a single line containing either `healthy`, `unhealthy`, or `resow`, based on the above criteria.

##### Subtasks
- Subtask 1 (40%): It is guaranteed that the farm will always be either `healthy` or `unhealthy`.
- Subtask 2 (60%): No further restrictions apply.

##### Sample Input 1
```
3
12 12 12
8
12 12 12 12 12 12 12 12
2
12 12
```

##### Sample Output 1
```
unhealthy
```
