**Note that being accepted by Kattis is not enough to pass the lab, your work has to be approved by a lab assistant. You also need to register for the course on Kattis before presenting the lab.**

### Lab 1: Dynamic Programming

_Lab 1 is a mandatory part of the course._

At the bottom of this page there are several theory questions related to the lab. _It is optional to solve the theory questions, but thinking about them will be helpful for solving the lab. If you present these no later than the theory bonus deadline for the lab, **and** also present the lab itself by the programming bonus deadline, then you obtain a bonus point, counting as one solved problem, on the Time Complexity part of the written exam. See [front page](https://canvas.kth.se/courses/44839/pages/dd2352-algorithms-and-complexity-spring-2022?wrap=1 "DD2352 Algorithms and Complexity, Spring 2022") for dates of these two bonus deadlines._

Note that you need to finish all the tasks 1(a)-1(e) and 2(a) to 2(d) below before you can present the programming part of the lab, you cannot make a partial presentation when you are half-done.

### About Timing and Estimating Time Complexities

In all problems below which ask you to find the value of _n_ for which your program takes 1 second to run (1b, 1d, 2b, 2d), you do not need to hit _exactly_ 1 second, any running time of approximately 1 second is fine, but it shouldn't be, say, less than 0.25 seconds, or more than 2 seconds.

For these tasks you are also asked to measure the running time for larger and larger inputs.  In some cases, the inputs may be so large that the program will not finish in any reasonable time. It is perfectly fine to put a cut-off of, say, a minute and consider this a time-out, you do not have to wait unreasonably long for the program to finish just to get an exact number.

When estimating time complexities in this lab, you need to keep in mind that there is a big difference between different exponential growths, for instance the function 4n4^n grows a lot faster than 1.1n1.1^n even though both are exponential.  So if you have a time complexity of the form Θ(cn)\\Theta\\left(c^n\\right) for some constant _c_, try to give an estimate what _c_ is based on your measurements, do not just assume _c=_2 or something like this without checking that it fits your data.

### Using a Linux environment

Depending on which programming language you use, **you will probably need to work in a Linux system for some of these tasks,** in order to avoid your program crashing due to running out of memory (e.g. getting MemoryError in Python or segmentation fault in C++).  This is typically caused by the stack size of your environment, which in Linux can be increased by running

```ulimit -s unlimited```

in the terminal before running your program (you only have to run it once, it will effect all subsequent commands run in that terminal).

If you do not have Linux on your own computer you can run your program in the Linux environment at KTH, which you can access either directly in a computer room, or remotely over SSH (see [https://intra.kth.se/en/it/arbeta-pa-distans/unix/ssh-1.971102](https://intra.kth.se/en/it/arbeta-pa-distans/unix/ssh-1.971102)).

If you are a **Windows** user, it should be possible to instead use the Windows Subsystem for Linux (WSL) which you might find a more convenient option.  On **MacOS** it does not seem to be possible to increase the stack space as described above, so in this case you have to resort to running the program in the Linux environment at KTH as described above.)

If you are using **Java,** the above information does not apply because Java manages its own stack rather than using the one in the operating system.  When running Java, you can pass the flag \-Xss:64m to set the stack size to 64 MB (which should be enough for your solutions on this lab; if it is not you can just increase it a bit more).

### The Kattis System

Your implementations are tested by sending your solution to the [Kattis Links to an external site.](https://kth.kattis.com/) system. To complete the lab your work must be approved by Kattis _and_ a lab assistant. Start by logging into Kattis and enroll in the [algokomp24 course Links to an external site.](https://kth.kattis.com/courses/DD2352/algokomp24).

Your solutions must follow the input and output formats described on Kattis. You can in principle use any of the programming languages available on Kattis, but in order to do some of the subtasks below it must be a language that has for-loops.

If you have trouble getting your code accepted by Kattis, a hint about the test case your code fails on can be found on the page for your submission, making it easier to test locally. Also, you may want to look at the Kattis help page for the language of your choice and make sure to use the same compiler flags and if possible the same compiler/interpreter version as is used on Kattis.

### Some practical advice

*   This lab requires you to write a number of small programs, implementing similar but slightly different algorithms for solving the same problem. When presenting the lab, you need to be able to show all the different programs written, so when moving from one implementation to another in each task, make sure to make a new program and do not just update the old one.
*   If you use Python, use the "[sys.setrecursionlimit() Links to an external site.](https://docs.python.org/3/library/sys.html#sys.setrecursionlimit)" function to increase the recursion limit of your program, otherwise it will crash with a runtime error for larger inputs.
*   The second task below involves floating-point computations. If you use C/C++ or Java where there are both single-precision (float) and double-precision (double) data types for real values, use double to ensure that your results have enough precision.
*   Several tasks below involve timing how fast your implementation is.  In a Linux terminal, this can conveniently be done using the "time" command. For example, if you have a Python program for task 1(a) below called 1a.py and a text file "test.in" containing an input on which you want to time it, you can run
    
    time python 1a.py < test.in
    
    in a terminal.
*   In Python there is a decorator for automatically adding memoization to any function ([functools.cache Links to an external site.](https://docs.python.org/3/library/functools.html#functools.cache "Link")).  This is a nice feature to know of and have in your toolbox when writing dynamic programming algorithms in general, but **in this lab, you need to implement the memoization yourself from scratch** and cannot use this feature.

### Task 1: Coin Change

We first look at a basic coin changing problem which can be solved using dynamic programming. The problem is described in [kth.algokomp.coinchange Links to an external site.](https://kth.kattis.com/courses/DD2352/algokomp24/assignments/hnrhei/problems/kth.algokomp.coinchange). Read through the problem description there and make sure you understand what the problem is asking for.

A recurrence that describes the solution to the problem is the following:

$$\operatorname{Coins}(n) = \begin{cases} \infty& \text{if $n < 0$} \\ 0 & \text{if $n = 0$} \\ \min(n, 1 + \operatorname{Coins}(n-a), 1 + \operatorname{Coins}(n-b), 1 + \operatorname{Coins}(n-c)) & \text{otherwise} \end{cases}$$

**(a)** Write a direct recursive implementation (without any dynamic programming component) of this recurrence.

Submit this implementation to the Kattis problem. It will likely get a "Time Limit Exceeded Result". _In order to pass this part of the lab, your implementation here must pass at least the first 33 test cases on Kattis._

**(b)** Fix the values a = 5, b = 6, c = 7 (the same ones used in the second sample input on Kattis) and time your code for increasing values of n. How large instances (what value of n) can your program solve in 1 second (on your machine)? If you take that n and increment it by one at a time (i.e., try n+1, n+2, n+3 etc), how does the runtime  
change? If you instead take that n and then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does the runtime change? Tabulate and/or plot the results. Based on these experiments, what would be a reasonable estimate of the time complexity of your program?

**(c)** Make a new implementation, based on the one from (a), but where you add memoization to save computed values of the Coins() function and reuse them when they are needed again, thereby turning this into a dynamic programming solution.

Submit this implementation to the Kattis problem. _In order to pass this part of the lab, your implementation here must get an Accepted result on Kattis._

**(d)** Repeat subtask (b) but on the dynamic programming solution from (c):  
Fix the values a = 5, b = 6, c = 7 (the same ones used in the second sample input on Kattis) and time your code for increasing values of n. How large instances (what value of n) can your program solve in 1 second (on your machine)? If you take that n and increment it by one at a time (i.e., try n+1, n+2, n+3 etc), how does the runtime change? If you instead take that n and then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does the runtime change? Tabulate and/or plot the results. Based on these experiments, what would be a reasonable estimate of the time complexity of your program?

  
**(e)** Make a bottom-up implementation of this algorithm. In other words, instead of a recursive algorithm, write an iterative algorithm with a for loop that tabulates the values of Coins() from small to large. _In order to pass this part of the lab, your implementation here must get an Accepted result on Kattis._

Compare the running time of this program with the one from (c). What differences are there, if any?

### Task 2: Winning Streaks

We now look at another problem which can be solved with dynamic programming in a few different ways. The problem we will solve is [kth.algokomp.winningstreak Links to an external site.](https://kth.kattis.com/courses/DD2352/algokomp24/assignments/hnrhei/problems/kth.algokomp.winningstreak). Read through the problem description there and make sure you understand what the problem is asking for.

**(a)** A basic recurrence that can be used to compute the answer is to define, for integers $0≤x≤n0$ and $0≤y≤k0$,

$$f(x, y) = \begin{cases}
1.0 & \text{if $y = 0$,}\\
0.0 & \text{if $x = 0$ and $y > 0$,}\\
p \cdot f(x-1, y-1) + (1-p) \cdot f(x-1, k) & \text{otherwise (if $x \ge 1$ and $y \ge 1$),}
\end{cases}$$

The answer is then given by f(n, k).  Note that the second term in the above recurrence has _k_ as the second argument, and not _y_ (so the value of _y_ is "reset" to _k_).

Implement this recurrence as a recursive algorithm and add memoization to turn it into a dynamic programming solution.

Submit your solution Kattis. It will likely get a "Time Limit Exceeded Result". _In order to pass this part of the lab, your implementation here must pass the first 22 test cases on Kattis._

  
**(b)** Test your solution with different values of n, and use the values k = n/2 and p = 0.99. How large instances (what value of n) can your program solve in 1 second (on your machine)? If you take that n and increment it by one at a time (i.e., try n+1, n+2, n+3 etc), how does the runtime change? If you instead take that n and then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does the runtime change? Tabulate and/or plot the results. Based on these experiments, what would be a reasonable estimate of the time complexity of your program?

**(c)** Another recurrence that can be used to solve the problem is to define, for an integer $0≤x≤n0$

$$g(x) = \begin{cases}
0.0 & \text{if $x < k$,}\\
p^k & \text{if $x = k$,}\\
g(x-1) + p^k \cdot (1-p) \cdot (1-g(x-k-1)) & \text{otherwise (if $x \ge k+1$),}
\end{cases}$$

The answer is then given by g(n).

Implement this recurrence as a recursive algorithm and add memoization to turn it into a dynamic programming solution.

Submit your solution to Kattis. _In order to pass this part of the lab, your solution must get the result "Accepted" on Kattis._

  
**(d)** Repeat task (b) but on the new solution from (c):  
Test your solution with different values of n, and use the values k = n/2 and p = 0.99. How large instances (what value of n) can your program solve in 1 second (on your machine)? If you take that n and increment it by one at a time (i.e., try n+1, n+2, n+3 etc), how does the runtime change? If you instead take that n and then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does the runtime change? Tabulate and/or plot the results. Based on these experiments, what would be a reasonable estimate of the time complexity of your program?

### Theory Questions:

1.  Simulate what the solution to 1(a) will do on the input n=15, a=5, b=6, c=7: draw the tree of recursive calls that the recurrence gives rise to.
2.  How will the recursive calls made in the memoized version in 1(c) differ from in this case (n=15, a=5, b=6, c=7)?
3.  Study the recurrence Coins(n) in problem 1. Why is this a correct recurrence that solves the problem?
4.  Suppose we were trying to solve a variant of the coin change problem where we only had silver, gold and platinum coins at our disposal, no copper coins.  How would the recurrence describing the answer to the problem change in this variant?
5.  Study the function f(x, y) defined in subproblem 2(a). What does the value of f(x, y) represent, in words? Argue why computing f(n, k) gives a solution to the problem.
6.  Of the five different programs that you will write (in problems 1(a), 1(c), 1(e), 2(a), and 2(c)), which ones will have exponential time complexity, and which ones will have polynomial time complexity?
7.  Of the ones with polynomial time complexity, which ones will have linear time complexity?  
      
    
8.  _**Bonus question.** You do not need to answer this question to obtain the theory bonus point, but if you want to understand why the two different recurrences in Task 2 yield the same answer, this question guides you to that.  
    _Consider the two functions f(x, y) and g(x) defined in subproblems 2(a) and 2(b). Prove using induction on x that f(x, k) = g(x) for all x. This can be a little challenging, so here are hints about how to proceed:  
    *   It can be helpful to define an intermediate function h(x), which has the same base cases as g(x) but in the inductive case (when x >= k+1) is defined by h(x)\=pk+∑i\=0k−1(1−p)⋅pi⋅h(x−i−1)h(x)= p^k + \\sum\_{i=0}^{k-1} (1-p) \\cdot p^i \\cdot h(x-i-1) 
    *   Prove that h(x) = f(x, k) for all x using induction.  This is conceptually not difficult but involves somewhat tedious calculations: we take the recursive definition of f(x, k) and expand it, getting terms involving f(x-1, k-1) and f(x-1, k).  For the terms of the form f(x-1, k) we use the inductive hypothesis to identify them with h(x-1), and for the terms of the form f(x-1, k-1) we again expand using the recursive definition getting terms involving f(x-2, k-2) and f(x-2, k).  We continue in the same way: whenever we get terms of the form f(x', k) we replace them by h(x'), and then we will also have terms of the form f(x-i, k-i) which we keep expanding using the recursive definition until we get f(x-k, 0) which is a base case and can replaced by a constant.
    *   After this the remaining thing to prove is that h(x) also equals g(x). Here there is a nice "trick" that can make things work out in a nice way, which is to look at the quantity h(x)−p⋅h(x−1)h(x)-p \\cdot h(x-1), expand both terms using the recurrence for h(x), and simplify the resulting expression. The resulting identity should then lead towards the conclusion that in fact h(x)=g(x).