#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate vmtests;

use serde_json::Value;
use std::collections::HashMap;
use vmtests::{load_tests, run_test};

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmArithmeticTest/");
}

#[test]
fn test_add0() {
    assert!(run_test(&TESTS["add0"]));
}
#[test]
fn test_add1() {
    assert!(run_test(&TESTS["add1"]));
}
#[test]
fn test_add2() {
    assert!(run_test(&TESTS["add2"]));
}
#[test]
fn test_add3() {
    assert!(run_test(&TESTS["add3"]));
}
#[test]
fn test_add4() {
    assert!(run_test(&TESTS["add4"]));
}
#[test]
fn test_addmod0() {
    assert!(run_test(&TESTS["addmod0"]));
}
#[test]
fn test_addmod1() {
    assert!(run_test(&TESTS["addmod1"]));
}
#[test]
fn test_addmod1_overflow2() {
    assert!(run_test(&TESTS["addmod1_overflow2"]));
}
#[test]
fn test_addmod1_overflow3() {
    assert!(run_test(&TESTS["addmod1_overflow3"]));
}
#[test]
fn test_addmod1_overflow4() {
    assert!(run_test(&TESTS["addmod1_overflow4"]));
}
#[test]
fn test_addmod1_overflowDiff() {
    assert!(run_test(&TESTS["addmod1_overflowDiff"]));
}
#[test]
fn test_addmod2() {
    assert!(run_test(&TESTS["addmod2"]));
}
#[test]
fn test_addmod2_0() {
    assert!(run_test(&TESTS["addmod2_0"]));
}
#[test]
fn test_addmod2_1() {
    assert!(run_test(&TESTS["addmod2_1"]));
}
#[test]
fn test_addmod3() {
    assert!(run_test(&TESTS["addmod3"]));
}
#[test]
fn test_addmod3_0() {
    assert!(run_test(&TESTS["addmod3_0"]));
}
#[test]
fn test_addmodBigIntCast() {
    assert!(run_test(&TESTS["addmodBigIntCast"]));
}
#[test]
fn test_addmodDivByZero() {
    assert!(run_test(&TESTS["addmodDivByZero"]));
}
#[test]
fn test_addmodDivByZero1() {
    assert!(run_test(&TESTS["addmodDivByZero1"]));
}
#[test]
fn test_addmodDivByZero2() {
    assert!(run_test(&TESTS["addmodDivByZero2"]));
}
#[test]
fn test_addmodDivByZero3() {
    assert!(run_test(&TESTS["addmodDivByZero3"]));
}
#[test]
fn test_arith1() {
    assert!(run_test(&TESTS["arith1"]));
}
#[test]
fn test_div1() {
    assert!(run_test(&TESTS["div1"]));
}
#[test]
fn test_divBoostBug() {
    assert!(run_test(&TESTS["divBoostBug"]));
}
#[test]
fn test_divByNonZero0() {
    assert!(run_test(&TESTS["divByNonZero0"]));
}
#[test]
fn test_divByNonZero1() {
    assert!(run_test(&TESTS["divByNonZero1"]));
}
#[test]
fn test_divByNonZero2() {
    assert!(run_test(&TESTS["divByNonZero2"]));
}
#[test]
fn test_divByNonZero3() {
    assert!(run_test(&TESTS["divByNonZero3"]));
}
#[test]
fn test_divByZero() {
    assert!(run_test(&TESTS["divByZero"]));
}
#[test]
fn test_divByZero_2() {
    assert!(run_test(&TESTS["divByZero_2"]));
}
#[test]
fn test_exp0() {
    assert!(run_test(&TESTS["exp0"]));
}
#[test]
fn test_exp1() {
    assert!(run_test(&TESTS["exp1"]));
}
#[test]
fn test_exp2() {
    assert!(run_test(&TESTS["exp2"]));
}
#[test]
fn test_exp3() {
    assert!(run_test(&TESTS["exp3"]));
}
#[test]
fn test_exp4() {
    assert!(run_test(&TESTS["exp4"]));
}
#[test]
fn test_exp5() {
    assert!(run_test(&TESTS["exp5"]));
}
#[test]
fn test_exp6() {
    assert!(run_test(&TESTS["exp6"]));
}
#[test]
fn test_exp7() {
    assert!(run_test(&TESTS["exp7"]));
}
#[test]
fn test_exp8() {
    assert!(run_test(&TESTS["exp8"]));
}
#[test]
fn test_expPowerOf2_128() {
    assert!(run_test(&TESTS["expPowerOf2_128"]));
}
#[test]
fn test_expPowerOf2_16() {
    assert!(run_test(&TESTS["expPowerOf2_16"]));
}
#[test]
fn test_expPowerOf2_2() {
    assert!(run_test(&TESTS["expPowerOf2_2"]));
}
#[test]
fn test_expPowerOf2_256() {
    assert!(run_test(&TESTS["expPowerOf2_256"]));
}
#[test]
fn test_expPowerOf2_32() {
    assert!(run_test(&TESTS["expPowerOf2_32"]));
}
#[test]
fn test_expPowerOf2_4() {
    assert!(run_test(&TESTS["expPowerOf2_4"]));
}
#[test]
fn test_expPowerOf256_1() {
    assert!(run_test(&TESTS["expPowerOf256_1"]));
}
#[test]
fn test_expPowerOf256_10() {
    assert!(run_test(&TESTS["expPowerOf256_10"]));
}
#[test]
fn test_expPowerOf256_11() {
    assert!(run_test(&TESTS["expPowerOf256_11"]));
}
#[test]
fn test_expPowerOf256_12() {
    assert!(run_test(&TESTS["expPowerOf256_12"]));
}
#[test]
fn test_expPowerOf256_13() {
    assert!(run_test(&TESTS["expPowerOf256_13"]));
}
#[test]
fn test_expPowerOf256_14() {
    assert!(run_test(&TESTS["expPowerOf256_14"]));
}
#[test]
fn test_expPowerOf256_15() {
    assert!(run_test(&TESTS["expPowerOf256_15"]));
}
#[test]
fn test_expPowerOf256_16() {
    assert!(run_test(&TESTS["expPowerOf256_16"]));
}
#[test]
fn test_expPowerOf256_17() {
    assert!(run_test(&TESTS["expPowerOf256_17"]));
}
#[test]
fn test_expPowerOf256_18() {
    assert!(run_test(&TESTS["expPowerOf256_18"]));
}
#[test]
fn test_expPowerOf256_19() {
    assert!(run_test(&TESTS["expPowerOf256_19"]));
}
#[test]
fn test_expPowerOf256_2() {
    assert!(run_test(&TESTS["expPowerOf256_2"]));
}
#[test]
fn test_expPowerOf256_20() {
    assert!(run_test(&TESTS["expPowerOf256_20"]));
}
#[test]
fn test_expPowerOf256_21() {
    assert!(run_test(&TESTS["expPowerOf256_21"]));
}
#[test]
fn test_expPowerOf256_22() {
    assert!(run_test(&TESTS["expPowerOf256_22"]));
}
#[test]
fn test_expPowerOf256_23() {
    assert!(run_test(&TESTS["expPowerOf256_23"]));
}
#[test]
fn test_expPowerOf256_24() {
    assert!(run_test(&TESTS["expPowerOf256_24"]));
}
#[test]
fn test_expPowerOf256_25() {
    assert!(run_test(&TESTS["expPowerOf256_25"]));
}
#[test]
fn test_expPowerOf256_26() {
    assert!(run_test(&TESTS["expPowerOf256_26"]));
}
#[test]
fn test_expPowerOf256_27() {
    assert!(run_test(&TESTS["expPowerOf256_27"]));
}
#[test]
fn test_expPowerOf256_28() {
    assert!(run_test(&TESTS["expPowerOf256_28"]));
}
#[test]
fn test_expPowerOf256_29() {
    assert!(run_test(&TESTS["expPowerOf256_29"]));
}
#[test]
fn test_expPowerOf256_3() {
    assert!(run_test(&TESTS["expPowerOf256_3"]));
}
#[test]
fn test_expPowerOf256_30() {
    assert!(run_test(&TESTS["expPowerOf256_30"]));
}
#[test]
fn test_expPowerOf256_31() {
    assert!(run_test(&TESTS["expPowerOf256_31"]));
}
#[test]
fn test_expPowerOf256_32() {
    assert!(run_test(&TESTS["expPowerOf256_32"]));
}
#[test]
fn test_expPowerOf256_33() {
    assert!(run_test(&TESTS["expPowerOf256_33"]));
}
#[test]
fn test_expPowerOf256_4() {
    assert!(run_test(&TESTS["expPowerOf256_4"]));
}
#[test]
fn test_expPowerOf256_5() {
    assert!(run_test(&TESTS["expPowerOf256_5"]));
}
#[test]
fn test_expPowerOf256_6() {
    assert!(run_test(&TESTS["expPowerOf256_6"]));
}
#[test]
fn test_expPowerOf256_7() {
    assert!(run_test(&TESTS["expPowerOf256_7"]));
}
#[test]
fn test_expPowerOf256_8() {
    assert!(run_test(&TESTS["expPowerOf256_8"]));
}
#[test]
fn test_expPowerOf256_9() {
    assert!(run_test(&TESTS["expPowerOf256_9"]));
}
#[test]
fn test_expPowerOf256Of256_0() {
    assert!(run_test(&TESTS["expPowerOf256Of256_0"]));
}
#[test]
fn test_expPowerOf256Of256_1() {
    assert!(run_test(&TESTS["expPowerOf256Of256_1"]));
}
#[test]
fn test_expPowerOf256Of256_10() {
    assert!(run_test(&TESTS["expPowerOf256Of256_10"]));
}
#[test]
fn test_expPowerOf256Of256_11() {
    assert!(run_test(&TESTS["expPowerOf256Of256_11"]));
}
#[test]
fn test_expPowerOf256Of256_12() {
    assert!(run_test(&TESTS["expPowerOf256Of256_12"]));
}
#[test]
fn test_expPowerOf256Of256_13() {
    assert!(run_test(&TESTS["expPowerOf256Of256_13"]));
}
#[test]
fn test_expPowerOf256Of256_14() {
    assert!(run_test(&TESTS["expPowerOf256Of256_14"]));
}
#[test]
fn test_expPowerOf256Of256_15() {
    assert!(run_test(&TESTS["expPowerOf256Of256_15"]));
}
#[test]
fn test_expPowerOf256Of256_16() {
    assert!(run_test(&TESTS["expPowerOf256Of256_16"]));
}
#[test]
fn test_expPowerOf256Of256_17() {
    assert!(run_test(&TESTS["expPowerOf256Of256_17"]));
}
#[test]
fn test_expPowerOf256Of256_18() {
    assert!(run_test(&TESTS["expPowerOf256Of256_18"]));
}
#[test]
fn test_expPowerOf256Of256_19() {
    assert!(run_test(&TESTS["expPowerOf256Of256_19"]));
}
#[test]
fn test_expPowerOf256Of256_2() {
    assert!(run_test(&TESTS["expPowerOf256Of256_2"]));
}
#[test]
fn test_expPowerOf256Of256_20() {
    assert!(run_test(&TESTS["expPowerOf256Of256_20"]));
}
#[test]
fn test_expPowerOf256Of256_21() {
    assert!(run_test(&TESTS["expPowerOf256Of256_21"]));
}
#[test]
fn test_expPowerOf256Of256_22() {
    assert!(run_test(&TESTS["expPowerOf256Of256_22"]));
}
#[test]
fn test_expPowerOf256Of256_23() {
    assert!(run_test(&TESTS["expPowerOf256Of256_23"]));
}
#[test]
fn test_expPowerOf256Of256_24() {
    assert!(run_test(&TESTS["expPowerOf256Of256_24"]));
}
#[test]
fn test_expPowerOf256Of256_25() {
    assert!(run_test(&TESTS["expPowerOf256Of256_25"]));
}
#[test]
fn test_expPowerOf256Of256_26() {
    assert!(run_test(&TESTS["expPowerOf256Of256_26"]));
}
#[test]
fn test_expPowerOf256Of256_27() {
    assert!(run_test(&TESTS["expPowerOf256Of256_27"]));
}
#[test]
fn test_expPowerOf256Of256_28() {
    assert!(run_test(&TESTS["expPowerOf256Of256_28"]));
}
#[test]
fn test_expPowerOf256Of256_29() {
    assert!(run_test(&TESTS["expPowerOf256Of256_29"]));
}
#[test]
fn test_expPowerOf256Of256_3() {
    assert!(run_test(&TESTS["expPowerOf256Of256_3"]));
}
#[test]
fn test_expPowerOf256Of256_30() {
    assert!(run_test(&TESTS["expPowerOf256Of256_30"]));
}
#[test]
fn test_expPowerOf256Of256_31() {
    assert!(run_test(&TESTS["expPowerOf256Of256_31"]));
}
#[test]
fn test_expPowerOf256Of256_32() {
    assert!(run_test(&TESTS["expPowerOf256Of256_32"]));
}
#[test]
fn test_expPowerOf256Of256_33() {
    assert!(run_test(&TESTS["expPowerOf256Of256_33"]));
}
#[test]
fn test_expPowerOf256Of256_4() {
    assert!(run_test(&TESTS["expPowerOf256Of256_4"]));
}
#[test]
fn test_expPowerOf256Of256_5() {
    assert!(run_test(&TESTS["expPowerOf256Of256_5"]));
}
#[test]
fn test_expPowerOf256Of256_6() {
    assert!(run_test(&TESTS["expPowerOf256Of256_6"]));
}
#[test]
fn test_expPowerOf256Of256_7() {
    assert!(run_test(&TESTS["expPowerOf256Of256_7"]));
}
#[test]
fn test_expPowerOf256Of256_8() {
    assert!(run_test(&TESTS["expPowerOf256Of256_8"]));
}
#[test]
fn test_expPowerOf256Of256_9() {
    assert!(run_test(&TESTS["expPowerOf256Of256_9"]));
}
#[test]
fn test_expPowerOf2_64() {
    assert!(run_test(&TESTS["expPowerOf2_64"]));
}
#[test]
fn test_expPowerOf2_8() {
    assert!(run_test(&TESTS["expPowerOf2_8"]));
}
#[test]
fn test_expXY() {
    assert!(run_test(&TESTS["expXY"]));
}
#[test]
fn test_expXY_success() {
    assert!(run_test(&TESTS["expXY_success"]));
}
#[test]
fn test_fibbonacci_unrolled() {
    assert!(run_test(&TESTS["fibbonacci_unrolled"]));
}
#[test]
fn test_mod0() {
    assert!(run_test(&TESTS["mod0"]));
}
#[test]
fn test_mod1() {
    assert!(run_test(&TESTS["mod1"]));
}
#[test]
fn test_mod2() {
    assert!(run_test(&TESTS["mod2"]));
}
#[test]
fn test_mod3() {
    assert!(run_test(&TESTS["mod3"]));
}
#[test]
fn test_mod4() {
    assert!(run_test(&TESTS["mod4"]));
}
#[test]
fn test_modByZero() {
    assert!(run_test(&TESTS["modByZero"]));
}
#[test]
fn test_mul0() {
    assert!(run_test(&TESTS["mul0"]));
}
#[test]
fn test_mul1() {
    assert!(run_test(&TESTS["mul1"]));
}
#[test]
fn test_mul2() {
    assert!(run_test(&TESTS["mul2"]));
}
#[test]
fn test_mul3() {
    assert!(run_test(&TESTS["mul3"]));
}
#[test]
fn test_mul4() {
    assert!(run_test(&TESTS["mul4"]));
}
#[test]
fn test_mul5() {
    assert!(run_test(&TESTS["mul5"]));
}
#[test]
fn test_mul6() {
    assert!(run_test(&TESTS["mul6"]));
}
#[test]
fn test_mul7() {
    assert!(run_test(&TESTS["mul7"]));
}
#[test]
fn test_mulmod0() {
    assert!(run_test(&TESTS["mulmod0"]));
}
#[test]
fn test_mulmod1() {
    assert!(run_test(&TESTS["mulmod1"]));
}
#[test]
fn test_mulmod1_overflow() {
    assert!(run_test(&TESTS["mulmod1_overflow"]));
}
#[test]
fn test_mulmod1_overflow2() {
    assert!(run_test(&TESTS["mulmod1_overflow2"]));
}
#[test]
fn test_mulmod1_overflow3() {
    assert!(run_test(&TESTS["mulmod1_overflow3"]));
}
#[test]
fn test_mulmod1_overflow4() {
    assert!(run_test(&TESTS["mulmod1_overflow4"]));
}
#[test]
fn test_mulmod2() {
    assert!(run_test(&TESTS["mulmod2"]));
}
#[test]
fn test_mulmod2_0() {
    assert!(run_test(&TESTS["mulmod2_0"]));
}
#[test]
fn test_mulmod2_1() {
    assert!(run_test(&TESTS["mulmod2_1"]));
}
#[test]
fn test_mulmod3() {
    assert!(run_test(&TESTS["mulmod3"]));
}
#[test]
fn test_mulmod3_0() {
    assert!(run_test(&TESTS["mulmod3_0"]));
}
#[test]
fn test_mulmod4() {
    assert!(run_test(&TESTS["mulmod4"]));
}
#[test]
fn test_mulmoddivByZero() {
    assert!(run_test(&TESTS["mulmoddivByZero"]));
}
#[test]
fn test_mulmoddivByZero1() {
    assert!(run_test(&TESTS["mulmoddivByZero1"]));
}
#[test]
fn test_mulmoddivByZero2() {
    assert!(run_test(&TESTS["mulmoddivByZero2"]));
}
#[test]
fn test_mulmoddivByZero3() {
    assert!(run_test(&TESTS["mulmoddivByZero3"]));
}
#[test]
fn test_mulUnderFlow() {
    assert!(run_test(&TESTS["mulUnderFlow"]));
}
#[test]
fn test_not1() {
    assert!(run_test(&TESTS["not1"]));
}
#[test]
fn test_sdiv0() {
    assert!(run_test(&TESTS["sdiv0"]));
}
#[test]
fn test_sdiv1() {
    assert!(run_test(&TESTS["sdiv1"]));
}
#[test]
fn test_sdiv2() {
    assert!(run_test(&TESTS["sdiv2"]));
}
#[test]
fn test_sdiv3() {
    assert!(run_test(&TESTS["sdiv3"]));
}
#[test]
fn test_sdiv4() {
    assert!(run_test(&TESTS["sdiv4"]));
}
#[test]
fn test_sdiv5() {
    assert!(run_test(&TESTS["sdiv5"]));
}
#[test]
fn test_sdiv6() {
    assert!(run_test(&TESTS["sdiv6"]));
}
#[test]
fn test_sdiv7() {
    assert!(run_test(&TESTS["sdiv7"]));
}
#[test]
fn test_sdiv8() {
    assert!(run_test(&TESTS["sdiv8"]));
}
#[test]
fn test_sdiv9() {
    assert!(run_test(&TESTS["sdiv9"]));
}
#[test]
fn test_sdivByZero0() {
    assert!(run_test(&TESTS["sdivByZero0"]));
}
#[test]
fn test_sdivByZero1() {
    assert!(run_test(&TESTS["sdivByZero1"]));
}
#[test]
fn test_sdivByZero2() {
    assert!(run_test(&TESTS["sdivByZero2"]));
}
#[test]
fn test_sdiv_dejavu() {
    assert!(run_test(&TESTS["sdiv_dejavu"]));
}
#[test]
fn test_sdiv_i256min() {
    assert!(run_test(&TESTS["sdiv_i256min"]));
}
#[test]
fn test_sdiv_i256min2() {
    assert!(run_test(&TESTS["sdiv_i256min2"]));
}
#[test]
fn test_sdiv_i256min3() {
    assert!(run_test(&TESTS["sdiv_i256min3"]));
}
#[test]
fn test_signextend_00() {
    assert!(run_test(&TESTS["signextend_00"]));
}
#[test]
fn test_signextend_0_BigByte() {
    assert!(run_test(&TESTS["signextend_0_BigByte"]));
}
#[test]
fn test_signextend_AlmostBiggestByte() {
    assert!(run_test(&TESTS["signextend_AlmostBiggestByte"]));
}
#[test]
fn test_signextend_BigByte_0() {
    assert!(run_test(&TESTS["signextend_BigByte_0"]));
}
#[test]
fn test_signextend_BigByteBigByte() {
    assert!(run_test(&TESTS["signextend_BigByteBigByte"]));
}
#[test]
fn test_signextend_bigBytePlus1() {
    assert!(run_test(&TESTS["signextend_bigBytePlus1"]));
}
#[test]
fn test_signextend_BigBytePlus1_2() {
    assert!(run_test(&TESTS["signextend_BigBytePlus1_2"]));
}
#[test]
fn test_signextend_BitIsNotSet() {
    assert!(run_test(&TESTS["signextend_BitIsNotSet"]));
}
#[test]
fn test_signextend_BitIsNotSetInHigherByte() {
    assert!(run_test(&TESTS["signextend_BitIsNotSetInHigherByte"]));
}
#[test]
fn test_signextend_bitIsSet() {
    assert!(run_test(&TESTS["signextend_bitIsSet"]));
}
#[test]
fn test_signextend_BitIsSetInHigherByte() {
    assert!(run_test(&TESTS["signextend_BitIsSetInHigherByte"]));
}
#[test]
fn test_signextendInvalidByteNumber() {
    assert!(run_test(&TESTS["signextendInvalidByteNumber"]));
}
#[test]
fn test_signextend_Overflow_dj42() {
    assert!(run_test(&TESTS["signextend_Overflow_dj42"]));
}
#[test]
fn test_smod0() {
    assert!(run_test(&TESTS["smod0"]));
}
#[test]
fn test_smod1() {
    assert!(run_test(&TESTS["smod1"]));
}
#[test]
fn test_smod2() {
    assert!(run_test(&TESTS["smod2"]));
}
#[test]
fn test_smod3() {
    assert!(run_test(&TESTS["smod3"]));
}
#[test]
fn test_smod4() {
    assert!(run_test(&TESTS["smod4"]));
}
#[test]
fn test_smod5() {
    assert!(run_test(&TESTS["smod5"]));
}
#[test]
fn test_smod6() {
    assert!(run_test(&TESTS["smod6"]));
}
#[test]
fn test_smod7() {
    assert!(run_test(&TESTS["smod7"]));
}
#[test]
fn test_smod8_byZero() {
    assert!(run_test(&TESTS["smod8_byZero"]));
}
#[test]
fn test_smod_i256min1() {
    assert!(run_test(&TESTS["smod_i256min1"]));
}
#[test]
fn test_smod_i256min2() {
    assert!(run_test(&TESTS["smod_i256min2"]));
}
#[test]
fn test_stop() {
    assert!(run_test(&TESTS["stop"]));
}
#[test]
fn test_sub0() {
    assert!(run_test(&TESTS["sub0"]));
}
#[test]
fn test_sub1() {
    assert!(run_test(&TESTS["sub1"]));
}
#[test]
fn test_sub2() {
    assert!(run_test(&TESTS["sub2"]));
}
#[test]
fn test_sub3() {
    assert!(run_test(&TESTS["sub3"]));
}
#[test]
fn test_sub4() {
    assert!(run_test(&TESTS["sub4"]));
}
