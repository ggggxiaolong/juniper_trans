#!/usr/bin/python3

import json
import re

def read():
    file = open("a_pad.json","r")
    data1 = file.read()
    langs = json.loads(data1)['data']['language']
    # split_lang(langs, 'en')
    # split_lang(langs, 'ko')
    split_lang(langs, 'es')


def split_lang(langs, key):
    value_dict:dict = {}
    length = len(langs)
    for row in range(0, length):
        lang = langs[row]
        # ko = lang['new_ko']
        value: str = lang[key]
        new_value: str = lang["new_{}".format(key)]
        label: str = lang['label']
        # mode: str = lang['mode_name']
        # print(value)
        # arr = value_dict.get(mode, {})
        # arr[label] = new_value if new_value else value
        value_dict[label] = new_value if new_value else value
    
    file = open("{}.js".format(key), "w")
    json_str = json.dumps(value_dict, indent=4, ensure_ascii=False)
    # print(json_str)
    # json_str = re.sub(r'"(?P<value>\w+)":', json_pare, json_str)
    file.write("export default " + json_str)
    file.close()

def json_pare(matched):
    value = matched.group('value')
    return value + ":"

if __name__ == "__main__":
    read()