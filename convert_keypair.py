import os, json, base58
import subprocess
from argparse import ArgumentParser


parser = ArgumentParser()
parser.add_argument("--fp")
args = parser.parse_args()

with open(args.fp, "r+") as f:
    arr = json.load(f)
    byte_data = bytes(arr)
    print(base58.b58encode(byte_data).decode('utf-8'))
