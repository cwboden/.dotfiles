CC = g++
FLAGS = -g

all: main.out

run: main.out
	chmod +x $<
	./$<

%.out: %.cpp
	$(CC) $(FLAGS) $< -o $@
