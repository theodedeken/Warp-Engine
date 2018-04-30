#include <assert.h>

#include "VectorDouble.hpp"

VectorDouble::VectorDouble(int size){
    assert(size >= 0);
    _size = size;
    _values = new double [size];
}

VectorDouble::~VectorDouble(){
    delete[] _values;
}

int VectorDouble::getSize() const {
    return _size;
}

double VectorDouble::getValue(int index){
    assert(index >= 0 && index < _size);
    return _values[index];
}

void VectorDouble::setValue(int index, double value){
    assert(index >= 0 && index < _size);
    _values[index] = value;
}

VectorDouble* VectorDouble::plus(VectorDouble* b){
    assert(_size == b->getSize());
    VectorDouble* res = new VectorDouble(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i] + b->getValue(i));
    }
    return res;
}

VectorDouble* VectorDouble::minus(VectorDouble* b){
    assert(_size == b->getSize());
    VectorDouble* res = new VectorDouble(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i] - b->getValue(i));
    }
    return res;
}

VectorDouble* VectorDouble::mult(double a){
    VectorDouble* res = new VectorDouble(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i]*a);
    }
    return res;
}

VectorDouble* VectorDouble::div(double a){
    VectorDouble* res = new VectorDouble(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i]/a);
    }
    return res;
}

double VectorDouble::dot(VectorDouble* b){
    assert(_size == b->getSize());
    double res = 0;
    for (int i = 0; i < _size; i++){
        res += _values[i] * b->getValue(i);
    }
    return res;
    
}