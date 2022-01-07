#!/usr/bin/env python3
import argparse
import os.path
import re
import subprocess
import sys

def parse_slot_info(slot_info_file):
  data = open(slot_info_file, "rb").read()
  slot_info_pattern = re.compile(b'\xba\xdb\xee\xef(.{4})(.{4})\xba\xdb\xee\xef', flags=re.MULTILINE)
  result = slot_info_pattern.search(data)
  kernel_header_address = int.from_bytes(result.group(1), "little")
  userland_header_address = int.from_bytes(result.group(2), "little")
  return kernel_header_address, userland_header_address

def parse_userland_header(userland_header_file):
  data = open(userland_header_file, "rb").read()
  header_pattern = re.compile(b'\xfe\xed\xc0\xde.{8}.{4}.{4}(.{4})(.{4})(.{4})(.{4})\xfe\xed\xc0\xde', flags=re.MULTILINE)
  result = header_pattern.search(data)
  external_apps_start_address = int.from_bytes(result.group(1), "little")
  external_apps_end_address = int.from_bytes(result.group(2), "little")
  return external_apps_start_address, external_apps_end_address

def load_elf(elf_file, app_index = 0):
  bin_file = os.path.splitext(elf_file)[0] + ".bin"
  subprocess.check_output(["arm-none-eabi-objcopy", "-O", "binary", elf_file, bin_file])
  ram_address = 0x20000000
  subprocess.check_output(["dfu-util", "-a", "0", "-s", str(hex(ram_address)) + ":16:force", "-U", "slot_info.bin"])
  _,userland_header_address = parse_slot_info("slot_info.bin")
  subprocess.check_output(["rm", "slot_info.bin"])
  subprocess.check_output(["dfu-util", "-a", "0", "-s", str(hex(userland_header_address)) + ":40:force", "-U", "userland_header.bin"])
  external_apps_start_address,external_apps_end_address = parse_userland_header("userland_header.bin")
  subprocess.check_output(["rm", "userland_header.bin"])
  external_apps_sector_size = 64 * 1024
  # Flash right slot with external apps
  download_address = external_apps_start_address + int(app_index) * external_apps_sector_size
  if download_address >= external_apps_end_address:
    sys.stderr.write("No more space in external apps range")
    sys.exit(-1)
  print("Download external app at address: " + str(hex(download_address)) + "\n")
  subprocess.check_output(["dfu-util", "-i", "0", "-a", "0", "-s", str(hex(download_address)) + ":leave", "-D", bin_file])

parser = argparse.ArgumentParser(description="Load ELF file over USB")
parser.add_argument('elf', metavar='file.elf', help='input ELF file')

args = parser.parse_args()
load_elf(args.elf)
