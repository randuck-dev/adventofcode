import os

dir_path = os.path.dirname(os.path.realpath(__file__));
f_path = dir_path + "/inputs/10.txt"
print(f_path)
f = open(f_path, "r")

register_x = 1

value_x_at_strength = []

total_cycles_ran = 0

res = 0

result = ""
sprite_position = 1

for line in f:
  split = line.split()
  
  operation = split[0]

  cycles_to_run = 1

  if operation == "addx":
    cycles_to_run = 2

  
  elif operation == "noop":
    cycles_to_run = 1
  
  for i in range(0, cycles_to_run):
    total_cycles_ran += 1
    if total_cycles_ran == 20 or (total_cycles_ran > 40 and (total_cycles_ran -20) % 40 == 0):
      w = (total_cycles_ran * register_x) 
      print(total_cycles_ran, w)
      res += w

    left_over = total_cycles_ran % 40
    if left_over == 0:
      result += "\n"
    else:
      if sprite_position == left_over or sprite_position - 1 == left_over or sprite_position +1 == left_over:
        result +="#"
      else:
        result += "."
    
  
  if operation == "addx": 
    register_x += int(split[1])
    sprite_position = register_x + 1
    # print("new value of x", register_x)
      
    

print("Part 1: {}".format(res))
  
print(result)

  




# Cycles concept. 

# if noop, 1 cycle

# if addx 2 cycles