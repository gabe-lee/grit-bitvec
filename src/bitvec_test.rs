#![allow(unused_assignments)]
use crate::*;
use std::slice::from_raw_parts as slice_from_raw;

static _111: usize = 0b_111;
static _010: usize = 0b_010;
static _101: usize = 0b_101;
static _000: usize = 0b_000;
static _100: usize = 0b_100;

static _1111: usize = 0b_1111;
static _0110: usize = 0b_0110;
static _1010: usize = 0b_1010;
static _0000: usize = 0b_0000;
static _1000: usize = 0b_1000;

static _11111: usize = 0b_11111;
static _01110: usize = 0b_01110;
static _10101: usize = 0b_10101;
static _00000: usize = 0b_00000;
static _10000: usize = 0b_10000;

static _FAIL: usize = 0b_11100000;

macro_rules! assert_bvec_state {
    ($MARK:literal, $PROTO:ident, $VEC:ident, $LEN:expr, $CAP:expr, $DATA:expr) => {
        assert_eq!($LEN, $VEC.0.len, "\n{} FAIL: incorrect length:\n\tEXP: {}\n\tGOT: {}\n", $MARK, $LEN, $VEC.0.len);
        let bitwise_cap = BitProto::calc_bitwise_count_from_block_count($PROTO, $VEC.0.true_cap);
        assert_eq!($CAP, bitwise_cap, "\n{} FAIL: incorrect capacity:\n\tEXP: {}\n\tGOT: {}\n", $MARK, $CAP, bitwise_cap);
        let got_slice = unsafe{slice_from_raw($VEC.0.ptr.as_ptr(), BitProto::calc_block_count_from_bitwise_count($PROTO, $VEC.0.len))};
        let mut exp_string = String::new();
        let mut got_string = String::new();
        let mut fail = false;
        let mut exp_done = false;
        let mut got_done = false;
        let last_proxy = BitProto::idx_proxy($PROTO, $VEC.0.len);
        let ignore_mask = !BitUtil::smear_left(1 << last_proxy.first_offset);
        if got_slice.len() != $DATA.len() {
            fail = true;
        }
        let mut idx: usize = 0;
        while !exp_done && !got_done {
            let mut exp_val: usize = 0;
            let mut got_val: usize = 0;
            if idx < $DATA.len() {
                exp_val = $DATA[idx];
                exp_string.push_str(&format!("{:064b} ", exp_val));
            } else {
                exp_done = true;
            }
            if idx < got_slice.len() {
                got_val = got_slice[idx];
                if idx == last_proxy.real_idx {
                    got_val &= ignore_mask;
                }
                got_string.push_str(&format!("{:064b} ", got_val));
            } else {
                got_done = true;
            }
            if !fail {
                fail = !(exp_val == got_val);
            }
            idx += 1;
        }
        if fail {
            panic!("\n{} FAIL: incorrect data:\n\tEXP DATA LEN: {}\n\tGOT_DATA_LEN: {}\n\tEXP DATA: {}\n\tGOT DATA: {}\n", $MARK, $DATA.len(), got_slice.len(), exp_string, got_string);
        }
    };
}

macro_rules! assert_val_result {
    ($MARK:literal, $EXP:expr, $GOT:expr) => {
        match $GOT {
            Ok(val) if val != $EXP => panic!("\n{} FAIL: incorrect val:\n\tEXP = {:08b}\n\tGOT = {:08b}", $MARK, $EXP, val),
            Err(err) => panic!("\n{} FAIL: error val:\n\tEXP = {:08b}\n\tGOT ERR = {}", $MARK, $EXP, err),
            _ => {}
        }
    };
}

macro_rules! assert_error {
    ($MARK:literal, $RESULT:expr) => {
        match $RESULT {
            Err(_) => {},
            Ok(_) => panic!("\n{} FAIL: expected Err(_), got Ok(_)", $MARK),
        }
    };
}

