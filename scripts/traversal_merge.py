#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on 2/21/22

@author: moinSebi

"""

import argparse
import logging

logging.basicConfig(format='%(asctime)s %(message)s', datefmt='%m/%d/%Y %I:%M:%S %p:')
logging.getLogger().setLevel(logging.INFO)
"""
**Script description**

Merging unique traversals of the same bubble
Input: Uniq traversal file from gSV -u and the corresponding GFA

Output: 
    - File with the follwing stuff
        - 1. Bubbleid 
        - 2. Group id 
        - 3. Traversals 



"""




def readTrans(ftrans):
    """

    Args:
        ftrans: file trans uniq input (s. gSV output)

    Returns:
        t: dict(bubbleid -> [[traversalid, "path"[node+,node2-]]
    """

    logging.info("Read trans file")
    t = dict()
    with open(ftrans) as file:
        for lines in file.readlines():
            lsplit = lines.split()
            if lsplit[0] in t:
                t[lsplit[0]].append([lsplit[1], lsplit[2]])
            else:
                t[lsplit[0]] = [[lsplit[1], lsplit[2]]]
    return t

def readGFA(fgfa):
    """

    Args:
        fgfa: file gfa input

    Returns:
        edges: Edges of the graph (Set((id1, dir1, id2, dir2))
        nodes: Nodes (Dict (id -> Seq)
        paths: Paths (Dict (name -> List)

    """
    logging.info("Read gfa file")
    # Read node, edge and path information
    edges = set()
    paths = dict()
    nodes = dict()
    with open(fgfa) as file:
        for x in file.readlines():
            if x.startswith("L"):
                l = x.split()
                o = (l[1], l[2], l[3], l[4])
                edges.add(o)
            elif x.startswith("S"):
                l = x.split()
                nodes[l[1]] = l[2]
            elif x.startswith("P"):

                l = x.split()
                if not l[1].startswith("Con"):
                    paths[l[1]] = [x for x in l[2].split(",")]
    return nodes, edges, paths

def getLen(s, nodes):
    """

    Args:
        s: tpath (transpath)
        nodes: nodes (from GFA)

    Returns:
        ll: Sum of all nodes in path
    """
    sp = s.split(",")
    spp = [x[:-1] for x in sp]
    if len(spp) == 0 or sp == [""]:
        return 0
    ll = 0
    for x in spp:
        ll += len(nodes[x])
    return ll


def inter(l1, l2):
    """
    Intersection of two paths
    Returns: Length of intersection, path1, path2
    """
    ls1 = set(l1.split(","))
    ls2 = set(l2.split(","))
    o = ls1.intersection(ls2)
    return [len(o), len(ls1), len(ls2)]


def inter2(l1, l2, nodes):
    """
    Intersection of two paths
    Returns: Length of intersection, path1, path2
    """
    ls1 = set(l1.split(","))
    ls2 = set(l2.split(","))
    o = ls1.intersection(ls2)
    ls1priv = getLen(",".join(list(ls1 - o)), nodes)
    ls1p = getLen(",".join(list(ls1)), nodes)

    ls2priv = getLen(",".join(list(ls2 - o)), nodes)
    ls2p = getLen(",".join(list(ls2)), nodes)
    olen = getLen(",".join(list(o)), nodes)
    return [ls1priv, ls1p, ls2priv, ls2p, olen]

def check1(t, nodes, intersection, diff):
    """

    Args:
        t: trans input (s. above)
        nodes: nodes (gfa)
        intersection: Fraction of intersection
        diff: bp difference

   Help:

   Returns:
        bubble, group, all traversal (first is the representative)
    """
    logging.info("Clustering")

    result = dict()
    count = 0
    for bubble_id, traverals in t.items():
        groups = dict()
        # print([v[1]+[getLen(v[0][1], nodes)]])
        groups[0] = [traverals[0] + [getLen(traverals[0][1], nodes)]]
        group_count = 1
        for x in traverals[1:]:
            ll = getLen(x[1], nodes)
            id = -1
            trig = False;
            for group_id, traverals in groups.items():
                if traverals[0][2] + traverals[0][2] * (1+diff) > ll > traverals[0][2] - traverals[0][2] * (1+diff):
                    node_intersection = inter(x[1], traverals[0][1])
                    if node_intersection[0] != 0:
                        if node_intersection[1] / node_intersection[0] > 1-intersection and node_intersection[2] / node_intersection[0] > 1-intersection and node_intersection[1] / node_intersection[0] < 1+intersection and node_intersection[2] / node_intersection[0] < 1+intersection:
                            if ll > traverals[0][2]:
                                trig = True
                            id = group_id
                            break
            if id == -1:
                groups[group_count] = [x + [getLen(x[1], nodes)]]
                group_count += 1
            else:
                if trig:
                    groups[id].insert(0, x + [getLen(x[1], nodes)])
                else:
                    groups[id].append(x + [getLen(x[1], nodes)])

        result[bubble_id] = groups
        count += 1;
    return result

def check2(t, nodes, intersection):
    """

    Args:
        t: trans input (s. above)
        nodes: nodes (gfa)
        intersection: Fraction of intersection
        diff: bp difference

   Help:

   Returns:
        bubble, group, all traversal (first is the representative)
    """
    logging.info("Clustering")

    result = dict()
    count = 0
    for bubble_id, traverals in t.items():
        groups = dict()
        # print([v[1]+[getLen(v[0][1], nodes)]])
        groups[0] = [traverals[0] + [getLen(traverals[0][1], nodes)]]
        group_count = 1
        for x in traverals[1:]:
            id = -1
            trig = False;
            for group_id, trav in groups.items():
                glen = getLen(x[1], nodes)
                node_intersection = inter2(x[1], trav[0][1], nodes)
                if (node_intersection[0]/node_intersection[1] < intersection) and (node_intersection[2]/node_intersection[3] < intersection):
                    if glen > trav[0][2]:
                        trig = True
                    id = group_id
                    break
            if id == -1:
                groups[group_count] = [x + [getLen(x[1], nodes)]]
                group_count += 1
            else:
                if trig:
                    groups[id].insert(0, x + [getLen(x[1], nodes)])
                else:
                    groups[id].append(x + [getLen(x[1], nodes)])

        result[bubble_id] = groups
        count += 1;
    return result

def add_stats(data, nodes):
    logging.info("Add stats")

    # bubble_id, dict -> groups

    for k,v in data.items():

        # group -> intervals (id, nodes, len)
        for key, val in v.items():
            j = []
            for x in val:
                node_intersection = inter2(x[1], val[0][1], nodes)
                j.append(node_intersection[0]/node_intersection[1])
                x.append(node_intersection[0]/node_intersection[1])



def some_stats(data, stats) -> None:
    c = 0
    for k, v in data.items():
        c += len(v)
    c1 = 0
    for k, v in stats.items():
        c1 += len(v);
    logging.info("Number of traversals: {}".format(c1))
    logging.info("Number of bubbles: {}".format(len(data)))
    logging.info("Number of clusters: {}".format(c))

def writeFile(fout, data):
    """

    Args:
        fout: file output name
        data:

    Returns:
        - file
    """

    logging.info("Write files")
    with open(fout, "w") as file:
        for k, v in data.items():
            for k2, v2 in v.items():
                na = [x[0] for x in v2]
                file.write("{}\t{}\t{}\t{}\n".format(k, k2, v2[0][1], ",".join(na)))



def writeFile2(fout, data):
    """

    Args:
        fout: file output name
        data:

    Returns:
        - file
    """

    logging.info("Write files2")
    with open(fout, "w") as file:
        for k, v in data.items():
            for k2, v2 in v.items():
                na = [str(x[0]) for x in v2]
                na2 = [str(x[3]) for x in v2]
                file.write("{}\t{}\t{}\t{}\t{}\n".format(k, k2, v2[0][1], ",".join(na), ",".join(na2)))


if __name__ == "__main__":

    parser = argparse.ArgumentParser()
    parser.add_argument("-g", "--gfa", help="gfa file", required=True)
    parser.add_argument("-u", "--uniq", help = "*traversal.unique.bubble.bed file ", required=True)
    parser.add_argument("-o", "--out", help = "output file name ", required=True)
    parser.add_argument("-i", "--intersection", help = "Fraction of private/total sequence", default=0.3)
    args = parser.parse_args()



    nodes, edges, paths = readGFA(args.gfa)
    stats = readTrans(args.uniq)

    interf = float(args.intersection)

    result = check2(stats, nodes, interf)
    add_stats(result, nodes)
    some_stats(result, stats)
    writeFile2(args.out, result)