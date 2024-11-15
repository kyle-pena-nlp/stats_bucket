import json
from argparse import ArgumentParser

"""
Annoyingly, the Shank IDL generator doesn't seem to recognize f32 as a built-in type,
and emits IDL as if f32 were a custom type.
This short Python program post-processes the IDL and rewrites occurrences of f32 
so that it looks like an actual built-in primitive type.
"""


parser = ArgumentParser()
parser.add_argument("--fp", type = str)
args = parser.parse_args()

def replace_recursively(obj, thing, replacement):
    if json_matches(obj,thing):
        return replacement
    elif isinstance(obj,dict):
        for key in obj:
            obj[key] = replace_recursively(obj[key], thing, replacement)
        return obj
    elif isinstance(obj,list):
        for i,item in enumerate(obj):
            obj[i]= replace_recursively(obj[i], thing, replacement)
        return obj
    else:
        return obj

def json_matches(obj,thing):
    obj_type = type(obj)
    thing_type = type(thing)
    if obj_type != thing_type:
        return False
    if isinstance(obj_type, dict):
        return obj == thing
    elif isinstance(obj_type, list):
        return obj == thing
    else:
        return obj == thing

with open(args.fp, "r+") as f:
    idl = json.load(f)
    idl = replace_recursively(idl, { "defined": "f32"}, "f32")

with open(args.fp, "w+") as f:
    json.dump(idl, f, indent = 2)