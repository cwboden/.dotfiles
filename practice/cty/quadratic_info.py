#!/usr/bin/python2

coeff1 = float(raw_input("Enter coefficient of square"))
coeff2 = float(raw_input("Enter coefficient for singular term"))
constant = float(raw_input("Enter constant"))
x = 0
equ = 0
sort = 11

print "\n"
print "Carson's Handy Parabola Program"
print "\n"
print "=*=*=*=*=*=NEW EQUATION=*=*=*=*=*="
print "y = " + str(coeff1) + "x^2" + " + " + str(coeff2) + "x" + " + " + str(constant)
sym = (-coeff2)/(2*coeff1)
x = sym
print "Line of symmetry: x= " + str(sym)
print "Vertex: (" + str(sym) + ", " + str((coeff1*x*x)+(coeff2*x)+constant) + ")"
print "\n"
print "4 other points defined one away from vertex"
x = x-6

while sort != 0:
    x = x+1
    equ = (str((coeff1*x*x)+(coeff2*x)+constant))
    if sort == 6:
        print "(" + str(x) + ", " + str(equ) + ")" + " <-- VERTEX"
    else:
        print "(" + str(x) + ", " + str(equ) + ")"
    sort = sort-1
