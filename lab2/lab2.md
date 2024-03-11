# Lab 2 (Programming) - NP-completeness reductions

- Due Apr 22 by 10am
- Points 1

**Note that being accepted by Kattis is not enough to pass the lab, your code has to be approved by a lab assistant too. You also need to [register for the course on Kattis Links to an external site.](https://kth.kattis.com/courses/DD2352/algokomp24) before presenting the lab.**

## Lab 2: NP-completeness reductions

At the bottom of this page there are several theory questions related to the lab. \_It is optional to solve the theory questions, but thinking about them will be helpful for solving the lab. If you present these no later than the theory bonus deadline for the lab, **and** also present the lab itself by the programming bonus deadline, then you obtain a bonus point, counting as one solved problem, on the Reductions part of the written exam.  See [front page](https://canvas.kth.se/courses/37557/ "DD2352 Algorithms and Complexity, Spring 2022") for dates of these two bonus deadlines.

### The Casting Problem

The person responsible for the casting in a film company needs to pair the right actor with the right roles. An actor may play multiple roles, but each role can only be played by one actor. The script specifies the roles that play in the same scenes. No monologues are allowed. Each actor may have only one role in each scene.

In addition the divas $p_1$ and $p_2$ are guaranteed roles in every casting. This means extra complexity, because the two do not tolerate each other, so the roles have to be cast in a way such that they never play against each other. The _casting problem_ is to determine (YES/NO) whether all roles can be cast using the available actors.

Thus the input parameters in the problem are:

- Roles $r_1, r_2,... , r_n$
- Actors $p_1, p_2,... ,p_k$
- Constraints of type 1 (one for each role). Example: $r_3$ can be played by $p_1, p_2, p_6$
- Constraints of type 2 (one for each scene). Example: in $s_4$ the roles $r_1, r_3, r_5, r_6$ and $r_7$ play

#### Input format

The first three lines of input consists of three integers, one per line: $n$, $s$ and $k$ (number of roles, number of scenes and number of actors, $n$ ≥ 2, $s$ ≥ 1, $k$ ≥ 2).  
The following $n$ lines represent the constraints of type 1 and begin with an integer indicating the number of subsequent integers on the line, followed by the numbers of the possible actors (between 1 and $k$, in boldface in the examples below).  
The last $s$ lines represent constraints of type 2. Each line begins with an integer indicating the number of subsequent integers on the line, followed by integers representing the different roles playing in that scene. Each role appears at most once in each row, so the number of roles is between 2 and $n$ (1 is not possible since monologues are not allowed).  Every role plays in at least one of the $s$ scenes.

Question: Can the roles be cast using some or all of the given $k$ actors so that $p_1$ and $p_2$ participate but not in the same scenes as each other?

Examples of valid input

**NO-instance:**

<pre>
5
5
3
3 <b>1 2 3</b>
2 <b>2 3</b>
2 <b>1 3</b>
1 <b>2</b>
3 <b>1 2 3</b>
2 1 2
2 1 2
3 1 3 4
2 3 5
3 2 3 5
</pre>

**YES-instance:**

<pre>
6
5
4
3 <b>1 3 4</b>
2 <b>2 3</b>
2 <b>1 3</b>
1 <b>2</b>
4 <b>1 2 3 4</b>
2 <b>1 4</b>
3 1 2 6
3 2 3 5
3 2 4 6
3 2 3 6
2 1 6
</pre>

### Assignment

In this lab assignment the objective is to show that the casting problem is NP-hard by a reduction involving an already known NP-complete problem. Your reduced instance will be tested and solved by Kattis. You can choose between reducing the following two problems: Graph Coloring ([kth.adk.npreduction1 Links to an external site.](https://kth.kattis.com/courses/DD2352/algokomp24/assignments/hnrhei/problems/kth.adk.npreduction1)) and Hamiltonian Cycle ([kth.adk.npreduction2 Links to an external site.](https://kth.kattis.com/courses/DD2352/algokomp24/assignments/hnrhei/problems/kth.adk.npreduction2)). The input format for these problems are defined below.

Your task thus will be to implement the transformation part of a (Karp) reduction, not to solve the casting problem!  In other words you need to take as input an instance of one problem A, transform this into an instance of another problem B (such that the answer yes/no is preserved, and running in polynomial time) and output the transformed instance.  Note that what problems A and B are is intentionally not specified here.  It is a part of the lab assignment to understand the concept of NP-completeness well enough to figure this out.

Kattis will check whether your solution is correct, but you of course need to be able to prove it to be correct at the lab examination. The purpose of the answers of Kattis is to guide you in your work with the proof and point out if you forget an important special case. During the lab examination the teaching assistant will ask you why the problem is in NP and what the complexity of your reduction is.

Kattis uses a solver for (another) NP-complete problem in order to test the correctness of your reduction. For technical reasons Kattis has a maximal allowed size of the instances. Kattis will tell you if you send too large an instance. If you can prove the correctness of your reduction you may be examined on the lab even if Kattis has not been able to check it completely.

#### GRAPH COLORING

Input: An undirected graph and a number of colors _m_. Isolated vertices and double edges may exist, but no loops.

Question: Can the vertices of the graph be colored using at most _m_ colors such that no neighbors have the same color?

Input format:  
Line one: integer V (number of vertices, $0\leq V \leq 300$)   
Line two: integer E (number of edges, $0\leq E\leq 25000$)  
Line three: goal _m_ (max number of colors, $0\leq m \leq 2^{30}$)  
One line for each of the E edges, consisting of the two endpoints of the edge (the vertices are numbered from 1 to V)

#### HAMILTONIAN CYCLE

Input: A directed graph.

Question: Is there a tour following edges in the graph and beginning and ending in the same vertex, passing each vertex in the graph exactly once?

Input format:  
Line one: integer V (number of vertices, $0\leq V \leq 200$)  
Line two: integer E (number of edges, $0\leq E\leq 5000$)  
One line for each of the E edges, consisting of the start vertex and end vertex of the edge (the vertices are numbered from 1 to V)

## Theory exercises

1.  Write, in some notation of your choice, a solution to the yes-instance of the casting problem in the example above.
2.  Show that the casting problem is in NP.
3.  Suppose we want to modify the no-instance above into a yes-instance, by adding a few actors. How many actors do we need to add in this case?  (Assume every actor we add can play every role.)
4.  Which is the smallest possible production that satisfies all input constraints for the casting problem and is possible to stage (i.e., the smallest possible YES-instance)? Specify the input for this production.
5.  Imagine an instance where the roles are divided into two groups, and where each role never occurs in a scene together with another role from the same group.  In other words the input has a structure similar to a bipartite graph. Suppose further that every actor can play every role.  How many actors will be sufficient in this case?
6.  Suppose that an instance A contains a scene with the roles 4, 7 and 12, while the instance B has three scenes with the roles 4 and 7, 7 and 12, 4 and 12. If all other constraints are identical between the instances, will the solutions be identical? Why/why not?
