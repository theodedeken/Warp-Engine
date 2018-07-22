const warp = import('../wasm/warp_engine');
import { Drive } from '../wasm/warp_drive';
var wapr;
var test;

warp.then(module => {
    test = new Drive('./script')
    test.load()
    test.spin()

    //module.temp_drive_test('./script');
})

