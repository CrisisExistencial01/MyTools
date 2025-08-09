CC = gcc
CFLAGS = -Wall -g
SRC_DIR = src
TARGET_DIR = bin
SOURCE = $(wildcard $(SRC_DIR)/*.c)
OBJECTS = $(patsubst $(SRC_DIR)/%.c,$(TARGET_DIR)/%.o,$(SOURCE))
TARGET = $(TARGET_DIR)/docker-utils

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CC) $(OBJECTS) -o $(TARGET)

$(TARGET_DIR)/%.o: $(SRC_DIR)/%.c | $(TARGET_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET_DIR):
	mkdir -p $(TARGET_DIR)

clean:
	rm -rf $(TARGET_DIR)
