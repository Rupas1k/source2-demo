use crate::field_path::FieldPath;
use crate::reader::Reader;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub(crate) enum FieldOp {
    PlusOne,
    PlusTwo,
    PlusThree,
    PlusFour,
    PlusN,
    PushOneLeftDeltaZeroRightZero,
    PushOneLeftDeltaZeroRightNonZero,
    PushOneLeftDeltaOneRightZero,
    PushOneLeftDeltaOneRightNonZero,
    PushOneLeftDeltaNRightZero,
    PushOneLeftDeltaNRightNonZero,
    PushOneLeftDeltaNRightNonZeroPack6Bits,
    PushOneLeftDeltaNRightNonZeroPack8Bits,
    PushTwoLeftDeltaZero,
    PushTwoPack5LeftDeltaZero,
    PushThreeLeftDeltaZero,
    PushThreePack5LeftDeltaZero,
    PushTwoLeftDeltaOne,
    PushTwoPack5LeftDeltaOne,
    PushThreeLeftDeltaOne,
    PushThreePack5LeftDeltaOne,
    PushTwoLeftDeltaN,
    PushTwoPack5LeftDeltaN,
    PushThreeLeftDeltaN,
    PushThreePack5LeftDeltaN,
    PushN,
    PushNAndNonTopological,
    PopOnePlusOne,
    PopOnePlusN,
    PopAllButOnePlusOne,
    PopAllButOnePlusN,
    PopAllButOnePlusNPack3Bits,
    PopAllButOnePlusNPack6Bits,
    PopNPlusOne,
    PopNPlusN,
    PopNAndNonTopographical,
    NonTopoComplex,
    NonTopoPenultimatePlusOne,
    NonTopoComplexPack4Bits,
    FieldPathEncodeFinish,
}

