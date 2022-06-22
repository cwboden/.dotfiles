#!/usr/bin/python2

#Carson Boden's EPIC RPS BATTLE!!!!!!!

import math
import random
import string

p = str.lower(raw_input("Please enter either rock, paper, or scissors."))
c = str(random.randint(0,2))

print "\n"
print "Player chose: " + p
print "Computer chose: " + c

if (p == "rock"):
    p == "0"
elif (p == "scissors"):
    p == "2"
elif (p == "paper"):
    p == "1"

if (p == "rock"):
    p == "0"
elif (p == "scissors"):
    p == "2"
elif (p == "paper"):
    p == "1"
elif (p == "carson"):
    winner = 3
else:
    p == 10

if ((c == "0" and p == "1") or (c == "1" and p == "2") or (c == "2" and p == "0")):
    winner = 1
elif ((c == "1" and p == "0") or (c == "2" and p == "1") or (c == "0" and p == "2")):
    winner = 0
elif ((c == "1" and p == "1") or (c == "2" and p == "2") or (c == "0" and p == "2")):
    winner = 2
else:
    if (winner != 3):
        print "Invalid input...please try again!"

if (winner == 1):
        print "Player wins...fatality"
elif (winner == 0):
    print "Computer wins! YOU FAAAAIIIIIL!!!!!!"
elif (winner == 2):
    print "OMG! IT'S A TIE!"
elif (winner == 3):
    print "CARSON ALWAYS WINS!!!!!"
