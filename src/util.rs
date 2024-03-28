use crate::zscii::{A0, A1, A2, ZSCIITAB};


/// `zpack` takes a series of 5bit values (stored
/// as `Vec<u8>`) chunks and packs clusters of
/// 3 values into one 16bit value according to
/// z-machine specification. The result is
/// represented as `Vec<u8>`. The routine will
/// discard bits 4,6 and 7 of each input byte.
pub fn zpack(input: Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    for chunk in input.chunks(3) {
        let mut hb: u8 = 0;
        let mut lb: u8 = 0;
        let chunk0 = chunk[0] & 0x1f;
        let chunk1 = match chunk.len() {
            2 => chunk[1] & 0x1f,
            3 => chunk[1] & 0x1f,
            _ => 5,
        };
        let chunk2 = match chunk.len() {
            3 => chunk[2] & 0x1f,
            _ => 5,
        };
        hb |= chunk0 << 2;
        hb |= chunk1 >> 3;
        lb |= chunk1 << 5;
        lb |= chunk2;
        v.push(hb);
        v.push(lb);
    }

    if v.len() == 0 {
        return v;
    }

    // mark last
    let n = v.len() - 2;
    let mut second_last = v[n].clone();
    second_last += 0x80;
    v[n] = second_last;
    return v;
}

/// `zunpack` takes a series of 16bit values (stored
/// as `Vec<u8>`) dissects them into clusters of
/// 3 5bit values according to z-machine specification.
/// The resulting series of 5bit values are represented
/// as `Vec<u8>`.
pub fn zunpack(input: Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    
    for chunk in input.chunks(2) {
        let chunk0 = match chunk.len() {
            1 => chunk[0],
            2 => chunk[0],
            _ => 0,
        };
        let chunk1 = match chunk.len() {
            2 => chunk[1],
            _ => 0,
        };
        let zchar0 = (chunk0 & 0x7c) >> 2;
        let zchar1 = ((chunk0 & 0x03) << 3) | ((chunk1 & 0xe0) >> 5);
        let zchar2 = chunk1 & 0x1f;
        v.push(zchar0);
        v.push(zchar1);
        v.push(zchar2);
    }
    
    return v;
}


/// map unicode chars to a corresponding sequence of zchars.
/// Not that zchars are 5bit unsigned integers. However, the
/// single zchar is represented as 8bit unsigned. The mapping
/// is done according to input alphabets `a0`, `a1` and `a2`
/// as well as to the zscii table `zscii` (specified
/// by the z-machine standard) .
pub fn zmap(
    c: char,
    a0: Vec<char>,
    a1: Vec<char>,
    a2: Vec<char>,
    zscii: Vec<Option<char>>
) -> Option<Vec<u8>> {
    
    let mut v: Vec<u8> = Vec::new();

    // check if is space
    if c == ' ' {
        v.push(0 as u8);
        return Some(v);
    }
    
    // check if is A0 char
    if a0.contains(&c) {
        let index = a0.iter().position(|&x| x == c).unwrap();
        v.push(index as u8 + 6);
        return Some(v);
    }

    // check if is A1 char
    if a1.contains(&c) {
        let index = a1.iter().position(|&x| x == c).unwrap();
        v.push(0x04);
        v.push(index as u8 + 6);
        return Some(v);
    }

    // check if is A2 char
    if a2.contains(&c) {
        let index = a2.iter().position(|&x| x == c).unwrap();
        v.push(0x05);
        v.push(index as u8 + 7);
        return Some(v);
    }

    // if the char is not representable by A0, A1 or A2
    // then map according to ZSCIITAB
    match zscii.into_iter().position(|x| Some(c) == x) {
        Some(index) => {
            v.push(0x05);
            v.push(0x06);
            let hb = ((index & 0xe0) >> 5) as u8;
            let lb = (index & 0x1f) as u8;
            v.push(hb);
            v.push(lb);
            return Some(v)
        },
        None => {return None;},
    };

}

