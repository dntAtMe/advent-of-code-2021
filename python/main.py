import pathlib
import os

def main():
    print(count_increased_depths())

def count_increased_depths() -> int:
    counter: int = 0
    with open(os.path.join(pathlib.Path().resolve(), 'input.txt'), 'r') as reader:
        values: list[int] = list(map(lambda x: int(x), reader.readlines()))
        for x in range(1, len(values)):
            # print("{} - {}".format(x, values[x]))
            current_value = values[x]
            previous_value = values[x-1]
            if previous_value < current_value:
                counter += 1
    return counter

if __name__ == "__main__":
    main()