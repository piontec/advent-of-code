import re
from typing import Dict

with open("i4.txt", "r") as f:
    lines = f.readlines()

hair_ptrn = re.compile("#[0-9a-f]{6}")
height_ptrn = re.compile("(\d+)(in|cm)")


def check_pass(p: Dict[str, str]) -> bool:
    req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    opt_fields = ["cid"]
    if not len(req_fields) <= len(p.keys()) <= len(req_fields) + len(opt_fields):
        return False
    req_ok = all(key in p.keys() for key in req_fields)
    if not req_ok:
        return False
    if len(p.keys()) == len(req_fields) + len(opt_fields) and not all(key in p.keys() for key in opt_fields):
        return False
    if not (p["byr"].isdigit() and 1920 <= int(p["byr"]) <= 2002):
        return False
    if not (p["iyr"].isdigit() and 2010 <= int(p["iyr"]) <= 2020):
        return False
    if not (p["eyr"].isdigit() and 2020 <= int(p["eyr"]) <= 2030):
        return False
    hgt_m = height_ptrn.fullmatch(p["hgt"])
    if not hgt_m:
        return False
    hgt = int(hgt_m.group(1))
    hgt_ok = 150 <= hgt <= 193 if hgt_m.group(2) == "cm" else 59 <= hgt <= 76
    if not hgt_ok:
        return False
    if not hair_ptrn.fullmatch(p["hcl"]):
        return False
    if not p["ecl"] in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
        return False
    if not (len(p["pid"]) == 9 and p["pid"].isdigit()):
        return False
    return True


passport = {}
valid = 0
for line in lines:
    if line == "\n":
        if check_pass(passport):
            valid += 1
        passport = {}
    passport.update(dict([field.split(":") for field in line.split()]))

print(valid)