pub fn zunmap(
    zchars: Vec<u8>,
    a0: Vec<char>,
    a1: Vec<char>,
    a2: Vec<char>,
    zscii: Vec<Option<char>>
) -> Option<(char, u8)> {
    if zchars.len() < 1 {
        return None;
    } 

    // null character
    if zchars[0] == 0 {
        return Some((' ', 1))
    }

    // newline character in version 1
    // abbreviation character since version 3
    else if zchars[0] == 1 {
        // TODO
        return None;
    }
    
    // shift lock chars unrecognized
    // since version  3 recognized
    // as abbreviation characters
    else if zchars[0] == 2 || zchars[0] == 3 {
        // TODO
        return None;
    }
    
    // shift1
    else if zchars[0] == 4 {
        if zchars.len() < 2 {
            return None;
        }

        if zchars[1] > 5 && zchars[1] <= 31 {
            let index = zchars[1] - 6;
            return Some((a1[index as usize], 2));
        }
        
        return None;
    }
    
    // shift2
    else if zchars[0] == 5 {
        if zchars.len() < 2 {
            return None;
        }
        
        // map according to zscii
        if zchars[1] == 6 {
            if zchars.len() < 4 {
                return None;
            }
            let hb = ((zchars[2] & 0x1f) as u16) << 5;
            let lb = (zchars[3] & 0x1f) as u16;
            let index = hb | lb;
            match zscii[index as usize] {
                Some(c) => {return Some((c, 4))},
                None => {return None;}
            }
        }

        // map according to A2
        else if zchars[1] > 6 && zchars[1] <= 31 {
            let index = zchars[1] - 7;
            return Some((a2[index as usize], 2))
        }
        
        // unrecognized sequence
        else {
            return None;
        }
    }

    // map according to A0
    else if zchars[0] > 5 && zchars[0] <= 31 {
        let index = zchars[0] - 6;
        return Some((a0[index as usize],1))
    }

    // zchars > 31 not recognized
    else {
        return None;
    }

}

pub fn string_to_zstring(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for x in s.chars() {
        let chars = zmap(
            x, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
        );
        match chars {
            Some(c) => {v.append(&mut c.clone())},
            None => {}
        }
    }
    let ret = zpack(v);
    return ret;
}

pub fn zstring_to_string(z: Vec<u8>) -> String {
    let v = zunpack(z);
    let mut s = String::new();
    let mut i = 0;
    while i < v.len() {
        let zchars = match v.len() - i {
            0 => {break;},
            1 => {vec![v[i]]},
            2 => {vec![v[i], v[i+1]]},
            3 => {vec![v[i], v[i+1], v[i+2]]},
            _ => {vec![v[i], v[i+1], v[i+2], v[i+3]]},
        };
        let c = zunmap(
            zchars, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
        );
        match c {
            Some(x) => {s.push(x.0); i += x.1 as usize;},
            None => { i+= 1; }
        }
    }
    return s;
}

mod tests {
    use std::string;

    use super::{zpack, zunpack};
    use crate::zscii::{A0, A1, A2, S, ZSCIITAB};

