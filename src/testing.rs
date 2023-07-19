mod bit_width_3;
mod bit_width_4;

#[macro_export]
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

#[macro_export]
macro_rules! assert_val_result {
    ($MARK:literal, $EXP:expr, $GOT:expr) => {
        match $GOT {
            Ok(val) if val != $EXP => panic!("\n{} FAIL: incorrect val:\n\tEXP = {:08b}\n\tGOT = {:08b}", $MARK, $EXP, val),
            Err(err) => panic!("\n{} FAIL: error val:\n\tEXP = {:08b}\n\tGOT ERR = {}", $MARK, $EXP, err),
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! assert_error {
    ($MARK:literal, $RESULT:expr) => {
        match $RESULT {
            Err(_) => {},
            Ok(_) => panic!("\n{} FAIL: expected Err(_), got Ok(_)", $MARK),
        }
    };
}

#[macro_export]
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

