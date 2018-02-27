#ifndef WARP_MATH_VECTORINT_H_
#define WARP_MATH_VECTORINT_H_

#include <emscripten/bind.h>
using namespace emscripten;

class VectorInt {
    private:
        int _size;
        int * _values;
    public:
        VectorInt(int size);
        ~VectorInt();
        int getSize() const;
        int getValue(int index);
        void setValue(int index, int value);
        
        VectorInt* plus(VectorInt* b);
        VectorInt* minus(VectorInt* b);
        VectorInt* mult(int a);
        VectorInt* div(int a);
        int dot(VectorInt* b);
};

EMSCRIPTEN_BINDINGS(vector_int) {
  class_<VectorInt>("VectorInt")
    .constructor<int>()
    .property("size", &VectorInt::getSize)
    .function("getValue", &VectorInt::getValue)
    .function("setValue", &VectorInt::setValue)
    .function("plus", &VectorInt::plus, allow_raw_pointers())
    .function("minus", &VectorInt::minus, allow_raw_pointers())
    .function("mult", &VectorInt::mult, allow_raw_pointers())
    .function("div", &VectorInt::div, allow_raw_pointers())
    .function("dot", & VectorInt::dot, allow_raw_pointers())
    ;
}

#endif