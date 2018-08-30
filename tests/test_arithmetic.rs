#[macro_use]
extern crate lazy_static;
extern crate serde_json;

mod common;

use common::{load_tests, run_test};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref tests: HashMap<String, Value> = load_tests("tests/vmArithmeticTest/");
}

#[test]
fn test_add0() {
    assert!(run_test(&tests["add0"]));
}
#[test]
fn test_add1() {
    assert!(run_test(&tests["add1"]));
}
#[test]
fn test_add2() {
    assert!(run_test(&tests["add2"]));
}
#[test]
fn test_add3() {
    assert!(run_test(&tests["add3"]));
}
#[test]
fn test_add4() {
    assert!(run_test(&tests["add4"]));
}
#[test]
fn test_addmod0() {
    assert!(run_test(&tests["addmod0"]));
}
#[test]
fn test_addmod1() {
    assert!(run_test(&tests["addmod1"]));
}
#[test]
fn test_addmod1_overflow2() {
    assert!(run_test(&tests["addmod1_overflow2"]));
}
#[test]
fn test_addmod1_overflow3() {
    assert!(run_test(&tests["addmod1_overflow3"]));
}
#[test]
fn test_addmod1_overflow4() {
    assert!(run_test(&tests["addmod1_overflow4"]));
}
#[test]
fn test_addmod1_overflowDiff() {
    assert!(run_test(&tests["addmod1_overflowDiff"]));
}
#[test]
fn test_addmod2() {
    assert!(run_test(&tests["addmod2"]));
}
#[test]
fn test_addmod2_0() {
    assert!(run_test(&tests["addmod2_0"]));
}
#[test]
fn test_addmod2_1() {
    assert!(run_test(&tests["addmod2_1"]));
}
#[test]
fn test_addmod3() {
    assert!(run_test(&tests["addmod3"]));
}
#[test]
fn test_addmod3_0() {
    assert!(run_test(&tests["addmod3_0"]));
}
#[test]
fn test_addmodBigIntCast() {
    assert!(run_test(&tests["addmodBigIntCast"]));
}
#[test]
fn test_addmodDivByZero() {
    assert!(run_test(&tests["addmodDivByZero"]));
}
#[test]
fn test_addmodDivByZero1() {
    assert!(run_test(&tests["addmodDivByZero1"]));
}
#[test]
fn test_addmodDivByZero2() {
    assert!(run_test(&tests["addmodDivByZero2"]));
}
#[test]
fn test_addmodDivByZero3() {
    assert!(run_test(&tests["addmodDivByZero3"]));
}
// #[test]
// fn test_arith1() {
//     assert!(run_test(&tests["arith1"]));
// }
// #[test]
// fn test_div1() {
//     assert!(run_test(&tests["div1"]));
// }
// #[test]
// fn test_divBoostBug() {
//     assert!(run_test(&tests["divBoostBug"]));
// }
// #[test]
// fn test_divByNonZero0() {
//     assert!(run_test(&tests["divByNonZero0"]));
// }
// #[test]
// fn test_divByNonZero1() {
//     assert!(run_test(&tests["divByNonZero1"]));
// }
// #[test]
// fn test_divByNonZero2() {
//     assert!(run_test(&tests["divByNonZero2"]));
// }
// #[test]
// fn test_divByNonZero3() {
//     assert!(run_test(&tests["divByNonZero3"]));
// }
// #[test]
// fn test_divByZero() {
//     assert!(run_test(&tests["divByZero"]));
// }
// #[test]
// fn test_divByZero_2() {
//     assert!(run_test(&tests["divByZero_2"]));
// }
// #[test]
// fn test_exp0() {
//     assert!(run_test(&tests["exp0"]));
// }
// #[test]
// fn test_exp1() {
//     assert!(run_test(&tests["exp1"]));
// }
// #[test]
// fn test_exp2() {
//     assert!(run_test(&tests["exp2"]));
// }
// #[test]
// fn test_exp3() {
//     assert!(run_test(&tests["exp3"]));
// }
// #[test]
// fn test_exp4() {
//     assert!(run_test(&tests["exp4"]));
// }
// #[test]
// fn test_exp5() {
//     assert!(run_test(&tests["exp5"]));
// }
// #[test]
// fn test_exp6() {
//     assert!(run_test(&tests["exp6"]));
// }
// #[test]
// fn test_exp7() {
//     assert!(run_test(&tests["exp7"]));
// }
// #[test]
// fn test_exp8() {
//     assert!(run_test(&tests["exp8"]));
// }
// #[test]
// fn test_expPowerOf2_128() {
//     assert!(run_test(&tests["expPowerOf2_128"]));
// }
// #[test]
// fn test_expPowerOf2_16() {
//     assert!(run_test(&tests["expPowerOf2_16"]));
// }
// #[test]
// fn test_expPowerOf2_2() {
//     assert!(run_test(&tests["expPowerOf2_2"]));
// }
// #[test]
// fn test_expPowerOf2_256() {
//     assert!(run_test(&tests["expPowerOf2_256"]));
// }
// #[test]
// fn test_expPowerOf2_32() {
//     assert!(run_test(&tests["expPowerOf2_32"]));
// }
// #[test]
// fn test_expPowerOf2_4() {
//     assert!(run_test(&tests["expPowerOf2_4"]));
// }
// #[test]
// fn test_expPowerOf256_1() {
//     assert!(run_test(&tests["expPowerOf256_1"]));
// }
// #[test]
// fn test_expPowerOf256_10() {
//     assert!(run_test(&tests["expPowerOf256_10"]));
// }
// #[test]
// fn test_expPowerOf256_11() {
//     assert!(run_test(&tests["expPowerOf256_11"]));
// }
// #[test]
// fn test_expPowerOf256_12() {
//     assert!(run_test(&tests["expPowerOf256_12"]));
// }
// #[test]
// fn test_expPowerOf256_13() {
//     assert!(run_test(&tests["expPowerOf256_13"]));
// }
// #[test]
// fn test_expPowerOf256_14() {
//     assert!(run_test(&tests["expPowerOf256_14"]));
// }
// #[test]
// fn test_expPowerOf256_15() {
//     assert!(run_test(&tests["expPowerOf256_15"]));
// }
// #[test]
// fn test_expPowerOf256_16() {
//     assert!(run_test(&tests["expPowerOf256_16"]));
// }
// #[test]
// fn test_expPowerOf256_17() {
//     assert!(run_test(&tests["expPowerOf256_17"]));
// }
// #[test]
// fn test_expPowerOf256_18() {
//     assert!(run_test(&tests["expPowerOf256_18"]));
// }
// #[test]
// fn test_expPowerOf256_19() {
//     assert!(run_test(&tests["expPowerOf256_19"]));
// }
// #[test]
// fn test_expPowerOf256_2() {
//     assert!(run_test(&tests["expPowerOf256_2"]));
// }
// #[test]
// fn test_expPowerOf256_20() {
//     assert!(run_test(&tests["expPowerOf256_20"]));
// }
// #[test]
// fn test_expPowerOf256_21() {
//     assert!(run_test(&tests["expPowerOf256_21"]));
// }
// #[test]
// fn test_expPowerOf256_22() {
//     assert!(run_test(&tests["expPowerOf256_22"]));
// }
// #[test]
// fn test_expPowerOf256_23() {
//     assert!(run_test(&tests["expPowerOf256_23"]));
// }
// #[test]
// fn test_expPowerOf256_24() {
//     assert!(run_test(&tests["expPowerOf256_24"]));
// }
// #[test]
// fn test_expPowerOf256_25() {
//     assert!(run_test(&tests["expPowerOf256_25"]));
// }
// #[test]
// fn test_expPowerOf256_26() {
//     assert!(run_test(&tests["expPowerOf256_26"]));
// }
// #[test]
// fn test_expPowerOf256_27() {
//     assert!(run_test(&tests["expPowerOf256_27"]));
// }
// #[test]
// fn test_expPowerOf256_28() {
//     assert!(run_test(&tests["expPowerOf256_28"]));
// }
// #[test]
// fn test_expPowerOf256_29() {
//     assert!(run_test(&tests["expPowerOf256_29"]));
// }
// #[test]
// fn test_expPowerOf256_3() {
//     assert!(run_test(&tests["expPowerOf256_3"]));
// }
// #[test]
// fn test_expPowerOf256_30() {
//     assert!(run_test(&tests["expPowerOf256_30"]));
// }
// #[test]
// fn test_expPowerOf256_31() {
//     assert!(run_test(&tests["expPowerOf256_31"]));
// }
// #[test]
// fn test_expPowerOf256_32() {
//     assert!(run_test(&tests["expPowerOf256_32"]));
// }
// #[test]
// fn test_expPowerOf256_33() {
//     assert!(run_test(&tests["expPowerOf256_33"]));
// }
// #[test]
// fn test_expPowerOf256_4() {
//     assert!(run_test(&tests["expPowerOf256_4"]));
// }
// #[test]
// fn test_expPowerOf256_5() {
//     assert!(run_test(&tests["expPowerOf256_5"]));
// }
// #[test]
// fn test_expPowerOf256_6() {
//     assert!(run_test(&tests["expPowerOf256_6"]));
// }
// #[test]
// fn test_expPowerOf256_7() {
//     assert!(run_test(&tests["expPowerOf256_7"]));
// }
// #[test]
// fn test_expPowerOf256_8() {
//     assert!(run_test(&tests["expPowerOf256_8"]));
// }
// #[test]
// fn test_expPowerOf256_9() {
//     assert!(run_test(&tests["expPowerOf256_9"]));
// }
// #[test]
// fn test_expPowerOf256Of256_0() {
//     assert!(run_test(&tests["expPowerOf256Of256_0"]));
// }
// #[test]
// fn test_expPowerOf256Of256_1() {
//     assert!(run_test(&tests["expPowerOf256Of256_1"]));
// }
// #[test]
// fn test_expPowerOf256Of256_10() {
//     assert!(run_test(&tests["expPowerOf256Of256_10"]));
// }
// #[test]
// fn test_expPowerOf256Of256_11() {
//     assert!(run_test(&tests["expPowerOf256Of256_11"]));
// }
// #[test]
// fn test_expPowerOf256Of256_12() {
//     assert!(run_test(&tests["expPowerOf256Of256_12"]));
// }
// #[test]
// fn test_expPowerOf256Of256_13() {
//     assert!(run_test(&tests["expPowerOf256Of256_13"]));
// }
// #[test]
// fn test_expPowerOf256Of256_14() {
//     assert!(run_test(&tests["expPowerOf256Of256_14"]));
// }
// #[test]
// fn test_expPowerOf256Of256_15() {
//     assert!(run_test(&tests["expPowerOf256Of256_15"]));
// }
// #[test]
// fn test_expPowerOf256Of256_16() {
//     assert!(run_test(&tests["expPowerOf256Of256_16"]));
// }
// #[test]
// fn test_expPowerOf256Of256_17() {
//     assert!(run_test(&tests["expPowerOf256Of256_17"]));
// }
// #[test]
// fn test_expPowerOf256Of256_18() {
//     assert!(run_test(&tests["expPowerOf256Of256_18"]));
// }
// #[test]
// fn test_expPowerOf256Of256_19() {
//     assert!(run_test(&tests["expPowerOf256Of256_19"]));
// }
// #[test]
// fn test_expPowerOf256Of256_2() {
//     assert!(run_test(&tests["expPowerOf256Of256_2"]));
// }
// #[test]
// fn test_expPowerOf256Of256_20() {
//     assert!(run_test(&tests["expPowerOf256Of256_20"]));
// }
// #[test]
// fn test_expPowerOf256Of256_21() {
//     assert!(run_test(&tests["expPowerOf256Of256_21"]));
// }
// #[test]
// fn test_expPowerOf256Of256_22() {
//     assert!(run_test(&tests["expPowerOf256Of256_22"]));
// }
// #[test]
// fn test_expPowerOf256Of256_23() {
//     assert!(run_test(&tests["expPowerOf256Of256_23"]));
// }
// #[test]
// fn test_expPowerOf256Of256_24() {
//     assert!(run_test(&tests["expPowerOf256Of256_24"]));
// }
// #[test]
// fn test_expPowerOf256Of256_25() {
//     assert!(run_test(&tests["expPowerOf256Of256_25"]));
// }
// #[test]
// fn test_expPowerOf256Of256_26() {
//     assert!(run_test(&tests["expPowerOf256Of256_26"]));
// }
// #[test]
// fn test_expPowerOf256Of256_27() {
//     assert!(run_test(&tests["expPowerOf256Of256_27"]));
// }
// #[test]
// fn test_expPowerOf256Of256_28() {
//     assert!(run_test(&tests["expPowerOf256Of256_28"]));
// }
// #[test]
// fn test_expPowerOf256Of256_29() {
//     assert!(run_test(&tests["expPowerOf256Of256_29"]));
// }
// #[test]
// fn test_expPowerOf256Of256_3() {
//     assert!(run_test(&tests["expPowerOf256Of256_3"]));
// }
// #[test]
// fn test_expPowerOf256Of256_30() {
//     assert!(run_test(&tests["expPowerOf256Of256_30"]));
// }
// #[test]
// fn test_expPowerOf256Of256_31() {
//     assert!(run_test(&tests["expPowerOf256Of256_31"]));
// }
// #[test]
// fn test_expPowerOf256Of256_32() {
//     assert!(run_test(&tests["expPowerOf256Of256_32"]));
// }
// #[test]
// fn test_expPowerOf256Of256_33() {
//     assert!(run_test(&tests["expPowerOf256Of256_33"]));
// }
// #[test]
// fn test_expPowerOf256Of256_4() {
//     assert!(run_test(&tests["expPowerOf256Of256_4"]));
// }
// #[test]
// fn test_expPowerOf256Of256_5() {
//     assert!(run_test(&tests["expPowerOf256Of256_5"]));
// }
// #[test]
// fn test_expPowerOf256Of256_6() {
//     assert!(run_test(&tests["expPowerOf256Of256_6"]));
// }
// #[test]
// fn test_expPowerOf256Of256_7() {
//     assert!(run_test(&tests["expPowerOf256Of256_7"]));
// }
// #[test]
// fn test_expPowerOf256Of256_8() {
//     assert!(run_test(&tests["expPowerOf256Of256_8"]));
// }
// #[test]
// fn test_expPowerOf256Of256_9() {
//     assert!(run_test(&tests["expPowerOf256Of256_9"]));
// }
// #[test]
// fn test_expPowerOf2_64() {
//     assert!(run_test(&tests["expPowerOf2_64"]));
// }
// #[test]
// fn test_expPowerOf2_8() {
//     assert!(run_test(&tests["expPowerOf2_8"]));
// }
// #[test]
// fn test_expXY() {
//     assert!(run_test(&tests["expXY"]));
// }
// #[test]
// fn test_expXY_success() {
//     assert!(run_test(&tests["expXY_success"]));
// }
// #[test]
// fn test_fibbonacci_unrolled() {
//     assert!(run_test(&tests["fibbonacci_unrolled"]));
// }
// #[test]
// fn test_mod0() {
//     assert!(run_test(&tests["mod0"]));
// }
// #[test]
// fn test_mod1() {
//     assert!(run_test(&tests["mod1"]));
// }
// #[test]
// fn test_mod2() {
//     assert!(run_test(&tests["mod2"]));
// }
// #[test]
// fn test_mod3() {
//     assert!(run_test(&tests["mod3"]));
// }
// #[test]
// fn test_mod4() {
//     assert!(run_test(&tests["mod4"]));
// }
// #[test]
// fn test_modByZero() {
//     assert!(run_test(&tests["modByZero"]));
// }
// #[test]
// fn test_mul0() {
//     assert!(run_test(&tests["mul0"]));
// }
// #[test]
// fn test_mul1() {
//     assert!(run_test(&tests["mul1"]));
// }
// #[test]
// fn test_mul2() {
//     assert!(run_test(&tests["mul2"]));
// }
// #[test]
// fn test_mul3() {
//     assert!(run_test(&tests["mul3"]));
// }
// #[test]
// fn test_mul4() {
//     assert!(run_test(&tests["mul4"]));
// }
// #[test]
// fn test_mul5() {
//     assert!(run_test(&tests["mul5"]));
// }
// #[test]
// fn test_mul6() {
//     assert!(run_test(&tests["mul6"]));
// }
// #[test]
// fn test_mul7() {
//     assert!(run_test(&tests["mul7"]));
// }
// #[test]
// fn test_mulmod0() {
//     assert!(run_test(&tests["mulmod0"]));
// }
// #[test]
// fn test_mulmod1() {
//     assert!(run_test(&tests["mulmod1"]));
// }
// #[test]
// fn test_mulmod1_overflow() {
//     assert!(run_test(&tests["mulmod1_overflow"]));
// }
// #[test]
// fn test_mulmod1_overflow2() {
//     assert!(run_test(&tests["mulmod1_overflow2"]));
// }
// #[test]
// fn test_mulmod1_overflow3() {
//     assert!(run_test(&tests["mulmod1_overflow3"]));
// }
// #[test]
// fn test_mulmod1_overflow4() {
//     assert!(run_test(&tests["mulmod1_overflow4"]));
// }
// #[test]
// fn test_mulmod2() {
//     assert!(run_test(&tests["mulmod2"]));
// }
// #[test]
// fn test_mulmod2_0() {
//     assert!(run_test(&tests["mulmod2_0"]));
// }
// #[test]
// fn test_mulmod2_1() {
//     assert!(run_test(&tests["mulmod2_1"]));
// }
// #[test]
// fn test_mulmod3() {
//     assert!(run_test(&tests["mulmod3"]));
// }
// #[test]
// fn test_mulmod3_0() {
//     assert!(run_test(&tests["mulmod3_0"]));
// }
// #[test]
// fn test_mulmod4() {
//     assert!(run_test(&tests["mulmod4"]));
// }
// #[test]
// fn test_mulmoddivByZero() {
//     assert!(run_test(&tests["mulmoddivByZero"]));
// }
// #[test]
// fn test_mulmoddivByZero1() {
//     assert!(run_test(&tests["mulmoddivByZero1"]));
// }
// #[test]
// fn test_mulmoddivByZero2() {
//     assert!(run_test(&tests["mulmoddivByZero2"]));
// }
// #[test]
// fn test_mulmoddivByZero3() {
//     assert!(run_test(&tests["mulmoddivByZero3"]));
// }
// #[test]
// fn test_mulUnderFlow() {
//     assert!(run_test(&tests["mulUnderFlow"]));
// }
// #[test]
// fn test_not1() {
//     assert!(run_test(&tests["not1"]));
// }
// #[test]
// fn test_sdiv0() {
//     assert!(run_test(&tests["sdiv0"]));
// }
// #[test]
// fn test_sdiv1() {
//     assert!(run_test(&tests["sdiv1"]));
// }
// #[test]
// fn test_sdiv2() {
//     assert!(run_test(&tests["sdiv2"]));
// }
// #[test]
// fn test_sdiv3() {
//     assert!(run_test(&tests["sdiv3"]));
// }
// #[test]
// fn test_sdiv4() {
//     assert!(run_test(&tests["sdiv4"]));
// }
// #[test]
// fn test_sdiv5() {
//     assert!(run_test(&tests["sdiv5"]));
// }
// #[test]
// fn test_sdiv6() {
//     assert!(run_test(&tests["sdiv6"]));
// }
// #[test]
// fn test_sdiv7() {
//     assert!(run_test(&tests["sdiv7"]));
// }
// #[test]
// fn test_sdiv8() {
//     assert!(run_test(&tests["sdiv8"]));
// }
// #[test]
// fn test_sdiv9() {
//     assert!(run_test(&tests["sdiv9"]));
// }
// #[test]
// fn test_sdivByZero0() {
//     assert!(run_test(&tests["sdivByZero0"]));
// }
// #[test]
// fn test_sdivByZero1() {
//     assert!(run_test(&tests["sdivByZero1"]));
// }
// #[test]
// fn test_sdivByZero2() {
//     assert!(run_test(&tests["sdivByZero2"]));
// }
// #[test]
// fn test_sdiv_dejavu() {
//     assert!(run_test(&tests["sdiv_dejavu"]));
// }
// #[test]
// fn test_sdiv_i256min() {
//     assert!(run_test(&tests["sdiv_i256min"]));
// }
// #[test]
// fn test_sdiv_i256min2() {
//     assert!(run_test(&tests["sdiv_i256min2"]));
// }
// #[test]
// fn test_sdiv_i256min3() {
//     assert!(run_test(&tests["sdiv_i256min3"]));
// }
// #[test]
// fn test_signextend_00() {
//     assert!(run_test(&tests["signextend_00"]));
// }
// #[test]
// fn test_signextend_0_BigByte() {
//     assert!(run_test(&tests["signextend_0_BigByte"]));
// }
// #[test]
// fn test_signextend_AlmostBiggestByte() {
//     assert!(run_test(&tests["signextend_AlmostBiggestByte"]));
// }
// #[test]
// fn test_signextend_BigByte_0() {
//     assert!(run_test(&tests["signextend_BigByte_0"]));
// }
// #[test]
// fn test_signextend_BigByteBigByte() {
//     assert!(run_test(&tests["signextend_BigByteBigByte"]));
// }
// #[test]
// fn test_signextend_bigBytePlus1() {
//     assert!(run_test(&tests["signextend_bigBytePlus1"]));
// }
// #[test]
// fn test_signextend_BigBytePlus1_2() {
//     assert!(run_test(&tests["signextend_BigBytePlus1_2"]));
// }
// #[test]
// fn test_signextend_BitIsNotSet() {
//     assert!(run_test(&tests["signextend_BitIsNotSet"]));
// }
// #[test]
// fn test_signextend_BitIsNotSetInHigherByte() {
//     assert!(run_test(&tests["signextend_BitIsNotSetInHigherByte"]));
// }
// #[test]
// fn test_signextend_bitIsSet() {
//     assert!(run_test(&tests["signextend_bitIsSet"]));
// }
// #[test]
// fn test_signextend_BitIsSetInHigherByte() {
//     assert!(run_test(&tests["signextend_BitIsSetInHigherByte"]));
// }
// #[test]
// fn test_signextendInvalidByteNumber() {
//     assert!(run_test(&tests["signextendInvalidByteNumber"]));
// }
// #[test]
// fn test_signextend_Overflow_dj42() {
//     assert!(run_test(&tests["signextend_Overflow_dj42"]));
// }
// #[test]
// fn test_smod0() {
//     assert!(run_test(&tests["smod0"]));
// }
// #[test]
// fn test_smod1() {
//     assert!(run_test(&tests["smod1"]));
// }
// #[test]
// fn test_smod2() {
//     assert!(run_test(&tests["smod2"]));
// }
// #[test]
// fn test_smod3() {
//     assert!(run_test(&tests["smod3"]));
// }
// #[test]
// fn test_smod4() {
//     assert!(run_test(&tests["smod4"]));
// }
// #[test]
// fn test_smod5() {
//     assert!(run_test(&tests["smod5"]));
// }
// #[test]
// fn test_smod6() {
//     assert!(run_test(&tests["smod6"]));
// }
// #[test]
// fn test_smod7() {
//     assert!(run_test(&tests["smod7"]));
// }
// #[test]
// fn test_smod8_byZero() {
//     assert!(run_test(&tests["smod8_byZero"]));
// }
// #[test]
// fn test_smod_i256min1() {
//     assert!(run_test(&tests["smod_i256min1"]));
// }
// #[test]
// fn test_smod_i256min2() {
//     assert!(run_test(&tests["smod_i256min2"]));
// }
// #[test]
// fn test_stop() {
//     assert!(run_test(&tests["stop"]));
// }
// #[test]
// fn test_sub0() {
//     assert!(run_test(&tests["sub0"]));
// }
// #[test]
// fn test_sub1() {
//     assert!(run_test(&tests["sub1"]));
// }
// #[test]
// fn test_sub2() {
//     assert!(run_test(&tests["sub2"]));
// }
// #[test]
// fn test_sub3() {
//     assert!(run_test(&tests["sub3"]));
// }
// #[test]
// fn test_sub4() {
//     assert!(run_test(&tests["sub4"]));
// }