    #[test]
    fn test_zpack() {
        let vec1: Vec<u8> = vec![0xa5, 0x5a, 0xa5, 0x5a, 0xa5, 0x5a];
        let vec2: Vec<u8> = vec![0x5a, 0xa5, 0x5a, 0xa5, 0x5a]; 
        let vec3: Vec<u8> = vec![0xa5, 0x5a, 0xa5, 0x5a];
        let vec4: Vec<u8> = vec![0x5a, 0xa5, 0x5a];
        let vec5: Vec<u8> = vec![0xa5, 0x5a];
        let vec6: Vec<u8> = vec![0x5a];
        let vec7: Vec<u8> = vec![];
        
        // zpacked strings are padded out with 5s
        let exp1: Vec<u8> = vec![0x17, 0x45, 0xe8, 0xba];
        let exp2: Vec<u8> = vec![0x68, 0xba, 0x97, 0x45];
        let exp3: Vec<u8> = vec![0x17, 0x45, 0xe8, 0xa5];
        let exp4: Vec<u8> = vec![0xe8, 0xba];
        let exp5: Vec<u8> = vec![0x97, 0x45];
        let exp6: Vec<u8> = vec![0xe8, 0xa5];
        let exp7: Vec<u8> = vec![];

        let packed1 = zpack(vec1);
        let packed2 = zpack(vec2);
        let packed3 = zpack(vec3);
        let packed4 = zpack(vec4);
        let packed5 = zpack(vec5);
        let packed6 = zpack(vec6);
        let packed7 = zpack(vec7);
 
        assert_eq!(packed1, exp1);
        assert_eq!(packed2, exp2);
        assert_eq!(packed3, exp3);
        assert_eq!(packed4, exp4);
        assert_eq!(packed5, exp5);
        assert_eq!(packed6, exp6);
        assert_eq!(packed7, exp7);
    }

    #[test]
    fn test_zunpack() {
        let vec1: Vec<u8> = vec![0x17, 0x45, 0xe8, 0xba];
        let vec2: Vec<u8> = vec![0x68, 0xba, 0x97, 0x40];
        let vec3: Vec<u8> = vec![0x17, 0x45, 0xe8, 0x00];
        let vec4: Vec<u8> = vec![0x17, 0x45, 0xe8];
        let vec5: Vec<u8> = vec![0xe8, 0xba];
        let vec6: Vec<u8> = vec![0x97, 0x40];
        let vec7: Vec<u8> = vec![0xe8, 0x00];
        let vec8: Vec<u8> = vec![0xe8];
        let vec9: Vec<u8> = vec![];

        let exp1: Vec<u8> = vec![0x05, 0x1a, 0x05, 0x1a, 0x05, 0x1a];
        let exp2: Vec<u8> = vec![0x1a, 0x05, 0x1a, 0x05, 0x1a, 0x00]; 
        let exp3: Vec<u8> = vec![0x05, 0x1a, 0x05, 0x1a, 0x00, 0x00];
        let exp4: Vec<u8> = vec![0x05, 0x1a, 0x05, 0x1a, 0x00, 0x00];
        let exp5: Vec<u8> = vec![0x1a, 0x05, 0x1a];
        let exp6: Vec<u8> = vec![0x05, 0x1a, 0x00];
        let exp7: Vec<u8> = vec![0x1a, 0x00, 0x00];
        let exp8: Vec<u8> = vec![0x1a, 0x00, 0x00];
        let exp9: Vec<u8> = vec![];

        let unpacked1 = zunpack(vec1);
        let unpacked2 = zunpack(vec2);
        let unpacked3 = zunpack(vec3);
        let unpacked4 = zunpack(vec4);
        let unpacked5 = zunpack(vec5);
        let unpacked6 = zunpack(vec6);
        let unpacked7 = zunpack(vec7);
        let unpacked8 = zunpack(vec8);
        let unpacked9 = zunpack(vec9);
 
        assert_eq!(unpacked1, exp1);
        assert_eq!(unpacked2, exp2);
        assert_eq!(unpacked3, exp3);
        assert_eq!(unpacked4, exp4);
        assert_eq!(unpacked5, exp5);
        assert_eq!(unpacked6, exp6);
        assert_eq!(unpacked7, exp7);
        assert_eq!(unpacked8, exp8);
        assert_eq!(unpacked9, exp9);
    }

    #[test]
    fn test_zmap_maps_a0_correctly() {
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            let zchar = super::zmap(
                c,
                A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(zchar, Some(vec![c as u8 - 0x61 + 6]));
        }
    }

