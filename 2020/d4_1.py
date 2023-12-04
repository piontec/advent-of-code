from typing import Dict

with open("i4.txt", "r") as f:
    lines = f.readlines()


def check_pass(p: Dict[str, str]) -> bool:
    req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    opt_fields = ["cid"]
    req_ok = all(key in p.keys() for key in req_fields)
    if req_ok and len(p.keys()) == len(req_fields):
        return True
    if req_ok and len(p.keys()) == len(req_fields) + len(opt_fields):
        return all(key in p.keys() for key in opt_fields)
    return False


passport = {}
valid = 0
for line in lines:
    if line == "\n":
        if check_pass(passport):
            valid += 1
        passport = {}
    passport.update(dict([field.split(":") for field in line.split()]))

print(valid)
