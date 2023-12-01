import os

dir_path = os.path.dirname(os.path.realpath(__file__));
f_path = dir_path + "/inputs/test_12.txt"
print(f_path)
f = open(f_path, "r")