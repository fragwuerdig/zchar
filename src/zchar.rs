use std::fmt;

const A0: [char;26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z'
];
const A1: [char;26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z'
];
const A2: [char;25] = [
    '^', '0', '1', '2', '3', '4', '5', '6',
    '7', '8', '9', '.', ',', '!', '?', '_',
    '#', '\'', '"', '/', '\\', '-', ':', '(', ')'
];

// default unicode translation table
const S: [char;69] = [
    'ä', 'ö', 'ü', 'Ä', 'Ö', 'Ü', 'ß', '»',
    '«', 'ë', 'ï', 'ÿ', 'Ë', 'Ï', 'á', 'é',
    'í', 'ó', 'ú', 'ý', 'Á', 'É', 'Í', 'Ó',
    'Ú', 'Ý', 'à', 'è', 'ì', 'ò', 'ù', 'À',
    'È', 'Ì', 'Ò', 'Ù', 'â', 'ê', 'î', 'ô',
    'û', 'Â', 'Ê', 'Î', 'Ô', 'Û', 'å', 'Å',
    'ø', 'Ø', 'ã', 'ñ', 'õ', 'Ã', 'Ñ', 'Õ',
    'æ', 'Æ', 'ç', 'Ç', 'þ', 'ð', 'Þ', 'Ð',
    '£', 'œ', 'Œ', '¡', '¿'
];

pub fn zpack(input: Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    for chunk in input.chunks(3) {
        let mut hb: u8 = 0;
        let mut lb: u8 = 0;
        let chunk0 = chunk[0] & 0x1f;
        let chunk1 = match chunk.len() {
            2 => chunk[1] & 0x1f,
            3 => chunk[1] & 0x1f,
            _ => 0,
        };
        let chunk2 = match chunk.len() {
            3 => chunk[2] & 0x1f,
            _ => 0,
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
    let N = v.len() - 2;
    let mut second_last = v[N].clone();
    second_last += 0x80;
    v[N] = second_last;
    return v;
}

// map unicode chars to zchars
fn utf8_to_zchar(
    c: char,
    a0: Vec<char>,
    a1: Vec<char>,
    a2: Vec<char>,
    utt: Vec<char>
) -> Option<Vec<u8>> {
    
    let mut v: Vec<u8> = Vec::new();
    if c == '\0' || c == ' ' {
        v.push(0 as u8);
        return Some(v);
    }
    
    if c == '\n' {
        v.push(13 as u8);
        return Some(v)
    }
    
    // map ASCII chars to ZCHAR
    if c >= 0x20.into() && c <= 126.into() {
        match a0.into_iter().position(|x| c == x) {
            Some(index) => {
                v.push(index as u8 + 6);
            },
            None => {},
        }
        match a1.into_iter().position(|x| c == x) {
            Some(index) => {
                v.push(0x04);
                v.push(index as u8 + 6);
            },
            None => {},
        }
        match a2.into_iter().position(|x| c == x) {
            Some(index) => {
                v.push(0x05);
                v.push(index as u8 + 7)
            },
            None => {},
        }
        return Some(v);
    }
    
    // map special chars to ZCHAR
    match utt.into_iter().position(|x| c == x) {
        Some(index) => {
            v.push(0x05);
            v.push(0x06);
            let comb = index as u8 + 155;
            let low = comb & 0x1f;
            let high = (comb & 0xe0) >> 5;
            v.push(high);
            v.push(low);
            return Some(v)
        },
        None => {return None;},
    };

}

fn string_to_zchar(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for x in s.chars() {
        let chars = utf8_to_zchar(
            x,
            A0.to_vec(),
            A1.to_vec(),
            A2.to_vec(),
            S.to_vec()
        );
        match chars {
            Some(c) => {v.append(&mut c.clone())},
            None => {}
        }
    }
    let ret = zpack(v);
    return ret;
}

fn format_vec(bytes: Vec<u8>) -> String {
    let hex_string = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");
    return hex_string;
}

mod tests {
    use crate::zpack;

    #[test]
    fn test_zpack() {
        let vec1: Vec<u8> = vec![0xa5, 0x5a, 0xa5, 0x5a, 0xa5, 0x5a];
        let vec2: Vec<u8> = vec![0x5a, 0xa5, 0x5a, 0xa5, 0x5a]; 
        let vec3: Vec<u8> = vec![0xa5, 0x5a, 0xa5, 0x5a];
        let vec4: Vec<u8> = vec![0x5a, 0xa5, 0x5a];
        let vec5: Vec<u8> = vec![0xa5, 0x5a];
        let vec6: Vec<u8> = vec![0x5a];
        let vec7: Vec<u8> = vec![];
        
        let exp1: Vec<u8> = vec![0x17, 0x45, 0xe8, 0xba];
        let exp2: Vec<u8> = vec![0x68, 0xba, 0x97, 0x40];
        let exp3: Vec<u8> = vec![0x17, 0x45, 0xe8, 0x00];
        let exp4: Vec<u8> = vec![0xe8, 0xba];
        let exp5: Vec<u8> = vec![0x97, 0x40];
        let exp6: Vec<u8> = vec![0xe8, 0x00];
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

}
