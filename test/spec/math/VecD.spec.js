describe('VecD', () => {
    var warp;
    var VecD;
    var MatD;

    beforeAll(() => {
        return import('../../../wasm/warp_engine').then(module => {
            warp = module;
            VecD = warp.VecD;
            MatD = warp.MatD;
        });
    })

    it('can be constructed', () => {
        var vector1 = VecD.new([1, 2, 3]);
        expect(vector1).toBeDefined();
        var vector2 = VecD.with_size(3);
        expect(vector2).toBeDefined();
    })

    it('throws exceptions when provided with negative size', () => {
        expect(() => VecD.with_size(-1)).toThrow();
    })

    it('returns the correct size', () => {
        var vector = VecD.with_size(1);
        expect(vector.len()).toBe(1);
        vector = VecD.with_size(0);
        expect(vector.len()).toBe(0);
        vector = VecD.with_size(123);
        expect(vector.len()).toBe(123);
        vector = VecD.new(new Float64Array([1.3, 5.2, 6.3]));
        expect(vector.len()).toBe(3);
    })

    it('has getters and setters for its elements', () => {
        var vector = VecD.with_size(3);
        expect(vector.get(0)).toBe(0);
        vector.set(1, 1.5);
        expect(vector.get(1)).toBe(1.5);
        vector.set(2, -5.635)
        expect(vector.get(0)).toBe(0);
        expect(vector.get(1)).toBe(1.5);
        expect(vector.get(2)).toBe(-5.635);
        var vector2 = VecD.new([0, 12, 5]);
        expect(vector2.get(0)).toBe(0);
        expect(vector2.get(1)).toBe(12);
        expect(vector2.get(2)).toBe(5);
    })

    it('throws exceptions when accessing elements out of bounds', () => {
        var vector = VecD.with_size(3)
        expect(() => vector.set(3, 8)).toThrow()
        expect(() => vector.get(-5)).toThrow()
    })

    it('can be added to another VecD', () => {
        var vector1 = VecD.with_size(3)
        var vector2 = VecD.with_size(3)
        vector1.set(0, -1.5)
        vector1.set(1, 3.14)
        vector2.set(0, 10)
        vector2.set(2, 7.6)
        var vectorPlus1 = vector1.add(vector2)
        var vectorPlus2 = vector2.add(vector1)
        expect(vectorPlus1.get(0)).toBe(8.5)
        expect(vectorPlus1.get(1)).toBe(3.14)
        expect(vectorPlus1.get(2)).toBe(7.6)
        expect(vectorPlus1.get(0)).toBe(vectorPlus2.get(0))
        expect(vectorPlus1.get(1)).toBe(vectorPlus2.get(1))
        expect(vectorPlus1.get(2)).toBe(vectorPlus2.get(2))
    })

    it('throws exceptions when dimensions of adding/dot or subtracting dont correspond', () => {
        var vector1 = VecD.with_size(2)
        var vector2 = VecD.with_size(3)
        expect(() => vector1.add(vector2)).toThrow()
        expect(() => vector2.add(vector1)).toThrow()

        expect(() => vector1.minus(vector2)).toThrow()
        expect(() => vector2.minus(vector1)).toThrow()

        expect(() => vector1.dot(vector2)).toThrow()
        expect(() => vector2.dot(vector1)).toThrow()
    })

    it('can be subtracted by another VecD', () => {
        var vector1 = VecD.with_size(3)
        var vector2 = VecD.with_size(3)
        vector1.set(0, -1.5)
        vector1.set(1, 3.14)
        vector2.set(0, 10)
        vector2.set(2, 7.6)
        var vectorMinus1 = vector1.sub(vector2)
        var vectorMinus2 = vector2.sub(vector1)
        expect(vectorMinus1.get(0)).toBe(-11.5)
        expect(vectorMinus1.get(1)).toBe(3.14)
        expect(vectorMinus1.get(2)).toBe(-7.6)
        expect(vectorMinus2.get(0)).toBe(11.5)
        expect(vectorMinus2.get(1)).toBe(-3.14)
        expect(vectorMinus2.get(2)).toBe(7.6)
    })

    it('can be multiplied by a value', () => {
        var vector = VecD.with_size(3)
        vector.set(0, 6.9)
        vector.set(1, 5)
        vector.set(2, 3.14)
        var vectorMult1 = vector.mult(5)
        var vectorMult2 = vector.mult(-2)
        var vectorMult3 = vector.mult(1.5)
        var vectorMult4 = vector.mult(0.5)
        expect(vectorMult1.get(0)).toBeCloseTo(34.5, 1)
        expect(vectorMult1.get(1)).toBeCloseTo(25, 0)
        expect(vectorMult1.get(2)).toBeCloseTo(15.7, 1)
        expect(vectorMult2.get(0)).toBeCloseTo(-13.8, 1)
        expect(vectorMult2.get(1)).toBeCloseTo(-10, 0)
        expect(vectorMult2.get(2)).toBeCloseTo(-6.28, 2)
        expect(vectorMult3.get(0)).toBeCloseTo(10.35, 2)
        expect(vectorMult3.get(1)).toBeCloseTo(7.5, 1)
        expect(vectorMult3.get(2)).toBeCloseTo(4.71, 2)
        expect(vectorMult4.get(0)).toBeCloseTo(3.45, 2)
        expect(vectorMult4.get(1)).toBeCloseTo(2.5, 1)
        expect(vectorMult4.get(2)).toBeCloseTo(1.57, 2)
    })

    it('can be divided by a value', () => {
        var vector = VecD.with_size(3)
        vector.set(0, 3.14)
        vector.set(1, 10)
        vector.set(2, -4.3)
        var vectorDiv1 = vector.div(-2)
        var vectorDiv2 = vector.div(1.6)
        var vectorDiv3 = vector.div(0.5)
        expect(vectorDiv1.get(0)).toBeCloseTo(-1.57, 2)
        expect(vectorDiv1.get(1)).toBeCloseTo(-5, 0)
        expect(vectorDiv1.get(2)).toBeCloseTo(2.15, 2)
        expect(vectorDiv2.get(0)).toBeCloseTo(1.9625, 4)
        expect(vectorDiv2.get(1)).toBeCloseTo(6.25, 2)
        expect(vectorDiv2.get(2)).toBeCloseTo(-2.6875, 4)
        expect(vectorDiv3.get(0)).toBeCloseTo(6.28, 2)
        expect(vectorDiv3.get(1)).toBeCloseTo(20, 0)
        expect(vectorDiv3.get(2)).toBeCloseTo(-8.6, 1)
    })

    it('can calculate dot product', () => {
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
        expect(dot1).toBeCloseTo(26.844, 3)
        expect(dot1).toBe(dot2)
    })

    it('can be converted to a matrix', () => {
        var vector = VecD.new([0, 1, 2, 3])
        var res = MatD.from_vec(2, 2, vector);
        expect(res.get(0, 0)).toBe(0)
        expect(res.get(0, 1)).toBe(1)
        expect(res.get(1, 0)).toBe(2)
        expect(res.get(1, 1)).toBe(3)
    })
})
