import test from 'ava';
import Module from './_Warp'

test('constructor', t => {
    var vector = new Module.VectorInt(3);
    vector.setValue(0, 1)

    t.is(vector.getValue(0), 1)
})