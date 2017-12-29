EMSDK =  C:/emsdk-portable-64bit/emscripten/1.37.26
PROJECT =  Warp

BUILDDIR = build
INCDIR = src/include
MAINDIR = src/main
TESTDIR = src/test

CXX = $(EMSDK)/em++

SRCS = $(wildcard $(MAINDIR)/*.cpp)

OBJS = $(addprefix $(BUILDDIR)/, $(patsubst %.cpp, %.o, $(notdir $(wildcard $(MAINDIR)/*.cpp))))
TESTOBJS = $(addprefix $(BUILDDIR)/, $(patsubst %.cpp, %.o, $(notdir $(wildcard $(TESTDIR)/*.cpp))))

CXXFLAGS += $(INCDIR:%=-I %) -std=c++1y -s WASM=1
.PHONY:
	clean

wasm: $(SRCS)
	$(CXX) $(CXXFLAGS) $^ -o $(BUILDDIR)/$(PROJECT).html

clean:
	$(RM) -f $(BUILDDIR)/*.d $(BUILDDIR)/*.o