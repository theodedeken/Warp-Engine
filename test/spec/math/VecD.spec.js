function floatTest(t, input, expected, decimal) {
    var val = Math.round(input * Math.pow(10, decimal)) / Math.pow(10, decimal)
    t.is(val, expected)
}


describe('The VecD object', () => {
    var warpPromise = import('../../../wasm/warp_engine')
    var warp;
    var blocked = true
    beforeAll(() => {
        return warpPromise.then(module => warp = module);
    })

    it('can be constructed', () => {
        while (warp == undefined) {

        }
        var vector1 = warp.VecD.new([1, 2, 3]);
        expect(vector1).toBeDefined()
        var vector2 = warp.VecD.with_size(3);
        expect(vector2).toBeDefined();
    })
})
/*
test('constructor', t => {
    var vector1 = VecD.new([1, 2, 3]);
    t.not(vector1, null);
    var vector2 = VecD.with_size(3);
    t.not(vector2, null);
})

test('constructor exceptions negative size', t => {
    t.throws(() => VecD.with_size(-1))
})

test('get size', t => {
    var vector = VecD.with_size(1)
    t.is(vector.len(), 1)
    vector = VecD.with_size(0)
    t.is(vector.len(), 0)
    vector = VecD.with_size(123)
    t.is(vector.len(), 123)
    // constructor with array seems to be broken right now
    //vector = VecD.new(new Float64Array([1.3, 5.2, 6.3]))
    //t.is(vector.len(), 3)
})

test('set and get value', t => {
    var vector = VecD.with_size(3)
    t.is(vector.get(0), 0)
    vector.set(1, 1.5)
    t.is(vector.get(1), 1.5)
    vector.set(2, -5.635)
    t.is(vector.get(0), 0)
    t.is(vector.get(1), 1.5)
    t.is(vector.get(2), -5.635)
})

test('exceptions when accessing elements out of bounds', t => {
    var vector = VecD.with_size(3)
    t.throws(() => vector.set(-1, 8))
    t.throws(() => vector.get(-5))
})

test('add', t => {
    var vector1 = VecD.with_size(3)
    var vector2 = VecD.with_size(3)
    vector1.set(0, -1.5)
    vector1.set(1, 3.14)
    vector2.set(0, 10)
    vector2.set(2, 7.6)
    var vectorPlus1 = vector1.add(vector2)
    var vectorPlus2 = vector2.add(vector1)
    t.is(vectorPlus1.get(0), 8.5)
    t.is(vectorPlus1.get(1), 3.14)
    t.is(vectorPlus1.get(2), 7.6)
    t.is(vectorPlus1.get(0), vectorPlus2.get(0))
    t.is(vectorPlus1.get(1), vectorPlus2.get(1))
    t.is(vectorPlus1.get(2), vectorPlus2.get(2))
})

test('exceptions with add', t => {
    var vector1 = VecD.with_size(2)
    var vector2 = VecD.with_size(3)
    t.throws(() => vector1.add(vector2))
    t.throws(() => vector2.add(vector1))
})

test('sub', t => {
    var vector1 = VecD.with_size(3)
    var vector2 = VecD.with_size(3)
    vector1.set(0, -1.5)
    vector1.set(1, 3.14)
    vector2.set(0, 10)
    vector2.set(2, 7.6)
    var vectorMinus1 = vector1.sub(vector2)
    var vectorMinus2 = vector2.sub(vector1)
    t.is(vectorMinus1.get(0), -11.5)
    t.is(vectorMinus1.get(1), 3.14)
    t.is(vectorMinus1.get(2), -7.6)
    t.is(vectorMinus2.get(0), 11.5)
    t.is(vectorMinus2.get(1), -3.14)
    t.is(vectorMinus2.get(2), 7.6)
})

test('exceptions with sub', t => {
    var vector1 = VecD.with_size(2)
    var vector2 = VecD.with_size(3)
    t.throws(() => vector1.minus(vector2))
    t.throws(() => vector2.minus(vector1))
})

test('mult', t => {
    var vector = VecD.with_size(3)
    vector.set(0, 6.9)
    vector.set(1, 5)
    vector.set(2, 3.14)
    var vectorMult1 = vector.mult(5)
    var vectorMult2 = vector.mult(-2)
    var vectorMult3 = vector.mult(1.5)
    var vectorMult4 = vector.mult(0.5)
    floatTest(t, vectorMult1.get(0), 34.5, 1)
    floatTest(t, vectorMult1.get(1), 25, 0)
    floatTest(t, vectorMult1.get(2), 15.7, 1)
    floatTest(t, vectorMult2.get(0), -13.8, 1)
    floatTest(t, vectorMult2.get(1), -10, 0)
    floatTest(t, vectorMult2.get(2), -6.28, 2)
    floatTest(t, vectorMult3.get(0), 10.35, 2)
    floatTest(t, vectorMult3.get(1), 7.5, 1)
    floatTest(t, vectorMult3.get(2), 4.71, 2)
    floatTest(t, vectorMult4.get(0), 3.45, 2)
    floatTest(t, vectorMult4.get(1), 2.5, 1)
    floatTest(t, vectorMult4.get(2), 1.57, 2)
})

test('div', t => {
    var vector = VecD.with_size(3)
    vector.set(0, 3.14)
    vector.set(1, 10)
    vector.set(2, -4.3)
    var vectorDiv1 = vector.div(-2)
    var vectorDiv2 = vector.div(1.6)
    var vectorDiv3 = vector.div(0.5)
    floatTest(t, vectorDiv1.get(0), -1.57, 2)
    floatTest(t, vectorDiv1.get(1), -5, 0)
    floatTest(t, vectorDiv1.get(2), 2.15, 2)
    floatTest(t, vectorDiv2.get(0), 1.9625, 4)
    floatTest(t, vectorDiv2.get(1), 6.25, 2)
    floatTest(t, vectorDiv2.get(2), -2.6875, 4)
    floatTest(t, vectorDiv3.get(0), 6.28, 2)
    floatTest(t, vectorDiv3.get(1), 20, 0)
    floatTest(t, vectorDiv3.get(2), -8.6, 1)
})

test('dot', t => {
    var vector1 = VecD.with_size(3)
    var vector2 = VecD.with_size(3)
    vector1.set(0, 1.1)
    vector1.set(1, 2.2)
    vector1.set(2, 3.14)
    vector2.set(0, 5)
    vector2.set(1, -4)
    vector2.set(2, 9.6)
    var dot1 = vector1.dot(vector2)
    var dot2 = vector2.dot(vector1)
    floatTest(t, dot1, 26.844, 3)
    t.is(dot1, dot2)
})

test('exceptions with dot', t => {
    var vector1 = VecD.with_size(2)
    var vector2 = VecD.with_size(3)
    t.throws(() => vector1.dot(vector2))
    t.throws(() => vector2.dot(vector1))
})*/