#!/usr/bin/python2

#Carson Boden's Cool Name Program

import string

print "\n"
name = string.lower(raw_input("Please enter your name"))

if ((name == "dan") or (name == "anurag") or (name == "carson")):
    print "You're name is awesome!!!!!"
elif ((name[:3] == "dan") or (name[:7] == "anurag") or (name[:6] == "carson")):
    print "Your name is Amazingasaurus Rex!!!!"
elif ((name[0] == "d" and name[-1] == "n") or (name[0] == "a" and name[-1] == "g") or (name[0] == "c" and name[-1] == "n")):
    print "Your name is epic!!!"
elif (name[0] == "a" or name[0] == "d" or name[0] == "c"):
    print "Your name is amazing!!"
else:
    print "Your name is pretty cool, but Dan or Anurag or Carson would be a cooler name."

print "EXPLOSION!"
