#!/usr/bin/python3

import json
from xml.etree.ElementTree import Element,ElementTree

def read():
    file = open("a_pad.json","r")
    data1 = file.read()
    langs = json.loads(data1)['data']['language']
    write_en(langs, "new_en", "pad_en.xml")
    write_en(langs, "new_ko", "pad_ko.xml")

def gen_node(value: str, label: str):
    node:Element = Element("string")
    node.set("name", label)
    node.text = value
    return node

def write_en(langs, key: str, name: str):
    root = Element('resources')
    tree = ElementTree(root)
    length = len(langs)
    for row in range(0, length):
        lang = langs[row]
        # ko = lang['new_ko']
        value: str = lang[key]
        label: str = lang['label']
        new_project_id = lang['new_project_id']
        if value:
            if new_project_id :
                node = Element(new_project_id)
                node.append(gen_node(value, label))
                root.append(node)
            else :
                root.append(gen_node(value, label))
    indent(root, 0)
    tree.write(name,"UTF-8")

def indent(elem:Element, level=0):
    """美化写入文件的内容"""
    i = "\n" + level*"  "
    if len(elem):
        if not elem.text or not elem.text.strip():
            elem.text = i + "  "
        if not elem.tail or not elem.tail.strip():
            elem.tail = i
        for elem in elem:
            indent(elem, level+1)
        if not elem.tail or not elem.tail.strip():
            elem.tail = i
    else:
        if level and (not elem.tail or not elem.tail.strip()):
            elem.tail = i

if __name__ == "__main__":
    read()