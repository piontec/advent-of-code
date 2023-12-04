import re
from dataclasses import dataclass
from typing import Optional, List, Dict, Set


@dataclass
class Ingredient:
    name: str
    allergen: Optional[str]

    def __hash__(self) -> int:
        return hash(self.name)

    def __str__(self) -> str:
        return self.name

    def __eq__(self, other: "Ingredient") -> bool:
        return self.name == other.name


Rules = Dict[str, List[Set[Ingredient]]]


def parse_rules(lines: List[str]) -> (Rules, Set[Ingredient]):
    all_ingredients: Set[Ingredient] = set()
    rules: Rules = {}
    rule_re = re.compile(r"(?P<ingredients>(\w+ )+)\(contains (?P<allergens>\w+(, \w+)*)\)")
    for line in lines:
        match = rule_re.match(line)
        if not match:
            raise Exception("this should have matched")
        ings = {Ingredient(name=ing_name.strip(), allergen="") for ing_name in
                match.group("ingredients").strip().split(" ")}
        all_ingredients = all_ingredients.union(set(ings))
        for allergen in match.group("allergens").split(","):
            a = allergen.strip()
            if a not in rules.keys():
                rules[a] = []
            rules[a].append(ings)
    return rules, all_ingredients


def solve_matches(allergen_to_ingredients: Rules) -> List[Ingredient]:
    solved_ingredients: List[Ingredient] = []
    while True:
        for alrg in allergen_to_ingredients.keys():
            possible_ingrs = allergen_to_ingredients[alrg][0]
            possible_ingrs = possible_ingrs.intersection(*allergen_to_ingredients[alrg][1:])
            if len(possible_ingrs) == 1:
                ing = possible_ingrs.pop()
                ing.allergen = alrg
                solved_ingredients.append(ing)
                del(allergen_to_ingredients[alrg])
                for sets in allergen_to_ingredients.values():
                    for s in sets:
                        for e in s:
                            if e.name == ing.name:
                                s.remove(e)
                                break
                break
        else:
            break
    return solved_ingredients


def find_single_without_allergens(lines: List[str]) -> (int, str):
    allergen_rules, all_ings = parse_rules(lines)
    solved_ings = solve_matches(allergen_rules.copy())
    diff = all_ings.difference(set(solved_ings))
    count = 0
    for ing in diff:
        find_re = re.compile(r"(^| )" + ing.name + " ")
        for line in lines:
            if find_re.search(line):
                count += 1
    sorted_list = list(solved_ings)
    sorted_list.sort(key=lambda x: x.allergen)
    the_list = ",".join([x.name for x in sorted_list])
    return count, the_list


test1 = """mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"""
res1, list1 = find_single_without_allergens(test1.splitlines())
assert res1 == 5
assert list1 == "mxmxvkd,sqjhc,fvjkl"

with open("i21.txt", "r") as f:
    lines = f.readlines()
res, lst = find_single_without_allergens(lines)
print(res)
print(lst)
