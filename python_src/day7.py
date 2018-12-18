import re
from collections import defaultdict

contents = open("inputs/day7.inputs1.txt").read().split("\n")
nodeMap = defaultdict(list)
parentMap = defaultdict(list)
for line in contents:
    match = re.search(r'Step ([A-Z]) must be finished before step ([A-Z]) can begin.', line)
    parent, child = '', ''
    if match:
        parent = match.group(1)
        child = match.group(2)
    else:
        print("No regex match for ", line)
    nodeMap[parent].append(child)
    parentMap[child].append(parent)

root = set([])
for k,v in nodeMap.items():
    if len(parentMap[k]) == 0:
        root.add(k)

root = sorted(root)
resultPath = []
resultPath.append(root.pop(0))
queue = []
queue.extend(root)
queue.extend(nodeMap[resultPath[0]])
queue.sort()

while len(queue) > 0:
    queue.sort()
    currentNode = queue.pop(0)
    if all(elem in resultPath for elem in parentMap[currentNode]) and currentNode not in resultPath:
        resultPath.append(currentNode)
        queue.extend(nodeMap[currentNode])

print("".join(resultPath))

    

    
