#ifndef WARP_MATH_VECTORINT_H_
#define WARP_MATH_VECTORINT_H_

#include <emscripten/bind.h>
using namespace emscripten;

class VectorInt {
    private:
        int _size;
        int * _values;
    public:
        /**
         * Constructs a vector with elements of type `int` with the specified size.
         * 
         * @param size The size of the vector
         */
        VectorInt(int size);

        /**
         * Destructs the vector
         */
        ~VectorInt();

        /**
         * Return the size of the vector
         * 
         * @return integer containing size
         */
        int getSize() const;

        /**
         * Return the value at the specified index in the vector
         * 
         * @param index the index to return
         * @return the value at the specified index
         */
        int getValue(int index);

        /**
         * Set the value at the specified index
         * 
         * @param index the index of the value to be set
         * @param value the value to be set
         */
        void setValue(int index, int value);
        
        /**
         * Performs an addition of two vectors and returns a new `VectorDouble` containing the result.
         * 
         * @param b the vector to be added to this vector
         * @return new `VectorDouble` containing the result
         */
        VectorInt* plus(VectorInt* b);

        /**
         * Performs a subtraction of two vectors and returns a new `VectorDouble` containing the result.
         * 
         * @param b the vector to be subtracted to this vector
         * @return new `VectorDouble` containing the result
         */
        VectorInt* minus(VectorInt* b);

        /**
         * Performs a multiplication of a vector and a value and return a new `VectorDouble` containing the result.
         * 
         * @param a the value to multiplicate to the elements of this vector.
         * @return new `VectorDouble` containing the result
         */
        VectorInt* mult(int a);

        /**
         * Performs a division of a vector and a value and returns a new `VectorDouble` containing the result.
         * 
         * @param a the value to divide the elements of this vector
         * @return new `VectorDouble` containing the resul
         */
        VectorInt* div(int a);

        /**
         * Calculates the dot product of two vectors.
         * 
         * @param b the other vector
         * @return the result of the dot product
         */
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