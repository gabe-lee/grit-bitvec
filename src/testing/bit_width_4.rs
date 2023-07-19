#![allow(unused_assignments)]
use crate::*;
use std::slice::from_raw_parts as slice_from_raw;

static _1111: usize = 0b_1111;
static _0110: usize = 0b_0110;
static _1010: usize = 0b_1010;
static _0000: usize = 0b_0000;
static _1000: usize = 0b_1000;

static _FAIL: usize = 0b_11100000;



#[test]
fn push() -> Result<(), String> {
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
    assert_bvec_state!("7", proto, bitvec, 17, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
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
    assert_bvec_state!("8", proto, bitvec, 32, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    bitvec.push(_1000)?;
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("9", proto, bitvec, 33, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    let old_true_cap = bitvec.0.true_cap;
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.push(_1000)?;
    //                                                                          15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("10", proto, bitvec, 34, proto.MAX_CAPACITY, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_1000__usize]);
    bitvec.0.len = proto.MAX_CAPACITY;
    assert_error!("11", bitvec.push(_1000));
    bitvec.0.true_cap = old_true_cap;
    Ok(())
}

#[test]
fn pop() -> Result<(), String> {
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
fn insert() -> Result<(), String> {
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
    assert_bvec_state!("7", proto, bitvec, 17, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    bitvec.insert(0, _1010)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16
    assert_bvec_state!("8", proto, bitvec, 18, 48, [0b__1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_1010__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_0110__usize]);
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
    assert_bvec_state!("9", proto, bitvec, 32, 48, [0b__1111_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize, 0b__1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110__usize]);
    bitvec.insert(24, _1000)?;
    //                                                     15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("10", proto, bitvec, 33, 48, [0b__1111_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize, 0b__0110_1111_0110_1111_0110_1111_0110_1000_1111_0110_1111_0110_1111_0110_1111_0110__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    let old_true_cap = bitvec.0.true_cap;
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.insert(32, _1000)?;
    //                                                                          15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("11", proto, bitvec, 34, proto.MAX_CAPACITY, [0b__1111_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize, 0b__0110_1111_0110_1111_0110_1111_0110_1000_1111_0110_1111_0110_1111_0110_1111_0110__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_1000__usize]);
    assert_error!("12", bitvec.insert(99, _1000));
    bitvec.0.len = proto.MAX_CAPACITY;
    assert_error!("13", bitvec.insert(0, _1000));
    bitvec.0.true_cap = old_true_cap;
    Ok(())
}

#[test]
fn remove() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::with_capacity(33);
    let proto = CProtoBitVec::<4>::PROTO;
    //                               15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(bitvec, 33,  [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("1", proto, bitvec, 33, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    assert_val_result!("2", _1000, bitvec.remove(32));
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("3", proto, bitvec, 32, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    assert_val_result!("4", _1010, bitvec.remove(31));
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("5", proto, bitvec, 31, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    assert_val_result!("6", _1010, bitvec.remove(30));
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("7", proto, bitvec, 30, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize]);
    bitvec.remove(29)?;
    bitvec.remove(28)?;
    bitvec.remove(27)?;
    bitvec.remove(26)?;
    bitvec.remove(25)?;
    bitvec.remove(24)?;
    bitvec.remove(23)?;
    bitvec.remove(22)?;
    bitvec.remove(21)?;
    bitvec.remove(20)?;
    bitvec.remove(19)?;
    bitvec.remove(18)?;
    bitvec.remove(17)?;
    //                                                      15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("7.1", proto, bitvec, 17, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    assert_val_result!("8", _1111, bitvec.remove(0));
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("9", proto, bitvec, 16, 48, [0b__1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110__usize]);
    assert_val_result!("10", _0110, bitvec.remove(0));
    //                                                     15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("11", proto, bitvec, 15, 48, [0b__0000_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize]);
    assert_val_result!("12", _0110, bitvec.remove(7));
    assert_val_result!("13", _1111, bitvec.remove(7));
    assert_val_result!("14", _0110, bitvec.remove(7));
    //                                                     15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("15", proto, bitvec, 12, 48, [0b__0000_0000_0000_0000_1111_0110_1111_0110_1111_1111_0110_1111_0110_1111_0110_1111__usize]);
    assert_val_result!("16", _1111, bitvec.remove(6));
    assert_val_result!("17", _1111, bitvec.remove(6));
    assert_val_result!("18", _0110, bitvec.remove(8));
    //                                                     15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("19", proto, bitvec, 9, 48, [0b__0000_0000_0000_0000_0000_0000_0000_1111_1111_0110_0110_1111_0110_1111_0110_1111__usize]);
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    bitvec.remove(0)?;
    assert_error!("20", bitvec.remove(1));
    assert_val_result!("21", _1111, bitvec.remove(0));
    //                                                15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("22", proto, bitvec, 0, 48, [0usize; 0]);
    assert_error!("23", bitvec.remove(0));
    Ok(())
}

#[test]
fn append_bitvec() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::with_capacity(58);
    let mut to_append_1 = CProtoBitVec::<4>::with_capacity(33);
    let mut to_append_2 = CProtoBitVec::<4>::with_capacity(25);
    let mut to_append_3 = CProtoBitVec::<4>::with_capacity(17);
    let empty_append = CProtoBitVec::<4>::new();
    let mut to_append_1_extra_a = CProtoBitVec::<4>::with_capacity(1);
    let mut to_append_1_extra_b = CProtoBitVec::<4>::with_capacity(1);
    to_append_1_extra_a.push(_1111)?;
    to_append_1_extra_b.push(_1111)?;
    let proto = CProtoBitVec::<4>::PROTO;
    //                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(to_append_1, 33,  [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    //                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(to_append_2, 25,  [0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize, 0b__0000_0000_0000_0000_0000_0000_0000_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize]);
    //                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(to_append_3, 17,  [0b__1111_1110_1101_1100_1011_1010_1001_1000_0111_0110_0101_0100_0011_0010_0001_0000__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    assert_bvec_state!("1", proto, bitvec, 0, 64, [0usize; 0]);
    bitvec.append_bitvec(to_append_1)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("2", proto, bitvec, 33, 64, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    bitvec.append_bitvec(to_append_2)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("3", proto, bitvec, 58, 64, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1000__usize, 0b__0000_0000_0000_0000_0000_0000_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize]);
    bitvec.append_bitvec(to_append_3)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("4", proto, bitvec, 75, 112, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1000__usize, 0b__0101_0100_0011_0010_0001_0000_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize, 0b__0000_0000_0000_0000_0000_1111_1111_1110_1101_1100_1011_1010_1001_1000_0111_0110__usize]);
    let old_true_cap = bitvec.0.true_cap;
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.append_bitvec(to_append_1_extra_a)?;
    bitvec.0.len = proto.MAX_CAPACITY;
    bitvec.append_bitvec(empty_append)?;
    assert_error!("5", bitvec.append_bitvec(to_append_1_extra_b));
    bitvec.0.true_cap = old_true_cap;
    Ok(())
}

#[test]
fn insert_bitvec() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::with_capacity(60);
    let mut to_insert_1 = CProtoBitVec::<4>::with_capacity(33);
    let mut to_insert_2 = CProtoBitVec::<4>::with_capacity(25);
    let mut to_insert_3 = CProtoBitVec::<4>::with_capacity(17);
    let empty_insert = CProtoBitVec::<4>::new();
    let mut to_insert_1_extra_a = CProtoBitVec::<4>::with_capacity(1);
    let mut to_insert_1_extra_b = CProtoBitVec::<4>::with_capacity(1);
    let to_insert_1_extra_c = CProtoBitVec::<4>::with_capacity(1);
    to_insert_1_extra_a.push(_1111)?;
    to_insert_1_extra_b.push(_1111)?;
    let proto = CProtoBitVec::<4>::PROTO;
    //                              15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(bitvec, 2,  [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1001_1001__usize]);
    //                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(to_insert_1, 33,  [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    //                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(to_insert_2, 25,  [0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize, 0b__0000_0000_0000_0000_0000_0000_0000_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize]);
    //                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    force_write!(to_insert_3, 17,  [0b__1111_1110_1101_1100_1011_1010_1001_1000_0111_0110_0101_0100_0011_0010_0001_0000__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111__usize]);
    assert_bvec_state!("1", proto, bitvec, 2, 64, [0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1001_1001__usize]);
    assert_bvec_state!("1.1", proto, to_insert_1, 33, 48, [0b__0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000__usize]);
    bitvec.insert_bitvec(1, to_insert_1)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("2", proto, bitvec, 35, 64, [0b__1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_1001__usize, 0b__1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1111_0110__usize, 0b__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1001_1000_1010__usize]);
    bitvec.insert_bitvec(1, to_insert_2)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("3", proto, bitvec, 60, 64, [0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1001__usize, 0b__0110_1111_0110_1111_0110_1111_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize, 0b__1010_1010_1010_1010_1010_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_0000_1001_1000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize]);
    bitvec.insert_bitvec(24, to_insert_3)?;
    //                                                    15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("4", proto, bitvec, 77, 112, [0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1001__usize, 0b__0111_0110_0101_0100_0011_0010_0001_0000_1100_1100_1100_1100_1100_1100_1100_1100__usize, 0b__1111_0110_1111_0110_1111_1100_1100_1111_1111_1110_1101_1100_1011_1010_1001_1000__usize, 0b__1010_1010_1010_1010_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110__usize, 0b__0000_0000_0000_1001_1000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize]);
    let old_true_cap = bitvec.0.true_cap;
    bitvec.0.true_cap = BitProto::calc_block_count_from_bitwise_count(proto, proto.MAX_CAPACITY);
    bitvec.insert_bitvec(0, to_insert_1_extra_a)?;
    //                                                            15   14   13   12   11   10    9    8    7    6    5    4    3    2    1    0               31   30   29   28   27   26   25   24   23   22   21   20   19   18   17   16               47   46   45   44   43   42   41   40   39   38   37   36   35   34   33   32
    assert_bvec_state!("5", proto, bitvec, 78, proto.MAX_CAPACITY, [0b__1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1100_1001_1111__usize, 0b__0110_0101_0100_0011_0010_0001_0000_1100_1100_1100_1100_1100_1100_1100_1100_1100__usize, 0b__0110_1111_0110_1111_1100_1100_1111_1111_1110_1101_1100_1011_1010_1001_1000_0111__usize, 0b__1010_1010_1010_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111_0110_1111__usize, 0b__0000_0000_1001_1000_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010__usize]);
    assert_error!("6", bitvec.insert_bitvec(1000, to_insert_1_extra_c));
    bitvec.0.len = proto.MAX_CAPACITY;
    bitvec.insert_bitvec(0, empty_insert)?;
    assert_error!("7", bitvec.insert_bitvec(0, to_insert_1_extra_b));
    bitvec.0.true_cap = old_true_cap;
    Ok(())
}

#[test]
fn grow() -> Result<(), String> {
    let mut bitvec = CProtoBitVec::<4>::new();
    let proto = CProtoBitVec::<4>::PROTO;
    assert_bvec_state!("1", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(0)?;
    assert_bvec_state!("2", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(1)?;
    assert_bvec_state!("3", proto, bitvec, 0, 16, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(16)?;
    assert_bvec_state!("4", proto, bitvec, 0, 16, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(17)?;
    assert_bvec_state!("5", proto, bitvec, 0, 32, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(32)?;
    assert_bvec_state!("6", proto, bitvec, 0, 32, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(33)?;
    assert_bvec_state!("7", proto, bitvec, 0, 48, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(81)?;
    assert_bvec_state!("8", proto, bitvec, 0, 96, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(96)?;
    assert_bvec_state!("9", proto, bitvec, 0, 96, [0usize; 0]);
    bitvec.grow_exact_for_total_elements_if_needed(97)?;
    assert_bvec_state!("10", proto, bitvec, 0, 112, [0usize; 0]);

    bitvec = CProtoBitVec::<4>::new();
    assert_bvec_state!("11", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(0)?;
    assert_bvec_state!("12", proto, bitvec, 0, 0, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(1)?;
    assert_bvec_state!("13", proto, bitvec, 0, 16, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(16)?;
    assert_bvec_state!("14", proto, bitvec, 0, 16, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(17)?;
    assert_bvec_state!("15", proto, bitvec, 0, 48, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(48)?;
    assert_bvec_state!("16", proto, bitvec, 0, 48, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(49)?;
    assert_bvec_state!("17", proto, bitvec, 0, 96, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(97)?;
    assert_bvec_state!("18", proto, bitvec, 0, 160, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(160)?;
    assert_bvec_state!("19", proto, bitvec, 0, 160, [0usize; 0]);
    bitvec.grow_for_total_elements_if_needed(161)?;
    assert_bvec_state!("20", proto, bitvec, 0, 256, [0usize; 0]);
    Ok(())
}