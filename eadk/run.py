#!/usr/bin/env python3
import argparse
import os.path
import subprocess

def load_elf(elf_file):
  bin_file = os.path.splitext(elf_file)[0] + ".bin"
  subprocess.check_output(["arm-none-eabi-objcopy", "-O", "binary", elf_file, bin_file])
  subprocess.check_output(["dfu-util", "-i", "0", "-a", "0", "-s", "0x90350000:leave", "-D", bin_file])

parser = argparse.ArgumentParser(description="Load ELF file over USB")
parser.add_argument('elf', metavar='file.elf', help='input ELF file')

args = parser.parse_args()
load_elf(args.elf)
