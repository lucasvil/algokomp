# Dynamic Programming 1

Basic ideas of dynamic programming:
- Split a problem into smaller subproblems such that results from one subproblem can be reused when solving other

Technique of adding a cache to avoid recomputing the same
thing over and over is called **memoization**

Fibonacci w mem $O(n)$ but bit-cost model $O(n^2)$

**Comparing top-down and bottom-up**
- In both, have to come up with a recurrence relation that says how to solve a subproblem in terms of smaller subproblems
- Top-down coding is completely mechanical from recurrence. For bottom-up we may have to think about which order to compute the results.
- The bottom-up code is often a bit simpler and faster
- Top-down only solves the subproblems that are needed, bottom-up solves all subproblems. In more complicated cases than the ones we have seen so far, can be a big difference.
- Exist (rare) cases which actually cannot be solved top-down without a lot of extra work, and where bottom-up is needed

**Characteristics of dynamic programming**
- A problem is amenable to dynamic program if we can define a set of subproblems such that:
    1. The number of different subproblems is as small as possible.
    2. There is some ordering of subproblems from “small” to “large”
    3. The value of a subproblem can be efficiently computed given the values of some set of smaller subproblems
    