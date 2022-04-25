numcount = input("How many numbers to dispaly: ")
# try converting numcount to int
try:
    numcount = int(numcount)
except:
    print("Invalid input")
    quit()

for i in range(numcount):
    i = i + 1
    if i % 3 == 0 and i % 5 == 0:
        print("FizzBuzz")
    elif i % 3 == 0:
        print("Fizz")
    elif i % 5 == 0:
        print("Buzz")
    else:
        print(i)