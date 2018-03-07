import test from 'ava';
import Module from '../_Warp'

function floatTest(t, input, expected, decimal){
    var val = Math.round(input * Math.pow(10, decimal)) / Math.pow(10, decimal)
    t.is(val, expected)
}

test('constructor', t => {
    var vector = new Module.VectorDouble(3)
    t.not(vector, null)
})

test.failing('constructor exceptions negative size', t => {
    var vector = new Module.VectorDouble(-1)
})

test('get size', t => {
    var vector = new Module.VectorDouble(1)
    t.is(vector.size, 1)
    vector = new Module.VectorDouble(0)
    t.is(vector.size, 0)
    vector = new Module.VectorDouble(123)
    t.is(vector.size, 123)
})

test('set and get value', t => {
    var vector = new Module.VectorDouble(3)
    t.is(vector.getValue(0), 0)
    vector.setValue(1, 1.5)
    t.is(vector.getValue(1), 1.5)
    vector.setValue(2, -5.635)
    t.is(vector.getValue(0), 0)
    t.is(vector.getValue(1), 1.5)
    t.is(vector.getValue(2), -5.635)
})

test.failing('exceptions when accessing elements out of bounds', t => {
    var vector = new Module.VectorDouble(3)
    vector.setValue(-1, 8)
    vector.getValue(-5)
})

test('plus', t => {
    var vector1 = new Module.VectorDouble(3)
    var vector2 = new Module.VectorDouble(3)
    vector1.setValue(0, -1.5)
    vector1.setValue(1, 3.14)
    vector2.setValue(0, 10)
    vector2.setValue(2, 7.6)
    var vectorPlus1 = vector1.plus(vector2)
    var vectorPlus2 = vector2.plus(vector1)
    t.is(vectorPlus1.getValue(0), 8.5)
    t.is(vectorPlus1.getValue(1), 3.14)
    t.is(vectorPlus1.getValue(2), 7.6)
    t.is(vectorPlus1.getValue(0), vectorPlus2.getValue(0))
    t.is(vectorPlus1.getValue(1), vectorPlus2.getValue(1))
    t.is(vectorPlus1.getValue(2), vectorPlus2.getValue(2))
})

test.failing('exceptions with plus', t => {
    var vector1 = new Module.VectorDouble(2)
    var vector2 = new Module.VectorDouble(3)
    var vectorPlus1 = vector1.plus(vector2)
    var vectorPlus2 = vector2.plus(vector1)
})

test('minus', t => {
    var vector1 = new Module.VectorDouble(3)
    var vector2 = new Module.VectorDouble(3)
    vector1.setValue(0, -1.5)
    vector1.setValue(1, 3.14)
    vector2.setValue(0, 10)
    vector2.setValue(2, 7.6)
    var vectorMinus1 = vector1.minus(vector2)
    var vectorMinus2 = vector2.minus(vector1)
    t.is(vectorMinus1.getValue(0), -11.5)
    t.is(vectorMinus1.getValue(1), 3.14)
    t.is(vectorMinus1.getValue(2), -7.6)
    t.is(vectorMinus2.getValue(0), 11.5)
    t.is(vectorMinus2.getValue(1), -3.14)
    t.is(vectorMinus2.getValue(2), 7.6)
})

test.failing('exceptions with minus', t => {
    var vector1 = new Module.VectorDouble(2)
    var vector2 = new Module.VectorDouble(3)
    var vectorPlus1 = vector1.minus(vector2)
    var vectorPlus2 = vector2.minus(vector1)
})

test('mult', t => {
    var vector = new Module.VectorDouble(3)
    vector.setValue(0, 6.9)
    vector.setValue(1, 5)
    vector.setValue(2, 3.14)
    var vectorMult1 = vector.mult(5)
    var vectorMult2 = vector.mult(-2)
    var vectorMult3 = vector.mult(1.5)
    var vectorMult4 = vector.mult(0.5)
    floatTest(t, vectorMult1.getValue(0), 34.5, 1)  
    floatTest(t, vectorMult1.getValue(1), 25, 0)
    floatTest(t, vectorMult1.getValue(2), 15.7, 1)
    floatTest(t, vectorMult2.getValue(0), -13.8, 1)
    floatTest(t, vectorMult2.getValue(1), -10, 0)
    floatTest(t, vectorMult2.getValue(2), -6.28, 2)
    floatTest(t, vectorMult3.getValue(0), 10.35, 2)
    floatTest(t, vectorMult3.getValue(1), 7.5, 1)
    floatTest(t, vectorMult3.getValue(2), 4.71, 2)
    floatTest(t, vectorMult4.getValue(0), 3.45, 2)
    floatTest(t, vectorMult4.getValue(1), 2.5, 1)
    floatTest(t, vectorMult4.getValue(2), 1.57, 2)
})

test('div', t => {
    var vector = new Module.VectorDouble(3)
    vector.setValue(0, 3.14)
    vector.setValue(1, 10)
    vector.setValue(2, -4.3)
    var vectorDiv1 = vector.div(-2)
    var vectorDiv2 = vector.div(1.6)
    var vectorDiv3 = vector.div(0.5)
    floatTest(t, vectorDiv1.getValue(0), -1.57, 2)
    floatTest(t, vectorDiv1.getValue(1), -5, 0)
    floatTest(t, vectorDiv1.getValue(2), 2.15, 2)
    floatTest(t, vectorDiv2.getValue(0), 1.9625, 4)
    floatTest(t, vectorDiv2.getValue(1), 6.25, 2)
    floatTest(t, vectorDiv2.getValue(2), -2.6875, 4)
    floatTest(t, vectorDiv3.getValue(0), 6.28, 2)
    floatTest(t, vectorDiv3.getValue(1), 20, 0)
    floatTest(t, vectorDiv3.getValue(2), -8.6, 1)
})

test('dot', t => {
    var vector1 = new Module.VectorDouble(3)
    var vector2 = new Module.VectorDouble(3)
    vector1.setValue(0, 1.1)
    vector1.setValue(1, 2.2)
    vector1.setValue(2, 3.14)
    vector2.setValue(0, 5)
    vector2.setValue(1, -4)
    vector2.setValue(2, 9.6)
    5.5 - 8.8 + 30.144
    var dot1 = vector1.dot(vector2)
    var dot2 = vector2.dot(vector1)
    floatTest(t, dot1, 26.844, 3)
    t.is(dot1, dot2)
})

test.failing('exceptions with dot', t => {
    var vector1 = new Module.VectorDouble(2)
    var vector2 = new Module.VectorDouble(3)
    var vectorPlus1 = vector1.dot(vector2)
    var vectorPlus2 = vector2.dot(vector1)
})