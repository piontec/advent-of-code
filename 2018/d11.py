grid_size = 300

def cell_value(x, y, serial_num):
    rid = x + 10
    n = (rid * y + serial_num) * rid
    return int((n % 1000) / 100) - 5


def get_cells(serial_num):
    return {(x, y): cell_value(x, y, serial_num) for y in range(1, grid_size+1) for x in range(1, grid_size+1)}


def square_value_with_prev(x, y, size, cells, prev):
    p = prev[(x, y)]
    sy = sum([cells[(x+size-1, y)] for y in range(y, y+size)])
    sx = sum([cells[(x, y+size-1)] for x in range(x, x+size-1)])
    return p + sy + sx
    

def get_max_cell_with_prev(size, cells, prev):
    print("Running for size {}".format(size))    
    squares = {(x, y): square_value_with_prev(x, y, size, cells, prev) for y in range(1, grid_size+2-size) 
        for x in range(1, grid_size+2-size)}    
    return squares


def print_grid(cells, size):
    for y in range(1, grid_size+2-size):
        for x in range(1, grid_size+2-size):
            print("{:3d} ".format(cells[(x, y)]), end="")
        print("")

# test case - expected 29 with (33,45)
c = get_cells(18)
prev = c
for i in range(2, 4):
    prev = get_max_cell_with_prev(i, c, prev)
max_coords = max(prev, key=lambda s: prev[s])
max_val = prev[max_coords]
assert max_val == 29
assert max_coords == (33, 45)


c = get_cells(9110)
# part1
prev = c
for i in range(2, 4):
    prev = get_max_cell_with_prev(i, c, prev)
max_coords = max(prev, key=lambda s: prev[s])      
print((max_coords, prev[max_coords]))

# part2
prev = c
max_val = -300*300*10 # must be less then min possible value
max_size = 0
for i in range(2, 301):
    prev = get_max_cell_with_prev(i, c, prev)
    coords = max(prev, key=lambda s: prev[s])
    val = prev[coords]
    if val > max_val:
        max_val = val
        max_coords = coords
        max_size = i
print(max_size, max_coords, max_val)