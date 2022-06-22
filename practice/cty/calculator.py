#!/usr/bin/python2

#Carson's Awesome Calculator!

print "\n"
N1 = raw_input("Please enter the first number.")
N2 = raw_input("Please enter the second number.")
operator = raw_input("Please enter your operator (ADD, SUB, MUL, DIV)")

N1 = float(N1)
N2 = float(N2)

if(operator == "ADD"):
    sum = str(N1 + N2)
    N1 = str(N1)
    N2 = str(N2)
    print N1 + " + " + N2 + " = " + sum
elif(operator == "SUB"):
    diference = str(N1 - N2)
    N1 = str(N1)
    N2 = str(N2)
    print N1 + " - " + N2 + " = " + diference
elif(operator == "MUL"):
    product = str(N1 * N2)
    N1 = str(N1)
    N2 = str(N2)
    print N1 + " * " + N2 + " = " + product
elif(operator == "DIV"):
    if(N2 != 0):
        quotient = str(N1 / N2)
        N1 = str(N1)
        N2 = str(N2)
        print N1 + " / " + N2 + " = " + quotient
    else:
        print "No dividing by zero, silly!"
else:
    print "Invalid Input"



print "\n"
print "Thank you for using 'Carson's Awesome Calculator.'"
print "\n Come again!"
