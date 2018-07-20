
describe('MatD', () => {
    var warp;
    var VecD;
    var MatD;

    beforeAll(() => {
        return import('../../../wasm/warp_engine').then(module => {
            warp = module
            VecD = warp.VecD
            MatD = warp.MatD
        });
    })

    it('can be constructed', () => {
        var mat1 = MatD.new(2, 3, [0, 1, 2, 3, 4, 5])
        expect(mat1).toBeDefined()
        var mat2 = MatD.with_dimension(5, 6)
        expect(mat2).toBeDefined()
    })

    it('throws exceptions when constructed with invalid dimensions', () => {
        expect(() => MatD.new(-1, 2, [0])).toThrow()
        expect(() => MatD.new(2, 2, [0, 1])).toThrow()
        expect(() => MatD.new(1, 1, [0, 1])).toThrow()
        expect(() => MatD.with_dimension(-1, 2)).toThrow()
    })

    it('returns the correct dimensions', () => {
        var mat1 = MatD.new(2, 2, [0, 1, 2, 3])
        var mat2 = MatD.with_dimension(3, 5)
        expect(mat1.rows()).toBe(2)
        expect(mat1.cols()).toBe(2)
        expect(mat2.rows()).toBe(3)
        expect(mat2.cols()).toBe(5)
    })

    it('has getters and setters that return the correct elements', () => {
        var mat = MatD.new(2, 2, [0, 1, 2, 3])

        expect(mat.get(0, 0)).toBe(0)
        expect(mat.get(0, 1)).toBe(1)
        expect(mat.get(1, 0)).toBe(2)
        expect(mat.get(1, 1)).toBe(3)
        mat.set(0, 0, 5)
        mat.set(1, 1, -9)
        expect(mat.get(0, 0)).toBe(5)
        expect(mat.get(1, 1)).toBe(-9)
    })

    it('can be transposed', () => {
        var mat = MatD.new(2, 2, [0, 1, 2, 3])
        var trans = mat.transpose()

        expect(trans.get(0, 0)).toBe(0)
        expect(trans.get(0, 1)).toBe(2)
        expect(trans.get(1, 0)).toBe(1)
        expect(trans.get(1, 1)).toBe(3)
    })

    it('can be multiplied to a VecD', () => {
        var mat = MatD.new(2, 2, [0, 1, 2, 3])
        var vec = VecD.new([-1, 2.5])
        var res = mat.vec_mult(vec)
        expect(res.get(0)).toBe(2.5)
        expect(res.get(1)).toBe(5.5)
    })

    it('can be multiplied to a MatD', () => {
        var mat1 = MatD.new(3, 2, [0, 1, 2, 3, 4, 5])
        var mat2 = MatD.new(2, 3, [0, 1, 2, 3, 4, 5])
        var res = mat1.mat_mult(mat2)

        expect(res.rows()).toBe(3)
        expect(res.cols()).toBe(3)
        expect(res.get(0, 0)).toBe(3)
        expect(res.get(0, 1)).toBe(4)
        expect(res.get(0, 2)).toBe(5)
        expect(res.get(1, 0)).toBe(9)
        expect(res.get(1, 1)).toBe(14)
        expect(res.get(1, 2)).toBe(19)
        expect(res.get(2, 0)).toBe(15)
        expect(res.get(2, 1)).toBe(24)
        expect(res.get(2, 2)).toBe(33)
    })

    it('throws exceptions when multiplying with invalid dimensions', () => {
        var mat = MatD.new(2, 2, [0, 1, 2, 3])
        var vec = VecD.new([-1, 2.5, 5])
        var mat1 = MatD.new(3, 2, [0, 1, 2, 3, 4, 5])
        expect(() => mat.vec_mult(vec)).toThrow()
        expect(() => mat.mat_mult(mat1)).toThrow()
    })

    it('can be added to another MatD', () => {
        var mat1 = MatD.new(2, 2, [0, 1, 2, 3])
        var mat2 = MatD.new(2, 2, [8, 6.2, 4, 5])
        var res = mat1.add(mat2)
        expect(res.get(0, 0)).toBe(8)
        expect(res.get(0, 1)).toBe(7.2)
        expect(res.get(1, 0)).toBe(6)
        expect(res.get(1, 1)).toBe(8)
    })

    it('can be subtracted to another MatD', () => {
        var mat1 = MatD.new(2, 2, [0, 1, 2, 3])
        var mat2 = MatD.new(2, 2, [8, 6.2, 4, 5])
        var res = mat2.sub(mat1)
        expect(res.get(0, 0)).toBe(8)
        expect(res.get(0, 1)).toBe(5.2)
        expect(res.get(1, 0)).toBe(2)
        expect(res.get(1, 1)).toBe(2)
    })

    it('throws exceptions when add/sub with different dimensions', () => {
        var mat1 = MatD.with_dimension(2, 2)
        var mat2 = MatD.with_dimension(3, 5)
        expect(() => mat1.add(mat2)).toThrow()
        expect(() => mat1.sub(mat2)).toThrow()
    })

    it('can be converted to a VecD', () => {
        var mat1 = MatD.new(2, 2, [3.1, 2, 8, 6])
        var res = VecD.from_mat(mat1)
        expect(res.len()).toBe(4)
        expect(res.get(0)).toBe(3.1)
        expect(res.get(1)).toBe(2)
        expect(res.get(2)).toBe(8)
        expect(res.get(3)).toBe(6)
    })
})