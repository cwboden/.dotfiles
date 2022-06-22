#!/usr/bin/python2

#Carson Boden's Palindrome Program

pal = raw_input("Please enter a palindrome to test.")
print "\n"
print "You entered: " + pal
if (pal[-1::-1] == pal):
    print "Palindrome accepted."
else:
    print "YOU FAAAAAAAIIIIIIL!!!!!"
