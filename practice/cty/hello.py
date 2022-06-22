#!/usr/bin/python2

#Carson Boden's Epic Python Script!
print "\n"

name = raw_input("What is your name?")
age = raw_input("How old are you?")
htown = raw_input("What town do you live in?")
color = raw_input("What is your favorite color?")
mom = raw_input("What is your mother's name?")
dad = raw_input("What is your father's name?")
fname = raw_input("What is your family name?")
mname = raw_input("What is your middle name?")

print "Hello " + name + " " + mname + " " + fname + ", the son of " + dad + " and " + mom + ", who lives in the town of " + htown + " and is " + age + " years old and favorite color is " + color + "."
years = int(age)
now = years + 10
years = str(now)
print "In ten years, you will be " + years
print "Carson is awesome. "*25
