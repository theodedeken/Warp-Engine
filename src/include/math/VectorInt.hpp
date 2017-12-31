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
};

EMSCRIPTEN_BINDINGS(vector_int) {
  class_<VectorInt>("VectorInt")
    .constructor<int>()
    .property("size", &VectorInt::getSize)
    .function("getValue", &VectorInt::getValue)
    .function("setValue", &VectorInt::setValue)
    ;
}