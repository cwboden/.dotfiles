#!/usr/bin/python2

coeff1 = int(raw_input("Enter coefficient of square"))
coeff2 = int(raw_input("Enter coefficient for singular term"))
constant = int(raw_input("Enter constant"))
again = "1"

print "=*=*=*=*=*=NEW EQUATION=*=*=*=*=*="
print str(coeff1) + "x^2" + " + " + str(coeff2) + "x" + " + " + str(constant)

while again == "1":
    x = int(raw_input("Enter value for 'x'"))
    print str(coeff1) + "(" + str(x) + ")^2" + " + " + str(coeff2) + "(" + str(x) + ")" + " + " + str(constant) + "="
    print (coeff1*x*x)+(coeff2*x)+constant
    again = raw_input("Same equation but a different value for x? (1 = yes, 0 = no)")