impl FieldOp {
    pub(crate) fn execute(&self, r: &mut Reader, fp: &mut FieldPath) {
        match &self {
            FieldOp::PlusOne => {
                fp.path[fp.last] += 1;
                // fp.inc_cur(1)
            }
            FieldOp::PlusTwo => {
                fp.path[fp.last] += 2;
                // fp.inc_cur(2)
            }
            FieldOp::PlusThree => {
                fp.path[fp.last] += 3;
                // fp.inc_cur(3)
            }
            FieldOp::PlusFour => {
                fp.path[fp.last] += 4;
                // fp.inc_cur(4)
            }
            FieldOp::PlusN => {
                fp.path[fp.last] += r.read_ubit_var_fieldpath() + 5;
                // fp.inc_cur((r.read_ubit_var_fieldpath() + 5) as i64)
            }
            FieldOp::PushOneLeftDeltaZeroRightZero => {
                fp.last += 1;
                fp.path[fp.last] = 0;
                // fp.down()
            }
            FieldOp::PushOneLeftDeltaZeroRightNonZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fieldpath();
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushOneLeftDeltaOneRightZero => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = 0;
                // fp.inc_cur(1);
                // fp.down();
            }
            FieldOp::PushOneLeftDeltaOneRightNonZero => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fieldpath();
                // fp.inc_cur(1);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushOneLeftDeltaNRightZero => {
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] = 0;
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
            }
            FieldOp::PushOneLeftDeltaNRightNonZero => {
                fp.path[fp.last] += r.read_ubit_var_fieldpath() + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fieldpath() + 1;
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64 + 1);
            }
            FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits => {
                fp.path[fp.last] += r.read_bits(3) as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(3) as i32 + 1;
                // fp.inc_cur(r.read_bits(3) as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_bits(3) as i64 + 1);
            }
            FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits => {
                fp.path[fp.last] += r.read_bits(4) as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(4) as i32 + 1;
                // fp.inc_cur(r.read_bits(4) as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_bits(4) as i64 + 1);
            }
            FieldOp::PushTwoLeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushTwoPack5LeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
            }
            FieldOp::PushThreeLeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushThreePack5LeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
            }
            FieldOp::PushTwoLeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                // fp.inc_cur(1);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushTwoPack5LeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                // fp.inc_cur(1);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
            }
            FieldOp::PushThreeLeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                // fp.inc_cur(1);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushThreePack5LeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                // fp.inc_cur(1);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
            }
            FieldOp::PushTwoLeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                // fp.inc_cur(r.read_ubit_var() as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushTwoPack5LeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                // fp.inc_cur(r.read_ubit_var() as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
            }
            FieldOp::PushThreeLeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                // fp.inc_cur(r.read_ubit_var() as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // fp.down();
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
            }
            FieldOp::PushThreePack5LeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                // fp.inc_cur(r.read_ubit_var() as i64 + 2);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
                // fp.down();
                // fp.inc_cur(r.read_bits(5) as i64);
            }
            FieldOp::PushN => {
                let n = r.read_ubit_var() as i32;
                fp.path[fp.last] += r.read_ubit_var() as i32;
                for _ in 0..n {
                    fp.last += 1;
                    fp.path[fp.last] += r.read_ubit_var_fieldpath();
                }
                // let n = r.read_ubit_var() as i32;
                // fp.inc_cur(r.read_ubit_var() as i64);
                // for _ in 0..n {
                //     fp.down();
                //     fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // }
            }
            FieldOp::PushNAndNonTopological => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_var_i32() + 1;
                    }
                }
                let count = r.read_ubit_var() as usize;
                for _ in 0..count {
                    fp.last += 1;
                    fp.path[fp.last] = r.read_ubit_var_fieldpath();
                }
                // for i in 0..=fp.last() {
                //     if r.read_bool() {
                //         fp.inc(i, r.read_var_i32() as i64 + 1);
                //     }
                // }
                // let count = r.read_ubit_var() as usize;
                // for _ in 0..count {
                //     fp.down();
                //     fp.inc_cur(r.read_ubit_var_fieldpath() as i64);
                // }
            }
            FieldOp::PopOnePlusOne => {
                fp.pop(1);
                fp.path[fp.last] += 1;
                // fp.up(1);
                // fp.inc_cur(1);
            }

            FieldOp::PopOnePlusN => {
                fp.pop(1);
                fp.path[fp.last] += r.read_ubit_var_fieldpath() + 1;
                // fp.up(1);
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64 + 1);
            }
            FieldOp::PopAllButOnePlusOne => {
                fp.pop(fp.last);
                fp.path[0] += 1;
                // fp.up(fp.last());
                // fp.inc_cur(1);
            }
            FieldOp::PopAllButOnePlusN => {
                fp.pop(fp.last);
                fp.path[0] += r.read_ubit_var_fieldpath() + 1;
                // fp.up(fp.last());
                // fp.inc_cur(r.read_ubit_var_fieldpath() as i64 + 1);
            }
            FieldOp::PopAllButOnePlusNPack3Bits => {
                fp.pop(fp.last);
                fp.path[0] += r.read_bits(3) as i32 + 1;
                // fp.up(fp.last());
                // fp.inc_cur(r.read_bits(3) as i64 + 1);
            }
            FieldOp::PopAllButOnePlusNPack6Bits => {
                fp.pop(fp.last);
                fp.path[0] += r.read_bits(6) as i32 + 1;
                // fp.up(fp.last());
                // fp.inc_cur(r.read_bits(6) as i64 + 1);
            }
            FieldOp::PopNPlusOne => {
                fp.pop(r.read_ubit_var_fieldpath() as usize);
                fp.path[fp.last] += 1;
                // fp.up(r.read_ubit_var_fieldpath() as usize);
                // fp.inc_cur(1);
            }
            FieldOp::PopNPlusN => {
                fp.pop(r.read_ubit_var_fieldpath() as usize);
                fp.path[fp.last] += r.read_var_i32();
                // fp.up(r.read_ubit_var_fieldpath() as usize);
                // fp.inc_cur(r.read_var_i32() as i64);
            }
            FieldOp::PopNAndNonTopographical => {
                fp.pop(r.read_ubit_var_fieldpath() as usize);
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_var_i32();
                    }
                }
                // fp.up(r.read_ubit_var_fieldpath() as usize);
                // for i in 0..=fp.last() {
                //     if r.read_bool() {
                //         fp.inc(i, r.read_var_i32() as i64);
                //     }
                // }
            }
            FieldOp::NonTopoComplex => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_var_i32();
                    }
                }
                // for i in 0..=fp.last() {
                //     if r.read_bool() {
                //         fp.inc(i, r.read_var_i32() as i64);
                //     }
                // }
            }
            FieldOp::NonTopoPenultimatePlusOne => {
                fp.path[fp.last - 1] += 1;
                // fp.inc(fp.last() - 1, 1);
            }
            FieldOp::NonTopoComplexPack4Bits => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_bits(4) as i32 - 7;
                    }
                }
                // for i in 0..=fp.last() {
                //     if r.read_bool() {
                //         fp.inc(i, r.read_bits(4) as i64 - 7);
                //     }
                // }
            }
            FieldOp::FieldPathEncodeFinish => {
                // fp.done = true;
            }
        }
    }
    pub fn weight(&self) -> u32 {
        match self {
            FieldOp::PlusOne => 36271,
            FieldOp::PlusTwo => 10334,
            FieldOp::PlusThree => 1375,
            FieldOp::PlusFour => 646,
            FieldOp::PlusN => 4128,
            FieldOp::PushOneLeftDeltaZeroRightZero => 35,
            FieldOp::PushOneLeftDeltaZeroRightNonZero => 3,
            FieldOp::PushOneLeftDeltaOneRightZero => 521,
            FieldOp::PushOneLeftDeltaOneRightNonZero => 2942,
            FieldOp::PushOneLeftDeltaNRightZero => 560,
            FieldOp::PushOneLeftDeltaNRightNonZero => 471,
            FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits => 10530,
            FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits => 251,
            FieldOp::PushTwoLeftDeltaZero => 0,
            FieldOp::PushTwoPack5LeftDeltaZero => 0,
            FieldOp::PushThreeLeftDeltaZero => 0,
            FieldOp::PushThreePack5LeftDeltaZero => 0,
            FieldOp::PushTwoLeftDeltaOne => 0,
            FieldOp::PushTwoPack5LeftDeltaOne => 0,
            FieldOp::PushThreeLeftDeltaOne => 0,
            FieldOp::PushThreePack5LeftDeltaOne => 0,
            FieldOp::PushTwoLeftDeltaN => 0,
            FieldOp::PushTwoPack5LeftDeltaN => 0,
            FieldOp::PushThreeLeftDeltaN => 0,
            FieldOp::PushThreePack5LeftDeltaN => 0,
            FieldOp::PushN => 0,
            FieldOp::PushNAndNonTopological => 310,
            FieldOp::PopOnePlusOne => 2,
            FieldOp::PopOnePlusN => 0,
            FieldOp::PopAllButOnePlusOne => 1837,
            FieldOp::PopAllButOnePlusN => 149,
            FieldOp::PopAllButOnePlusNPack3Bits => 300,
            FieldOp::PopAllButOnePlusNPack6Bits => 634,
            FieldOp::PopNPlusOne => 0,
            FieldOp::PopNPlusN => 0,
            FieldOp::PopNAndNonTopographical => 1,
            FieldOp::NonTopoComplex => 76,
            FieldOp::NonTopoPenultimatePlusOne => 271,
            FieldOp::NonTopoComplexPack4Bits => 99,
            FieldOp::FieldPathEncodeFinish => 25474,
        }
    }

    pub fn from_position(position: i32) -> Self {
        match position {
            0 => FieldOp::PlusOne,
            1 => FieldOp::PlusTwo,
            2 => FieldOp::PlusThree,
            3 => FieldOp::PlusFour,
            4 => FieldOp::PlusN,
            5 => FieldOp::PushOneLeftDeltaZeroRightZero,
            6 => FieldOp::PushOneLeftDeltaZeroRightNonZero,
            7 => FieldOp::PushOneLeftDeltaOneRightZero,
            8 => FieldOp::PushOneLeftDeltaOneRightNonZero,
            9 => FieldOp::PushOneLeftDeltaNRightZero,
            10 => FieldOp::PushOneLeftDeltaNRightNonZero,
            11 => FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits,
            12 => FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits,
            13 => FieldOp::PushTwoLeftDeltaZero,
            14 => FieldOp::PushTwoPack5LeftDeltaZero,
            15 => FieldOp::PushThreeLeftDeltaZero,
            16 => FieldOp::PushThreePack5LeftDeltaZero,
            17 => FieldOp::PushTwoLeftDeltaOne,
            18 => FieldOp::PushTwoPack5LeftDeltaOne,
            19 => FieldOp::PushThreeLeftDeltaOne,
            20 => FieldOp::PushThreePack5LeftDeltaOne,
            21 => FieldOp::PushTwoLeftDeltaN,
            22 => FieldOp::PushTwoPack5LeftDeltaN,
            23 => FieldOp::PushThreeLeftDeltaN,
            24 => FieldOp::PushThreePack5LeftDeltaN,
            25 => FieldOp::PushN,
            26 => FieldOp::PushNAndNonTopological,
            27 => FieldOp::PopOnePlusOne,
            28 => FieldOp::PopOnePlusN,
            29 => FieldOp::PopAllButOnePlusOne,
            30 => FieldOp::PopAllButOnePlusN,
            31 => FieldOp::PopAllButOnePlusNPack3Bits,
            32 => FieldOp::PopAllButOnePlusNPack6Bits,
            33 => FieldOp::PopNPlusOne,
            34 => FieldOp::PopNPlusN,
            35 => FieldOp::PopNAndNonTopographical,
            36 => FieldOp::NonTopoComplex,
            37 => FieldOp::NonTopoPenultimatePlusOne,
            38 => FieldOp::NonTopoComplexPack4Bits,
            39 => FieldOp::FieldPathEncodeFinish,
            _ => unreachable!(),
        }
    }
}
