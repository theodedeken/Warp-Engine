import test from 'ava';
import Module from '../_Warp'

test('constructor', t => {
    var vector = new Module.VectorInt(3)
    t.not(vector, null)
})

test.failing('constructor exceptions negative size', t => {
    var vector = new Module.VectorInt(-1)
})

test('get size', t => {
    var vector = new Module.VectorInt(1)
    t.is(vector.size, 1)
    vector = new Module.VectorInt(0)
    t.is(vector.size, 0)
    vector = new Module.VectorInt(123)
    t.is(vector.size, 123)
})

test('set and get value', t => {
    var vector = new Module.VectorInt(3)
    t.is(vector.getValue(0), 0)
    vector.setValue(1, 5)
    t.is(vector.getValue(1), 5)
    vector.setValue(2, -5)
    t.is(vector.getValue(0), 0)
    t.is(vector.getValue(1), 5)
    t.is(vector.getValue(2), -5)
})

test.failing('exceptions when accessing elements out of bounds', t => {
    var vector = new Module.VectorInt(3)
    vector.setValue(-1, 8)
    vector.getValue(-5)
})

test('plus', t => {
    var vector1 = new Module.VectorInt(3)
    var vector2 = new Module.VectorInt(3)
    vector1.setValue(0,-5)
    vector1.setValue(1, 2)
    vector2.setValue(0, 10)
    vector2.setValue(2, 3)
    var vectorPlus1 = vector1.plus(vector2)
    var vectorPlus2 = vector2.plus(vector1)
    t.is(vectorPlus1.getValue(0), 5)
    t.is(vectorPlus1.getValue(1), 2)
    t.is(vectorPlus1.getValue(2), 3)
    t.is(vectorPlus1.getValue(0), vectorPlus2.getValue(0))
    t.is(vectorPlus1.getValue(1), vectorPlus2.getValue(1))
    t.is(vectorPlus1.getValue(2), vectorPlus2.getValue(2))
})

test.failing('exceptions with plus', t => {
    var vector1 = new Module.VectorInt(2)
    var vector2 = new Module.VectorInt(3)
    var vectorPlus1 = vector1.plus(vector2)
    var vectorPlus2 = vector2.plus(vector1)
})

test('minus', t => {
    var vector1 = new Module.VectorInt(3)
    var vector2 = new Module.VectorInt(3)
    vector1.setValue(0,-5)
    vector1.setValue(1, 2)
    vector2.setValue(0, 10)
    vector2.setValue(2, 3)
    var vectorMinus1 = vector1.minus(vector2)
    var vectorMinus2 = vector2.minus(vector1)
    t.is(vectorMinus1.getValue(0), -15)
    t.is(vectorMinus1.getValue(1), 2)
    t.is(vectorMinus1.getValue(2), -3)
    t.is(vectorMinus2.getValue(0), 15)
    t.is(vectorMinus2.getValue(1), -2)
    t.is(vectorMinus2.getValue(2), 3)
})

test.failing('exceptions with minus', t => {
    var vector1 = new Module.VectorInt(2)
    var vector2 = new Module.VectorInt(3)
    var vectorPlus1 = vector1.minus(vector2)
    var vectorPlus2 = vector2.minus(vector1)
})

test('mult', t => {
    var vector = new Module.VectorInt(3)
    vector.setValue(0,1)
    vector.setValue(1,2)
    vector.setValue(2,3)
    var vectorMult1 = vector.mult(5)
    var vectorMult2 = vector.mult(-3)
    var vectorMult3 = vector.mult(1.5)
    t.is(vectorMult1.getValue(0), 5)
    t.is(vectorMult1.getValue(1), 10)
    t.is(vectorMult1.getValue(2), 15)
    t.is(vectorMult2.getValue(0), -3)
    t.is(vectorMult2.getValue(1), -6)
    t.is(vectorMult2.getValue(2), -9)
    t.is(vectorMult3.getValue(0), 1)
    t.is(vectorMult3.getValue(1), 2)
    t.is(vectorMult3.getValue(2), 3)
})

test('div', t => {
    var vector = new Module.VectorInt(3)
    vector.setValue(0, 4)
    vector.setValue(1, 9)
    vector.setValue(2, 8)
    var vectorDiv1 = vector.div(2)
    var vectorDiv2 = vector.div(3)
    var vectorDiv3 = vector.div(1.5)
    t.is(vectorDiv1.getValue(0), 2)
    t.is(vectorDiv1.getValue(1), 4)
    t.is(vectorDiv1.getValue(2), 4)
    t.is(vectorDiv2.getValue(0), 1)
    t.is(vectorDiv2.getValue(1), 3)
    t.is(vectorDiv2.getValue(2), 2)    
    t.is(vectorDiv3.getValue(0), 4)
    t.is(vectorDiv3.getValue(1), 9)
    t.is(vectorDiv3.getValue(2), 8)

})

test('dot', t => {
    var vector1 = new Module.VectorInt(3)
    var vector2 = new Module.VectorInt(3)
    vector1.setValue(0, 1)
    vector1.setValue(1, 2)
    vector1.setValue(2, -3)
    vector2.setValue(0, 5)
    vector2.setValue(1, 4)
    vector2.setValue(2, 6)
    var dot1 = vector1.dot(vector2)
    var dot2 = vector2.dot(vector1)
    t.is(dot1, -5)
    t.is(dot1, dot2)
})

test.failing('exceptions with dot', t => {
    var vector1 = new Module.VectorInt(2)
    var vector2 = new Module.VectorInt(3)
    var vectorPlus1 = vector1.dot(vector2)
    var vectorPlus2 = vector2.dot(vector1)
})