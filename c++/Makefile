CFLAGS = -std=c++23 -O0 -g
LDFLAGS = -lglfw -lvulkan -ldl -lpthread
SRCS = main.cpp utils.cpp validation.cpp queues.cpp
SHADERS = $(wildcard ./shaders/*.vert) $(wildcard ./shaders/*.frag)

.PHONY: test clean all VulkanTest shaders

all: VulkanTest shaders

shaders: $(SHADERS)
	@for shader in $(SHADERS); do \
		glslc $$shader -o $$shader.spv; \
		echo "Compiled $$shader"; \
	done

VulkanTest: $(SRCS) $(SHADERS)
	g++ $(CFLAGS) -o VulkanTest $(SRCS) $(LDFLAGS)


test: all
	./VulkanTest

clean:
	rm -f VulkanTest
	rm -f ./shaders/*.spv