macro_rules! force_write {
    ($VEC:ident, $LEN:expr, $DATA:expr) => {
        let len = $DATA.len();
        let mut idx = 0;
        let block_ptr = $VEC.0.ptr.as_ptr();
        while idx < len {
            unsafe{ptr::write(block_ptr.add(idx), $DATA[idx])};
            idx += 1;
        }
        $VEC.0.len = $LEN;
    };
}
#[test]
fn push_4_bits() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::new();
    let proto = CProtoBitVec::<4>::PROTO;
    assert_bvec_state!("1", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.push(_1111)?;
    assert_bvec_state!("2", proto, bitvec, 1, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    bitvec.push(_0110)?;
    assert_bvec_state!("3", proto, bitvec, 2, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_1111__usize]);
    bitvec.push(_1111)?;
    assert_bvec_state!("4", proto, bitvec, 3, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_0110_1111__usize]);
    bitvec.push(_0110)?;
    assert_bvec_state!("5", proto, bitvec, 4, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_1111_0110_1111__usize]);
    bitvec.push(_1111)?;
    bitvec.push(_0110)?;
    bitvec.push(_1111)?;
    bitvec.push(_0110)?;
    bitvec.push(_1111)?;
    bitvec.push(_0110)?;
    bitvec.push(_1111)?;
    bitvec.push(_0110)?;
    bitvec.push(_1111)?;
    bitvec.push(_0110)?;
    bitvec.push(_1111)?;
    bitvec.push(_0110)?;
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0
    assert_bvec_state!("6", proto, bitvec, 16, 16, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize]);
    bitvec.push(_1111)?;
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16
    assert_bvec_state!("7", proto, bitvec, 17, 32, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    bitvec.push(_1010)?;
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16     
    assert_bvec_state!("8", proto, bitvec, 32, 32, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    bitvec.push(_1000)?;
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("9", proto, bitvec, 33, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.push(_1000)?;
    //                                                                          15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("10", proto, bitvec, 34, proto.MAX_CAPACITY, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_1000__usize]);
    bitvec.0.len = proto.MAX_CAPACITY;
    assert_error!("11", bitvec.push(_1000));
    Ok(())
}

#[test]
fn push_3_bits() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<3>::new();
    let proto = CProtoBitVec::<3>::PROTO;
    assert_bvec_state!("1", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.push(_111)?;
    assert_bvec_state!("2", proto, bitvec, 1, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_111__usize]);
    bitvec.push(_010)?;
    assert_bvec_state!("3", proto, bitvec, 2, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_010_111__usize]);
    bitvec.push(_111)?;
    assert_bvec_state!("4", proto, bitvec, 3, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_111_010_111__usize]);
    bitvec.push(_010)?;
    assert_bvec_state!("5", proto, bitvec, 4, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_010_111_010_111__usize]);
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    bitvec.push(_010)?;
    bitvec.push(_111)?;
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
    assert_bvec_state!("6", proto, bitvec, 21, 21, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize]);
    bitvec.push(_010)?;
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21
    assert_bvec_state!("7", proto, bitvec, 22, 42, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_01__usize]);
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    bitvec.push(_101)?;
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21
    assert_bvec_state!("8", proto, bitvec, 42, 42, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize]);
    bitvec.push(_100)?;
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("9", proto, bitvec, 43, 64, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_1__usize]);
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.push(_100)?;
    //                                                                       21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("10", proto, bitvec, 44, proto.MAX_CAPACITY, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_100_1__usize]);
    bitvec.0.len = proto.MAX_CAPACITY;
    assert_error!("11", bitvec.push(_100));
    Ok(())
}