    #[test]
    fn test_zmap_maps_a1_correctly() {
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            let zchar = super::zmap(
                c,
                A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(zchar, Some(vec![0x04, c as u8 - 0x41 + 6]));
        }
    }

    #[test]
    fn test_zmap_maps_a2_correctly() {
        for c in "^".chars() {
            let zchar = super::zmap(
                c, A0.to_vec(),  A1.to_vec(),  A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(zchar, Some(vec![0x05, 7u8]));
        }
        
        for c in "0123456789".chars() {
            let zchar = super::zmap(
                c, A0.to_vec(),A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(zchar, Some(vec![0x05, c as u8 - 0x2f  + 7]));
        }
        
        let mut index = 18u8;
        for c in ".,!?_#\'\"/\\-:()".chars() {
            let zchar = super::zmap(
                c, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(zchar, Some(vec![0x05, index]));
            index += 1;
        }
    }

    #[test]
    fn test_zmap_maps_zscii_correctly() {
        for c in S.iter() {
            let zchar = super::zmap(
                *c, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            let index = ZSCIITAB.iter().position(|&x| x == Some(*c)).unwrap();
            assert_eq!(zchar, Some(vec![0x05, 0x06, (index as u8 & 0xe0) >> 5, index as u8 & 0x1f]));
        }
    }

    #[test]
    fn test_zmap_maps_space_correctly() {
        let zchar = super::zmap(
            ' ', A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
        );
        assert_eq!(zchar, Some(vec![0x00]));
    }

    #[test]
    fn test_zunmap_maps_a0_correctly() {
        for seq in 6..31 {
            let chars: Vec<u8> = vec![seq];
            let c = super::zunmap(
                chars, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(c, Some(((seq - 6 + 0x61) as char, 1u8)));
        }
    }

    #[test]
    fn test_zunmap_maps_a1_correctly() {
        for seq in 6..31 {
            let chars: Vec<u8> = vec![0x04, seq];
            let c = super::zunmap(
                chars, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(c, Some(((seq - 6 + 0x41) as char, 2u8)));
        }
    }

    #[test]
    fn test_zunmap_maps_a2_correctly() {
        for seq in 7..31 {
            let chars: Vec<u8> = vec![0x05, seq];
            let c = super::zunmap(
                chars, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
            );
            assert_eq!(c, Some((A2[seq as usize - 7], 2u8)));
        }
    }

    #[test]
    fn test_zunmap_maps_zscii_correctly() {
        for c in ZSCIITAB.iter() {
            match c {
                Some(x) => {
                    let index = ZSCIITAB.iter().position(|&y| y == Some(*x)).unwrap();
                    let chars: Vec<u8> = vec![0x05, 0x06, (index as u8 & 0xe0) >> 5, index as u8 & 0x1f];
                    let c = super::zunmap(
                        chars, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
                    );
                    assert_eq!(c, Some((*x,4)));
                },
                None => {}
            }
        }
    }

    #[test]
    fn test_zunmap_maps_space_correctly() {
        let chars: Vec<u8> = vec![0x00];
        let c = super::zunmap(
            chars, A0.to_vec(), A1.to_vec(), A2.to_vec(), ZSCIITAB.to_vec()
        );
        assert_eq!(c, Some((' ', 1)));
    }

    #[test]
    fn test_zstring_conversion_is_consistent() {
        let s1 = "»Grüß Gott!\nWie geht's?\tDes is a Gaudi!«".to_string();
        let s2 = "".to_string();
        let s3 = "      ".to_string();

        let zstring = super::string_to_zstring(s1.clone());
        let string = super::zstring_to_string(zstring);
        assert_eq!(string, s1);

        let zstring = super::string_to_zstring(s2.clone());
        let string = super::zstring_to_string(zstring);
        assert_eq!(string, s2);

        let zstring = super::string_to_zstring(s3.clone());
        let string = super::zstring_to_string(zstring);
        assert_eq!(string, s3);
    }

}
