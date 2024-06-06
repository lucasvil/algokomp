def main():
    V = int(input()) # number of vertices
    E = int(input()) # number of edges
    m = int(input()) # max number of colors
    edges = [[0]*2]*E

    # read edges
    for i in range(E):
        a, b = input().split()
        edges[i] = [int(a),int(b)]
    GC2Cast(V, E, m, edges)

# Transformation of GC input to casting problem
def GC2Cast(V, E, m, edges):
    n = V + 2 # number of roles
    s = E + V + 1 # number of scenes
    k = m + 2 # number of actors

    casting_input = f"{n}\n{s}\n{k}"
    casting_input += f"\n1 1\n1 2\n" # divas

    for _ in range(V):
        casting_input +=  f"{m}"
        for p in range(3, k+1):
            casting_input += f" {p}\n"

    # scenes (non diva roles)
    for edge in edges:
        casting_input += f"2 {edge[0]+2} {edge[1]+2}\n"

    # p1/r1: add scene with all other roles except p2 (monologues)
    for i in range(3, n+1):
        casting_input += f"2 1 {i}\n"
    
    # p2/r2: add scene with any other role except p1.
    casting_input += f"2 2 3\n"
    
    print(casting_input, end="")

if __name__ == "__main__":
    main()