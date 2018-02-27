#include "VectorInt.hpp"

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

VectorInt* VectorInt::plus(VectorInt* b){
    if (_size == b->getSize()){
        VectorInt* res = new VectorInt::VectorInt(_size);
        for (int i = 0; i < _size; i++){
            res->setValue(i, _values[i] + b->getValue(i));
        }
        return res;
    }
    else {
        //TODO error
        return nullptr;
    }
}

VectorInt* VectorInt::minus(VectorInt* b){
    if (_size == b->getSize()){
        VectorInt* res = new VectorInt::VectorInt(_size);
        for (int i = 0; i < _size; i++){
            res->setValue(i, _values[i] - b->getValue(i));
        }
        return res;
    }
    else {
        //TODO error
        return nullptr;
    }
}

VectorInt* VectorInt::mult(int a){
    VectorInt* res = new VectorInt::VectorInt(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i]*a);
    }
    return res;
}

VectorInt* VectorInt::div(int a){
    VectorInt* res = new VectorInt::VectorInt(_size);
    for (int i = 0; i < _size; i++){
        res->setValue(i, _values[i]/a);
    }
    return res;
}

int VectorInt::dot(VectorInt* b){
    if (_size == b->getSize()){
        int res = 0;
        for (int i = 0; i < _size; i++){
            res += _values[i] * b->getValue(i);
        }
        return res;
    }
    else {
        //TODO error
        return -1;
    }
}

