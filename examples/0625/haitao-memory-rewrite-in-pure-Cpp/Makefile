CXX = g++
CXXFLAGS = -Wall -Wextra -std=c++14
LDFLAGS =

# Define the include directory
INCLUDES = -I include

# Define source directory and executable output
SRCDIR = src
TARGET = main

# Collect all sources
SOURCES = $(wildcard $(SRCDIR)/*.cc)
OBJECTS = $(SOURCES:.cc=.o)

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CXX) $(CXXFLAGS) $(INCLUDES) $(LDFLAGS) -o $@ $^

%.o: %.cc
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

clean:
	rm -f $(SRCDIR)/*.o $(TARGET)

.PHONY: all clean
