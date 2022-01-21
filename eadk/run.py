#!/usr/bin/env python3
import argparse
import os.path
import re
import subprocess
import sys

def parse_device_information(device_information_file):
  data = open(device_information_file, "r").read()
  pattern = re.compile(
          "EXTERNAL_APPS_FLASH_START = (0[xX][0-9a-fA-F]{8});\n"
          "EXTERNAL_APPS_FLASH_END = (0[xX][0-9a-fA-F]{8});\n"
          "EXTERNAL_APPS_RAM_START = (0[xX][0-9a-fA-F]{8});\n"
          "EXTERNAL_APPS_RAM_END = (0[xX][0-9a-fA-F]{8});\n",
    flags=re.MULTILINE)
  result = pattern.search(data)
  external_apps_flash_start =int(result.group(1), base = 16)
  external_apps_flash_end = int(result.group(2), base = 16)
  external_apps_ram_start = int(result.group(3), base = 16)
  external_apps_ram_end = int(result.group(4), base = 16)

  print(external_apps_flash_start)

  return external_apps_flash_start, external_apps_flash_end, external_apps_ram_start, external_apps_ram_end

def load_elf(elf_file, device_information, app_index):
  # Generate bin file
  bin_file = os.path.splitext(elf_file)[0] + ".bin"
  subprocess.check_output(["arm-none-eabi-objcopy", "-O", "binary", elf_file, bin_file])
  external_apps_flash_start, external_apps_flash_end, external_apps_ram_start, external_apps_ram_end = parse_device_information(device_information)

  # Compute external app address
  external_apps_sector_size = 64 * 1024
  # Flash right slot with external apps
  download_address = external_apps_flash_start + int(app_index) * external_apps_sector_size
  if download_address >= external_apps_flash_end:
    sys.stderr.write("No more space in external apps range")
    sys.exit(-1)
  print("Download external app at address: " + str(hex(download_address)) + "\n")
  subprocess.check_output(["dfu-util", "-i", "0", "-a", "0", "-s", str(hex(download_address)) + ":leave", "-D", bin_file])

parser = argparse.ArgumentParser(description="Load ELF file over USB")
parser.add_argument('elf', metavar='file.elf', help='input ELF file')
parser.add_argument('--app-index', metavar='i', default=0, help='Specify the application index')
parser.add_argument('--device-information', default="target/device_information.ld", metavar='file', help='Provide ld file with flash/RAM layout information')

args = parser.parse_args()
load_elf(args.elf, args.device_information, args.app_index)
