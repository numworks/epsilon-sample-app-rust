#!/usr/bin/env python3
import argparse
import os.path
import subprocess

def load_elf(elf_file, app_index = 0):
  bin_file = os.path.splitext(elf_file)[0] + ".bin"
  subprocess.check_output(["arm-none-eabi-objcopy", "-O", "binary", elf_file, bin_file])
  external_apps_addresses = [0x90350000,0x90750000]
  external_apps_sector_size = 64 * 1024
  # Flash both slots with external apps
  for address in external_apps_addresses:
    download_address = address + int(app_index) * external_apps_sector_size
    subprocess.check_output(["dfu-util", "-i", "0", "-a", "0", "-s", str(hex(download_address)), "-D", bin_file])
  # Leave
  subprocess.check_output(["dfu-util", "-i", "0", "-a", "0", "-s", str(hex(external_apps_addresses[0])) + ":leave", "-D", bin_file])

parser = argparse.ArgumentParser(description="Load ELF file over USB")
parser.add_argument('elf', metavar='file.elf', help='input ELF file')

args = parser.parse_args()
load_elf(args.elf)
