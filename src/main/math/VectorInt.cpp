#include "../../include/math/VectorInt.hpp"

VectorInt::VectorInt(int size){
    _size = size;
    _values = new int [size];
}

VectorInt::~VectorInt(){
    delete[] _values;
}

int VectorInt::getSize() const {
    return _size;
}

int VectorInt::getValue(int index){
    return _values[index];
}

void VectorInt::setValue(int index, int value){
    _values[index] = value;
}