#[test]
fn pop_4_bits() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::with_capacity(33);
    let proto = CProtoBitVec::<4>::PROTO;
    //                               15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(bitvec, 33,  [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("1", proto, bitvec, 33, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    assert_val_result!("2", _1000, bitvec.pop());
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("3", proto, bitvec, 32, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    assert_val_result!("4", _1010, bitvec.pop());
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("5", proto, bitvec, 31, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    assert_val_result!("6", _1010, bitvec.pop());
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("7", proto, bitvec, 30, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    assert_val_result!("8", _1111, bitvec.pop());
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("9", proto, bitvec, 16, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize]);
    assert_val_result!("10", _0110, bitvec.pop());
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("11", proto, bitvec, 15, 48, [0b__0000_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize]);
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    assert_val_result!("12", _1111, bitvec.pop());
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("13", proto, bitvec, 0, 48, [0usize; 0]);
    assert_error!("14", bitvec.pop());
    Ok(())

}

#[test]
fn pop_3_bits() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<3>::with_capacity(44);
    let proto = CProtoBitVec::<3>::PROTO;
    //                            21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    force_write!(bitvec, 44, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_100_1__usize]);
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("1", proto, bitvec, 44, 64, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_100_1__usize]);
    assert_val_result!("2", _100, bitvec.pop());
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("3", proto, bitvec, 43, 64, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_1__usize]);
    assert_val_result!("4", _100, bitvec.pop());
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("5", proto, bitvec, 42, 64, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize]);
    assert_val_result!("6", _101, bitvec.pop());
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("7", proto, bitvec, 41, 64, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_000_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_01__usize]);
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    assert_val_result!("8", _010, bitvec.pop());
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("9", proto, bitvec, 21, 64, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize]);
    assert_val_result!("10", _111, bitvec.pop());
    //                                              21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("11", proto, bitvec, 20, 64, [0b__0_000_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize]);
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    bitvec.pop()?;
    assert_val_result!("12", _111, bitvec.pop());
    //                                             21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("13", proto, bitvec, 0, 64, [0usize; 0]);
    assert_error!("14", bitvec.pop());
    Ok(())
}

#[test]
fn insert_4_bits() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::new();
    let proto = CProtoBitVec::<4>::PROTO;
    assert_bvec_state!("1", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.insert(0, _1111)?;
    assert_bvec_state!("2", proto, bitvec, 1, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    bitvec.insert(1, _0110)?;
    assert_bvec_state!("3", proto, bitvec, 2, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_1111__usize]);
    bitvec.insert(2, _1111)?;
    assert_bvec_state!("4", proto, bitvec, 3, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_0110_1111__usize]);
    bitvec.insert(3, _0110)?;
    assert_bvec_state!("5", proto, bitvec, 4, 16, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_1111_0110_1111__usize]);
    bitvec.insert(4, _1111)?;
    bitvec.insert(5, _0110)?;
    bitvec.insert(6, _1111)?;
    bitvec.insert(7, _0110)?;
    bitvec.insert(8, _1111)?;
    bitvec.insert(9, _0110)?;
    bitvec.insert(10, _1111)?;
    bitvec.insert(11, _0110)?;
    bitvec.insert(12, _1111)?;
    bitvec.insert(13, _0110)?;
    bitvec.insert(14, _1111)?;
    bitvec.insert(15, _0110)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0
    assert_bvec_state!("6", proto, bitvec, 16, 16, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize]);
    bitvec.insert(16, _1111)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16
    assert_bvec_state!("7", proto, bitvec, 17, 32, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    bitvec.insert(0, _1010)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16
    assert_bvec_state!("8", proto, bitvec, 18, 32, [0b__1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_1010__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_0110__usize]);
    bitvec.insert(0, _1010)?;
    bitvec.insert(1, _1010)?;
    bitvec.insert(1, _1010)?;
    bitvec.insert(2, _1010)?;
    bitvec.insert(2, _1010)?;
    bitvec.insert(3, _1010)?;
    bitvec.insert(3, _1010)?;
    bitvec.insert(4, _1010)?;
    bitvec.insert(4, _1010)?;
    bitvec.insert(5, _1010)?;
    bitvec.insert(5, _1010)?;
    bitvec.insert(6,_1010)?;
    bitvec.insert(6, _1010)?;
    bitvec.insert(7, _1010)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16     
    assert_bvec_state!("9", proto, bitvec, 32, 32, [0b__1111_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize, 0b__1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110__usize]);
    bitvec.insert(24, _1000)?;
    //                                                     15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("10", proto, bitvec, 33, 48, [0b__1111_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize, 0b__0110_1111_0110_1111_0110_1111_0110_1000_1111_0110_1111_0110_1111_0110_1111_0110__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.insert(32, _1000)?;
    //                                                                          15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("11", proto, bitvec, 34, proto.MAX_CAPACITY, [0b__1111_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize, 0b__0110_1111_0110_1111_0110_1111_0110_1000_1111_0110_1111_0110_1111_0110_1111_0110__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_1000__usize]);
    assert_error!("12", bitvec.insert(99, _1000));
    bitvec.0.len = proto.MAX_CAPACITY;
    assert_error!("13", bitvec.insert(0, _1000));
    Ok(())
}

