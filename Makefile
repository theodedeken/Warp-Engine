EMSDK =  C:/emsdk-portable-64bit/emscripten/1.37.26
PROJECT =  Warp

BUILDDIR = build
INCDIR = src/include
MAINDIR = src/main
DIRS = math
TESTDIR = src/test
FILES = $(foreach dir, $(DIRS), $(addprefix $(BUILDDIR)/, $(patsubst %.cpp, %.bc, $(wildcard $(MAINDIR)/$(dir)/*.cpp))))

CXX = $(EMSDK)/em++
#TODO temp
#SRCS = $(wildcard $(MAINDIR)/math/*.cpp)

#OBJS = $(addprefix $(BUILDDIR)/, $(patsubst %.cpp, %.o, $(notdir $(wildcard $(MAINDIR)/*.cpp))))
#TESTOBJS = $(addprefix $(BUILDDIR)/, $(patsubst %.cpp, %.o, $(notdir $(wildcard $(TESTDIR)/*.cpp))))

CXXFLAGS += $(DIRS:%=-I/$(INCDIR)/%) -std=c++1y
CXXFLAGSO = $(CXXFLAGS) -s WASM=1
.PHONY:
	clean
debug: 
	echo $(FILES)

wasm: wasmbc wasmjs

wasmbc: $(FILES)
$(FILES): $(BUILDDIR)/%.bc: %.cpp
	$(CXX) $(CXXFLAGS) $< -o $@

wasmjs: $(FILES)
	$(CXX) $(CXXFLAGSO) $^ -o $(BUILDDIR)/$(PROJECT).html

clean:
	$(RM) -f $(BUILDDIR)/*.d $(BUILDDIR)/*.bc