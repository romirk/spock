CFLAGS = -std=c++23 -O3
LDFLAGS = -lglfw -lvulkan -ldl -lpthread
SRCS = main.cpp utils.cpp validation.cpp queues.cpp

VulkanTest: $(SRCS)
	g++ $(CFLAGS) -o VulkanTest $(SRCS) $(LDFLAGS)

.PHONY: test clean all

test: VulkanTest
	./VulkanTest

all: VulkanTest

clean:
	rm -f VulkanTest
