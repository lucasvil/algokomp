**Time complexity notation**
- **$O(n)$**
    - For two functunis $f(n)$ and $g(n)$, we say $f(n)=O(g(n))$ if there exists two constants $c$ and $n_0$ such that $f(n)\leq c*g(n)$ for all $n\geq n_0$
    - *in words*: for all sufficiently large $n$, $f(n)$ is at most a constant factor larger than $g(n)$
    - *intuitively*: $f(n)$  does not grow faster than $g(n)$

- **$\Omega (n)$**
    - We say $f(n)=\Omega(g(n))$ if $g(n)=O(f(n))$
    - "$f(n)$ grows *at least* as fast as $g(n)$"

- **$\Theta (n)$**
    - We say $f(n)=\Theta(g(n))$ if $f(n)=O(g(n))$ and $g(n)=O(f(n))$
    - "$f(n)$ and $g(n)$ grow *equally fast*"

**Comparing time complexities**
- To compare the growth rates of two functions $f(n)$ and $g(n)$, we can compute how their ratio ${f(n)}/{g(n)}$ behaves in the limit as $n\to \infty$
- typically three possibilities:
    - ratio tends to 0:
        - means $g(n)$ grows faster than $f(n)$
        - so $f(n)=O(g(n))$ but $g(n)\neq O(f(n))$
    - ratio tends to $\infty$:
        - means $f(n)$ grows faster than $g(n)$
        - so $g(n)=O(f(n))$ but $f(n)\neq O(g(n))$
    - ratio tends to some positive finite value:
        - means $f(n)$ and $g(n)$ grow equally fast
        - so $f(n)=O(g(n))$ and $g(n)=O(f(n))$

**Common running time growths (ascending order)**
- Logarithmic: $O(log(n))$
- Linear: $O(n)$
- Near-linear: $O(nlog^c(n))$ for some constant $c>0$
- Quadratic: $O(n^2)$
- Cubic: $O(n^3)$
- Polynomial: $O(n^c)$ for some constant $c>0$
- Exponential: $O(c^n)$ for some constant $c>1$

*General rule of thumb*: logarithms << polynomials << exponentials