#[test]
fn insert_3_bits() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<3>::new();
    let proto = CProtoBitVec::<3>::PROTO;
    assert_bvec_state!("1", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.insert(0, _111)?;
    assert_bvec_state!("2", proto, bitvec, 1, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_111__usize]);
    bitvec.insert(1, _010)?;
    assert_bvec_state!("3", proto, bitvec, 2, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_010_111__usize]);
    bitvec.insert(2, _111)?;
    assert_bvec_state!("4", proto, bitvec, 3, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_111_010_111__usize]);
    bitvec.insert(3, _010)?;
    assert_bvec_state!("5", proto, bitvec, 4, 21, [0b__0_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_010_111_010_111__usize]);
    bitvec.insert(4, _111)?;
    bitvec.insert(5, _010)?;
    bitvec.insert(6, _111)?;
    bitvec.insert(7, _010)?;
    bitvec.insert(8, _111)?;
    bitvec.insert(9, _010)?;
    bitvec.insert(10, _111)?;
    bitvec.insert(11, _010)?;
    bitvec.insert(12, _111)?;
    bitvec.insert(13, _010)?;
    bitvec.insert(14, _111)?;
    bitvec.insert(15, _010)?;
    bitvec.insert(16, _111)?;
    bitvec.insert(17, _010)?;
    bitvec.insert(18, _111)?;
    bitvec.insert(19, _010)?;
    bitvec.insert(20, _111)?;
    //                                                 21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
    assert_bvec_state!("6", proto, bitvec, 21, 21, [0b__0_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize]);
    bitvec.insert(21, _111)?;
    //                                                 21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21
    assert_bvec_state!("7", proto, bitvec, 22, 42, [0b__1_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111__usize, 0b__00_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_11__usize]);
    bitvec.insert(0,_101)?;
    //                                                 21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21
    assert_bvec_state!("8", proto, bitvec, 23, 42, [0b__1_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_101__usize, 0b__00_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_111_11__usize]);
    bitvec.insert(0,_101)?;
    bitvec.insert(1, _101)?;
    bitvec.insert(1, _101)?;
    bitvec.insert(2, _101)?;
    bitvec.insert(2, _101)?;
    bitvec.insert(3, _101)?;
    bitvec.insert(3, _101)?;
    bitvec.insert(4, _101)?;
    bitvec.insert(4, _101)?;
    bitvec.insert(5, _101)?;
    bitvec.insert(5, _101)?;
    bitvec.insert(6, _101)?;
    bitvec.insert(6, _101)?;
    bitvec.insert(7, _101)?;
    bitvec.insert(7, _101)?;
    bitvec.insert(8, _101)?;
    bitvec.insert(8, _101)?;
    bitvec.insert(9, _101)?;
    bitvec.insert(9, _101)?;
    //                                                 21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21
    assert_bvec_state!("9", proto, bitvec, 42, 42, [0b__0_111_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101__usize, 0b__00_111_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_01__usize]);
    bitvec.insert(0, _100)?;
    //                                                 21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("10", proto, bitvec, 43, 64, [0b__1_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_100__usize, 0b__11_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_11__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_1__usize]);
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.insert(42, _100)?;
    //                                                                  21  20  19  18  17  16  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0             42  41  40  39  38  37  36  35  34  33  32  31  30  29  28  27  26  25  24  23  22 21              63  62  61  60  59  58  57  56  55  54  53  52  51  50  49  48  47  46  45  44  43 42
    assert_bvec_state!("11", proto, bitvec, 44, proto.MAX_CAPACITY, [0b__1_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_101_100__usize, 0b__00_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_111_010_11__usize, 0b__000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_111_1__usize]);
    assert_error!("12", bitvec.insert(99, _100));
    bitvec.0.len = proto.MAX_CAPACITY;
    assert_error!("12", bitvec.insert(0, _100));
    Ok(())
}

