#ifndef WARP_MATH_VECTORDOUBLE_H_
#define WARP_MATH_VECTORDOUBLE_H_

#include <emscripten/bind.h>
using namespace emscripten;

class VectorDouble {
    private:
        int _size;
        double * _values;
    public:
        VectorDouble(int size);
        ~VectorDouble();
        int getSize() const;
        double getValue(int index);
        void setValue(int index, double value);
        
        VectorDouble* plus(VectorDouble* b);
        VectorDouble* minus(VectorDouble* b);
        VectorDouble* mult(double a);
        VectorDouble* div(double a);
        double dot(VectorDouble* b);
};

EMSCRIPTEN_BINDINGS(vector_double) {
  class_<VectorDouble>("VectorDouble")
    .constructor<int>()
    .property("size", &VectorDouble::getSize)
    .function("getValue", &VectorDouble::getValue)
    .function("setValue", &VectorDouble::setValue)
    .function("plus", &VectorDouble::plus, allow_raw_pointers())
    .function("minus", &VectorDouble::minus, allow_raw_pointers())
    .function("mult", &VectorDouble::mult, allow_raw_pointers())
    .function("div", &VectorDouble::div, allow_raw_pointers())
    .function("dot", &VectorDouble::dot, allow_raw_pointers())
    ;
}

#endif