from typing import List


Layer = List[List[int]]


class Image():
    def __init__(self, width: int, height: int, data: str):
        self.layers: List[Layer] = []
        self.width = width
        self.height = height
        self.parse(data)

    def parse(self, data: str):
        digits = [int(c) for c in data.strip()]
        start = 0
        pixel_count = self.height * self.width
        layers_count = len(digits) // pixel_count
        for _ in range(layers_count):
            new_layer: Layer = []
            for _ in range(self.height):
                new_layer.append(digits[start:start + self.width])
                start += self.width
            self.layers.append(new_layer)

    def digits_per_layer(self, layer: Layer, digit: int) -> int:
        count = 0
        for row in layer:
            found_row = [d == digit for d in row]
            count += sum(found_row)
        return count

    def find_layer(self) -> Layer:
        best_layer = None
        best_zeros = 0
        for layer in self.layers:
            zeros = self.digits_per_layer(layer, 0)
            if best_layer == None or zeros < best_zeros:
                best_layer = layer
                best_zeros = zeros
        return best_layer

    def checksum(self) -> int:
        layer = self.find_layer()
        ones = self.digits_per_layer(layer, 1)
        twos = self.digits_per_layer(layer, 2)
        return ones * twos

    def print(self, layer: Layer):
        for y in range(self.height):
            for x in range(self.width):
                p = layer[y][x]
                if p == 0:
                    print(" ", end='')
                else:
                    print("O", end='')
            print()

    def render(self) -> Layer:
        result: Layer = []
        for y in range(self.height):
            result.append(self.width * [-1])
        for y in range(self.height):
            for x in range(self.width):
                for l in self.layers:
                    if l[y][x] == 2:
                        continue
                    result[y][x] = l[y][x]
                    break
        return result


image = Image(3, 2, "123456789012")
assert image.checksum() == 1
image.print(image.render())
with open("i8.txt") as f:
    lines = f.readlines()
image = Image(25, 6, lines[0])
print(str.format("Part 1: {}", image.checksum()))
print("part2:")
image.print(image.render())
