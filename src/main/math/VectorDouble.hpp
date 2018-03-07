#ifndef WARP_MATH_VECTORDOUBLE_H_
#define WARP_MATH_VECTORDOUBLE_H_

#include <emscripten/bind.h>
using namespace emscripten;

/**
 * Class representing vectors with elements of type `double`.
 * 
 * @author Theo Dedeken
 */
class VectorDouble {
    private:
        int _size;
        double * _values;
    public:
        /**
         * Constructs a vector with elements of type `double` with the specified size.
         * 
         * @param size The size of the vector
         */
        VectorDouble(int size);
        
        /**
         * Destructs the vector
         */
        ~VectorDouble();
        
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
        double getValue(int index);
        
        /**
         * Set the value at the specified index
         * 
         * @param index the index of the value to be set
         * @param value the value to be set
         */
        void setValue(int index, double value);
        
        /**
         * Performs an addition of two vectors and returns a new `VectorDouble` containing the result.
         * 
         * @param b the vector to be added to this vector
         * @return new `VectorDouble` containing the result
         */
        VectorDouble* plus(VectorDouble* b);
        
        /**
         * Performs a subtraction of two vectors and returns a new `VectorDouble` containing the result.
         * 
         * @param b the vector to be subtracted to this vector
         * @return new `VectorDouble` containing the result
         */
        VectorDouble* minus(VectorDouble* b);
        
        /**
         * Performs a multiplication of a vector and a value and return a new `VectorDouble` containing the result.
         * 
         * @param a the value to multiplicate to the elements of this vector.
         * @return new `VectorDouble` containing the result
         */
        VectorDouble* mult(double a);
        
        /**
         * Performs a division of a vector and a value and returns a new `VectorDouble` containing the result.
         * 
         * @param a the value to divide the elements of this vector
         * @return new `VectorDouble` containing the resul
         */
        VectorDouble* div(double a);
        
        /**
         * Calculates the dot product of two vectors.
         * 
         * @param b the other vector
         * @return the result of the dot product
         */
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

#endif