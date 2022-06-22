#!/usr/bin/python2

#Carson Boden's EPIC RPS BATTLE!!!!!!!

import math
import random
import string

def play():
    timestoplay = -1
    if (timestoplay != -1):
        timestoplay = int(str.lower(raw_input("How many times would you like to play?")))

    p = str.lower(raw_input("Please enter either rock, paper, lizard, Spock, or scissors."))
    c = str(random.randint(0,4))

    winner = 4

    if (c == "0"):
        c = "rock"
    elif (c == "2"):
        c = "scissors"
    elif (c == "1"):
        c = "paper"
    elif (c == "3"):
        c = "spock"
    elif (c == "4"):
        c = "lizard"

    print "\n"
    print "Player chose: " + p
    print "Computer chose: " + c

    if (c == "rock"):
        c = "0"
    elif (c == "scissors"):
        c = "2"
    elif (c == "paper"):
        c = "1"
    elif (c == "spock"):
        c = "3"
    elif (c == "lizard"):
        c = "4"

    if (p == "rock"):
        p = "0"
    elif (p == "scissors"):
        p = "2"
    elif (p == "paper"):
        p = "1"
    elif (p == "spock"):
        p = "3"
    elif (p == "lizard"):
        p = "4"
    elif (p == "cheat"):
        winner = 3
    elif (p == "carson"):
        winner = 5

    if ((c == "0" and p == "1") or (c == "1" and p == "2") or (c == "2" and p == "0") or (c == "2" and p == "3") or (c == "0" and p == "3") or (c == "3" and p == "4") or (c == "1" and p == "4") or (c == "4" and p == "2") or (c == "3" and p == "1") or (c == "4" and p == "0")):
        winner = 1
    elif ((c == "1" and p == "0") or (c == "2" and p == "1") or (c == "0" and p == "2") or (c == "3" and p == "2") or (c == "3" and p == "0") or (c == "4" and p == "3") or (c == "4" and p == "1") or (c == "2" and p == "4") or (c == "1" and p == "3") or (c == "0" and p == "4")):
        winner = 0
    elif ((c == "1" and p == "1") or (c == "2" and p == "2") or (c == "2" and p == "2") or (c == "3" and p == "3") or (c == "4" and p == "4")):
        winner = 2

    if (winner == 1):
        if (p == "0"):
            if (c == "4"):
               print "Rock splatters lizard"
               print "**SPLUT!!!**"
            elif (c == "2"):
                print "Rock crushes scissors"
                print "**CRUNCH!**"
        if (p == "1"):
            if (c == "3"):
                print "Paper disproves Spock"
                print "LOL! SPOCK DOESN'T EXIST!!!"
            if (c == "0"):
                print "Paper covers rock"
                print "How does THAT work??"
        if (p == "2"):
            if (c == "1"):
                print "Scissors cuts paper"
                print "**SLICE!**"
            if (c == "4"):
                print "Scissors decapitates lizard"
                print "SNIPPETY SNIPETY SNIP!"
        if (p == "3"):
            if (c == "2"):
                print "Spock crushes scissors"
                print "CRUNCH!!!"
            if (c == "0"):
                print "Spock vaporizes rock"
                print "BLLLZRRRRT!!"
        if (p == "4"):
            if (c == "3"):
                print "Lizard poisons Spock"
                print "SPOCK DIED!"
            if (c == "1"):
                print "Lizard eats paper"
                print "NOM NOM NOM!!!"
        print "Player wins...fatality"

    elif (winner == 0):
        if (c == "0"):
            if (p == "4"):
               print "Rock splatters lizard"
               print "**SPLUT**"
            elif (p == "2"):
                print "Rock crushes scissors"
        if (c == "1"):
            if (p == "3"):
                print "Paper disproves spock"
                print "LOL! You don't exist!"
            if (p == "0"):
                print "Paper covers rock"
                print "How does THAT work?"
        if (c == "2"):
            if (p == "1"):
                print "Scissors cuts paper"
                print "SNIPPETY SNIPETY SNIP!"
            if (p == "4"):
                print "Scissors decapitates lizard"
                print "**SLICE**"
        if (c == "3"):
            if (p == "2"):
                print "Spock crushes scissors"
                print "CRUNCH!"
            if (p == "0"):
                print "Spock vaporizes rock"
                print "BLLLLZZZRT!!!"
        if (c == "4"):
            if (p == "3"):
                print "Lizard poisons Spock"
                print "SPOCK DIED!"
            if (p == "1"):
                print "Lizard eats paper"
                print "NOM NOM NOM NOM!!!!"
        print "Computer wins! YOU FAAAAIIIIIL!!!!!!"

    elif (winner == 2):
        print "OMG! IT'S A TIE!"

    elif (winner == 3):
        n = random.randint(0,1)
        if (n == 0):
            print "Computer noticed player cheating!"
            print "Computer wins! You FAAAAIIIIL!!!!!"
        if (n == 1):
            print "Computer was looking the other direction..."
            print "Player cheats...player wins!!!!!!"
    elif (winner == 4):
        print "Invalid input...please try again!"
        print "Player screwed up!"
        print "Computer wins! You FAAAAAAAIIIIIL!!!!!"
    elif (winner == 5):
        print "CARSON ALWAYS WINS!!!1!!ONE!!!!"

    while (timestoplay != 0):
        print "\n"
        print "AGAIN!"
        print "\n"
        timestoplay == timestoplay - 1
        play()
