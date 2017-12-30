class VectorInt {
        int _size;
        int * _values;
    public:
        VectorInt(int size);
        VectorInt(int size, int* values);
        ~VectorInt();
        int getSize();
        int getValue(int index);
        void setValue(int index, int value);
};