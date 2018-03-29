#include <assert.h>

#include "VectorInt.hpp"

VectorInt::VectorInt(int size){
    assert(size >= 0);
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
    assert(index >= 0 && index < _size);
    return _values[index];
}

void VectorInt::setValue(int index, int value){
    assert(index >= 0 && index < _size);
    _values[index] = value;
}

VectorInt* VectorInt::plus(VectorInt* b){
    assert(_size == b->getSize());
    VectorInt* res = new VectorInt(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i] + b->getValue(i));
    }
    return res;
}

VectorInt* VectorInt::minus(VectorInt* b){
    assert(_size == b->getSize());
    VectorInt* res = new VectorInt(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i] - b->getValue(i));
    }
    return res;
}

VectorInt* VectorInt::mult(int a){
    VectorInt* res = new VectorInt(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i]*a);
    }
    return res;
}

VectorInt* VectorInt::div(int a){
    VectorInt* res = new VectorInt(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i]/a);
    }
    return res;
}

int VectorInt::dot(VectorInt* b){
    assert(_size == b->getSize());
    int res = 0;
    for (int i = 0; i < _size; i++){
        res += _values[i] * b->getValue(i);
    }
    return res;    
}

