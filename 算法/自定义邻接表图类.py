class Vertex:
    def __init__(self, name):
        self.name = name
        self.connected_vertex = {}

    def __str__(self):
        return str(self.name)  +  str([(vertex.name,str(self.connected_vertex[vertex])) for vertex in self.connected_vertex.keys()])

    def appendNeighbor(self, neighbor, weight=1):
        self.connected_vertex[neighbor] = weight

    def getNeighbors(self):
        return self.connected_vertex.keys()

    def getWeight(self, neighbor):
        return self.connected_vertex[neighbor]
    
    def getName(self):
        return self.name
# a = Vertex('↖')
# print(a)
class Graph():

    def __init__(self):
        self.ver_list = {}
        self.num_vertex = 0
    
    def appendVertex(self, name):
        new_vertex = Vertex(name)
        self.ver_list[name] = new_vertex
        self.num_vertex += 1
        return new_vertex

    def getVertex(self, name):
        if name in self.ver_list:
            return self.ver_list[name]
        else:
            return None

    def appendEdge(self, init, target, weight=1):
        if init not in self.ver_list:
            self.appendVertex(init)
        if target not in self.ver_list:
            self.appendVertex(target)
        self.ver_list[init].appendNeighbor(self.ver_list[target], weight)

    def getVertices(self):
        return self.ver_list.keys()


    def __contains__(self, name):
        return name in self.ver_list

    def __iter__(self):
        return iter(self.ver_list.values())

g = Graph()
for i in ['a','b','c','d','e','f']:
    g.appendVertex(i)
print(g.getVertices())
g.appendEdge('a','b')
g.appendEdge('a','c')
g.appendEdge('b','a')
g.appendEdge('b','c')
g.appendEdge('b','d')
g.appendEdge('c','a')
g.appendEdge('c','b')
g.appendEdge('c','d')
g.appendEdge('c','e')
g.appendEdge('d','b')
g.appendEdge('d','c')
g.appendEdge('d','e')
g.appendEdge('d','f')
g.appendEdge('e','c')
g.appendEdge('e','d')
g.appendEdge('f','d')

def Bfs(graph, init):
    queue = []
    queue.append(init)
    visit = set()
    visit.append(init)
    while len(queue):
        print(queue)
        vertex = queue.pop(0)
        vertex = graph.getVertex(vertex)
        ver_list = vertex.getNeighbors()
        # print('a',end='')
        # print([ver.getName() for ver in ver_list])
        for inner_vertex in list(ver_list):
            
            if inner_vertex.getName() not in visit:
                # print(inner_vertex.getName(), end='')
                queue.append(inner_vertex.getName())
                # print(queue)
                visit.append(inner_vertex.getName())
        # print()
        # print(vertex)
# Bfs(g, 'a')

def Dfs(graph, s='a'):
    stack = []
    stack.append(s)
    seen = []
    for i in range(len(graph.getVertex(s).getNeighbors())+1):
        seen.append(s)
    big_list = []
    while len(stack):
        vertex = stack.pop()
        big_list.append(vertex)
        nodes = graph.getVertex(vertex).getNeighbors()
        names = [ver.getName() for ver in nodes]
        if 'f' in names:
            print(big_list)
            while True:
                pop1 = seen.pop()
                if len(big_list)>1:
                    pop2 = big_list.pop()
                if pop1 == seen[len(seen)-2]:
                    break
            continue
        flag = 0
        for neighbor in names:
            # print(seen)
            if neighbor not in seen:
                flag += 1
                stack.append(neighbor)
        for i in range(flag):
            seen.append(vertex)
        if flag>1:
            seen.append(vertex)
        if flag==0:
            while True:
                pop1 = seen.pop()
                pop2 = big_list.pop()
                if pop1 == seen[len(seen)-2]:
                    break
Dfs(g,'a')















