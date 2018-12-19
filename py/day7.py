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
rootbkp = sorted(root)
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

# Part 1 Solution
print("".join(resultPath))

# ======================== Part 2 =================
root = rootbkp
for n in root:
    parentMap[n].append("root-thala")

resultPath = []
resultPath.append("root-thala")
queue = []
queue.extend(root)
queue.sort()

time = -1
noOfWorkers = 5
workerTimers = [0] * noOfWorkers
workerProcess = [None] * noOfWorkers

while True:
    time += 1
    for idx in range(noOfWorkers):
        if workerTimers[idx] == 0 and workerProcess[idx] != None:
            completedWork = workerProcess[idx]
            resultPath.append(completedWork)
            workerProcess[idx] = None
    
    for idx in range(noOfWorkers):
        if workerProcess[idx] == None and len(queue) > 0:
            for j, currentNode in enumerate(queue):
                if all(elem in resultPath for elem in parentMap[currentNode]) and currentNode not in resultPath and currentNode not in workerProcess:
                    queue.pop(j)
                    workerProcess[idx] = currentNode
                    workerTimers[idx] = ord(currentNode) - 65 + 1 + 60
                    queue.extend(nodeMap[currentNode])
                    queue.sort()
                    break

    breakLoop = True
    for idx in range(noOfWorkers):
        if workerTimers[idx] > 0:
            breakLoop = False
            workerTimers[idx] -= 1
    
    strv = ""
    for w in  workerProcess:
        strv += "-- " + (w or 'None') + " --"
    # print("sec: ", time, " worker1: ", workerProcess[0], " worker2: ", workerProcess[1], " result: ", resultPath, " workers: ", "", " timers: ", "")
    # print("sec: ", time, strv, " ----", queue)
    if breakLoop:
        break

print("Total time is ", time) 
# print(root)
# print(''.join(resultPath))
# print(nodeMap)