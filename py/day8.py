contents = open("inputs/day8.inputs1.txt").read().split(" ")
headers = list(map(lambda x: int(x), contents.copy()))

class Node:
    def __init__(self, name, csize, msize):
        self.name = name
        self.child = []
        self.meta = []
        self.csize = csize
        self.msize = msize
        self.metaSum = 0

id = 1
stack = []
# stack.append(Node(id, headers.pop(0), headers.pop(0)))
# id += 1

def buildTree( headers, parentNode ):
    global id
    currentNode = Node(id, headers.pop(0), headers.pop(0))
    parentNode.child.append(currentNode)
    id += 1
    if currentNode.csize == 0:
        for i in range(currentNode.msize):
            currentNode.meta.append(headers.pop(0))
    else:
        # stack.append(currentNode)
        for i in range(currentNode.csize):
            buildTree(headers, currentNode)
        for i in range(currentNode.msize):
            currentNode.meta.append(headers.pop(0))

parentNode = Node("root-thala", 1, 0)
buildTree(headers, parentNode)


result = 0
def printAll ( refNode, count):
    global result
    # print("-" * count, refNode.name, " -- ", refNode.meta)
    refNode.metaSum = sum(list(map(lambda x: int(x), refNode.meta)))
    result += refNode.metaSum
    for n in refNode.child:
        printAll(n, count+1)

printAll(parentNode, 0)
print(result)

def computePart2 ( refNode ):
    tempRes = 0
    if refNode.csize == 0:
        # print(refNode.name, " -- ", refNode.metaSum, refNode.meta)
        return refNode.metaSum
    else:
        for val in refNode.meta:
            tempRes += computePart2(refNode.child[val-1]) if refNode.csize >= val else 0
    # print(refNode.name, " -- ", tempRes)             
    return tempRes

print(computePart2(parentNode.child[0]))