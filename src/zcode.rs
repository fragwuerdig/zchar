#[derive(Debug)]
pub struct ZInstruction {
    opcode: u8,
    operand_count: u8,
    operands: [Option<ZOperand>; 8],
}

impl PartialEq for ZInstruction {
    fn eq(&self, other: &Self) -> bool {
        let mut is_eq = self.opcode == other.opcode && self.operand_count == other.operand_count;
        for i in 0..7 {
            is_eq = is_eq && self.operands[i] == other.operands[i];
        }
        is_eq
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ZOperand {
    Large {
        value: [u8; 2],
    },
    Small {
        value: u8,
    },
    Variable {
        value: u8,
    },
}

impl PartialEq for ZOperand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Large{ value: s }, Self::Large{ value: o} ) => {
                s == o
            },
            (Self::Small{ value: s }, Self::Small{ value: o} ) => {
                s == o
            },
            (Self::Variable{ value: s }, Self::Variable{ value: o} ) => {
                s == o
            }
            _ => {false}
        }
    }
}

fn determine_operand_size(optypes: [u8; 2]) -> (u8, u8) {
    let mut opcount = 0;
    let mut memsize = 0;
    let mut is_end = false;
    for i in [0, 1, 2, 3] {
        let optype = (optypes[0] & (0b11000000 >> (i * 2))) >> (6 - i * 2);
        match optype {
            0b00 => { memsize += 2; opcount += 1;},
            0b01..=0b10 => { memsize += 1; opcount += 1;},
            _ => {is_end = true; break;},
        };
    }
    if is_end {
        return (opcount, memsize);
    }
    for i in [0, 1, 2, 3] {
        let optype = (optypes[1] & (0b11000000 >> (i * 2))) >> (6 - i * 2);
        match optype {
            0b00 => { memsize += 2; opcount += 1;},
            0b01..=0b10 => { memsize += 1; opcount += 1;},
            _ => {break;},
        };
    }
    (opcount, memsize)
}

fn determine_var_operands(optypes: [u8; 2], ops: Vec<u8>) -> [Option<ZOperand>; 8] {
    let mut operands: [Option<ZOperand>;8] = [None; 8];
    let mut opp = 0;
    let mut is_end = false;
    for i in [0, 1, 2, 3] {
        let optype = (optypes[0] & (0b11000000 >> (i * 2))) >> (6 - i * 2);
        let operand = match optype {
            0b00 => {
                let ret = Some(ZOperand::Large{value: [ops[opp], ops[opp+1]]});
                opp += 2;
                ret
            },
            0b01 => {
                let ret = Some(ZOperand::Small{value: ops[opp]});
                opp += 1;
                ret
            },
            0b10 => {
                let ret = Some(ZOperand::Variable{value: ops[opp]});
                opp += 1;
                ret
            },
            _ => {is_end = true; break;},
        };
        operands[i] = operand;
    }
    if is_end {
        return operands;
    }
    for i in [0, 1, 2, 3] {
        let optype = (optypes[1] & (0b11000000 >> (i * 2))) >> (6 - i * 2);
        let operand = match optype {
            0b00 => {
                let ret = Some(ZOperand::Large{value: [ops[opp], ops[opp+1]]});
                opp +=2;
                ret
            },
            0b01 => {
                let ret = Some(ZOperand::Small{value: ops[opp]});
                opp +=1;
                ret
            },
            0b10 => {
                let ret = Some(ZOperand::Variable{value: ops[opp]});
                opp +=1;
                ret
            },
            _ => {break;},
        };
        operands[i+4] = operand;
    }
    operands
}

pub fn decode_instruction(input: Vec<u8>) -> Option<ZInstruction> {
    if input.len() < 1 {
        return None;
    }
    let opcode = input[0];
    match opcode {
        // long form instruction - 2OP - small constant - small constant
        0x00..=0x1f => {
            if input.len() < 3 {
                return None;
            }
            Some(ZInstruction {
                opcode: opcode & 0b00011111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Small{ value: input[1] }),
                    Some(ZOperand::Small{ value: input[2] }),
                    None, None, None, None, None, None
                ],
            })
        },

        // long form instruction - 2OP - small constant - variable
        0x20..=0x3f => { 
            if input.len() < 3 {
                return None;
            }
            Some(ZInstruction{
                opcode: opcode & 0b00011111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Small{ value: input[1] }),
                    Some(ZOperand::Variable{ value: input[2]}),
                    None, None, None, None, None, None
                ],
            
            })
        },

        // long form instruction - 2OP - variable - small constant
        0x40..=0x5f => { 
            if input.len() < 3 {
                return None;
            }
            Some(ZInstruction{
                opcode: opcode & 0b00011111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Variable { value: input[1] }),
                    Some(ZOperand::Small{ value: input[2]}),
                    None, None, None, None, None, None
                ],
            
            })
        },
        
        // long form instruction - 2OP - variable - variable
        0x60..=0x7f => { 
            if input.len() < 3 {
                return None;
            }
            Some(ZInstruction{
                opcode: opcode & 0b00011111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Variable { value: input[1] }),
                    Some(ZOperand::Variable { value: input[2] }),
                    None, None, None, None, None, None
                ],
            
            })
        },

        // short form instruction - 1OP - large constant
        0x80..=0x8f => { 
            if input.len() < 3 {
                return None;
            }
            Some(ZInstruction{
                opcode: opcode & 0b00001111,
                operand_count: 1,
                operands: [
                    Some(ZOperand::Large {
                        value: [input[1], input[2]],
                    }),
                    None, None, None, None, None, None, None
                ],
            
            })
        },
        
        // short form instruction - 1OP - small constant
        0x90..=0x9f => { 
            if input.len() < 2 {
                return None;
            }
            Some(ZInstruction{
                opcode: opcode & 0b00001111,
                operand_count: 1,
                operands: [
                    Some(ZOperand::Small {
                        value: input[1],
                    }),
                    None, None, None, None, None, None, None
                ],
            
            })
        },
        
        // short form instruction - 1OP - variable
        0xa0..=0xaf => { 
            if input.len() < 2 {
                return None;
            }
            Some(ZInstruction{
                opcode: opcode & 0b00001111,
                operand_count: 1,
                operands: [
                    Some(ZOperand::Variable {
                        value: input[1],
                    }),
                    None, None, None, None, None, None, None
                ],
            
            })
        },
        
        // short form instruction - 0OP
        0xb0..=0xbd | 0xbf => { 
            Some(ZInstruction{
                opcode: opcode & 0b00001111,
                operand_count: 0,
                operands: [
                    None, None, None, None, None, None, None, None
                ],
            
            })
        },
        
        // extended form intruction
        // TODO
        0xbe => {
            if input.len() < 2 {
                return None;
            }
            Some(ZInstruction{
                opcode: input[1],
                operand_count: 0,
                operands: [
                    None, None, None, None, None, None, None, None
                ],
            })
        },
        
        // variable form instruction - 2OP - var operand types
        0xc0..=0xdf => {
            if input.len() < 2 {
                return None;
            }
            let opcode = input[0] & 0b11111;
            let optype0 = (input[1] & 0b11110000) | 0b00001111;
            let optypes = [optype0 , 0b11111111];
            let (opcount, memsize) = determine_operand_size(optypes);
            if opcount < 2 {
                return None;
            }
            if input.len() < (memsize + 2) as usize {
                return None;
            }
            let operands = determine_var_operands(
                optypes,
                input.clone().split_off(2),
            );
            Some(ZInstruction{
                opcode: opcode,
                operand_count: opcount,
                operands: operands
            })
        },
        
        // variable form instruction - VAR
        0xe0..=0xff => { 
            let opcode = input[0] & 0b11111;
            let mut offset = 0u8;
            let optypes = match opcode {
                12 | 26 => {
                    if input.len() < 3 {
                        return None
                    }
                    offset += 2;
                    [input[1], input[2]]
                }
                _ => {
                    if input.len() < 2 {
                        return None
                    }
                    offset += 1;
                    [input[1], 0b11111111]
                }
            };
            let (opcount, memsize) = determine_operand_size(optypes);
            if input.len() < (memsize + offset + 1) as usize {
                return None;
            }
            let operands = determine_var_operands(
                optypes,
                input.clone().split_off(offset as usize + 1),
            );
            Some(ZInstruction{
                opcode: opcode,
                operand_count: opcount,
                operands: operands
            })
        },
    }
}

mod tests {
    use crate::zcode::{ZInstruction, ZOperand};
    use super::decode_instruction;

    #[test]
    fn test_decode_detects_long_form_2op_small_small(){
        for i in 0x00..0x1f {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a, 0xa5]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b11111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Small{value: 0x5a}),
                    Some(ZOperand::Small{value: 0xa5}),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i, 0x5a]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_long_form_2op_small_variable(){
        for i in 0x20..0x3f {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a, 0xa5]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b11111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Small{value: 0x5a}),
                    Some(ZOperand::Variable {value: 0xa5}),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i, 0x5a]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_long_form_2op_variable_small(){
        for i in 0x40..0x5f {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a, 0xa5]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b11111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Variable{value: 0x5a}),
                    Some(ZOperand::Small{value: 0xa5}),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i, 0x5a]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_long_form_2op_variable_variable(){
        for i in 0x60..0x7f {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a, 0xa5]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b11111,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Variable{value: 0x5a}),
                    Some(ZOperand::Variable{value: 0xa5}),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i, 0x5a]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_short_form_1op_large(){
        for i in 0x80..0x8f {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a, 0xa5]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b1111,
                operand_count: 1,
                operands: [
                    Some(ZOperand::Large{value: [0x5a, 0xa5]}),
                    None, None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i, 0x5a]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_short_form_1op_small(){
        for i in 0x90..0x9f {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b1111,
                operand_count: 1,
                operands: [
                    Some(ZOperand::Small{value: 0x5a}),
                    None, None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_short_form_1op_variable(){
        for i in 0xa0..0xaf {
            // correct number of operands
            let decoded = decode_instruction(vec![i, 0x5a]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b1111,
                operand_count: 1,
                operands: [
                    Some(ZOperand::Variable{value: 0x5a}),
                    None, None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![i]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_short_form_0op(){
        for i in 0xb0..0xbf {
            if i == 0xbe {
                // skip extended instructions
                continue;
            }
            // correct number of operands
            let decoded = decode_instruction(vec![i]).unwrap();
            let expected = ZInstruction {
                opcode: i & 0b1111,
                operand_count: 0,
                operands: [
                    None, None, None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);
            
            // wrong number of operands
            let decoded = decode_instruction(vec![]);
            let expected: Option<_> = None;
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_detects_variable_form_2op_var(){
        for i in 0xc0..=0xdf {
            let opcode: u8 = i & 0b11111;

            // omitting all operands is illegal
            let optypes = 0b11111111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]);
            assert_eq!(decoded, None);

            // only 1 operand provided is illegal
            let opcode: u8 = i & 0b11111;
            let optypes = 0b00111111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]);
            assert_eq!(decoded, None);

            // provide 2 large constants
            let optypes = 0b00001111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03, 0x04,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide 2 large constants - but not enough to read from
            let optypes = 0b00001111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03,
            ]);
            assert_eq!(decoded, None);

            // provide 2 small constants
            let optypes = 0b01011111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x03,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Small { value: 0x01 }),
                    Some(ZOperand::Small { value: 0x03 }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide 2 small constants - but not enough to read from
            let optypes = 0b01011111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01,
            ]);
            assert_eq!(decoded, None);

            // provide 2 variables
            let optypes = 0b10101111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x03,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Variable { value: 0x01 }),
                    Some(ZOperand::Variable { value: 0x03 }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide 2 variables - but not enough to read from
            let optypes = 0b10101111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01,
            ]);
            assert_eq!(decoded, None);

            // provide large constant and variable
            let optypes = 0b00101111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Variable { value: 0x03 }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide large constant and variable - but not enough to read from
            let optypes = 0b00101111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01,
            ]);
            assert_eq!(decoded, None);

            // provide three large constants - third will be ignored
            // (even if not enough mem is provided)
            let optypes = 0b00000011;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03, 0x04
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large{ value: [0x03, 0x04] }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

        }
    }

    #[test]
    fn test_decode_detects_variable_form_opcount_var() {
        for i in 0xe0..=0xff {
            let opcode: u8 = i & 0b11111;

            // skip double var instructions
            if opcode == 12 || opcode == 26 {
                continue;
            }

            // provide 2 large constants
            let optypes = 0b00001111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03, 0x04,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide 2 large constants - but not enough to read from
            let optypes = 0b00001111;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03,
            ]);
            assert_eq!(decoded, None);

            // provide 3 large constants
            let optypes = 0b00000011;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 3,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    Some(ZOperand::Large { value: [0x05, 0x06] }),
                    None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide 3 large constants - but split by
            // ommitted -> ignore third operand and interpret
            // as 2OP
            let optypes = 0b00001100;
            let decoded = decode_instruction(vec![
                i, optypes,
                0x01, 0x02, 0x03, 0x04
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 2,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    None, None, None, None, None, None
                ]
            };
            assert_eq!(decoded, expected);



        }
    }

    #[test]
    fn test_decode_detects_variable_form_opcount_var_except_12_and_26() {
        for i in 0xe0..=0xff {
            let opcode: u8 = i & 0b11111;

            // skip double var instructions
            if opcode != 12 && opcode != 26 {
                continue;
            }

            // provide 6 large constants - but not enough
            // memory to read the types
            let optypes = 0b0000000;
            let decoded = decode_instruction(vec![
                i, optypes,
            ]);
            assert_eq!(decoded, None);

            // provide 6 large constants - but not enough
            // memory to read the operands
            let optypes0 = 0b00000000;
            let optypes1 = 0b00001111;
            let decoded = decode_instruction(vec![
                i, optypes0, optypes1,
            ]);
            assert_eq!(decoded, None);

            // provide 6 large constants
            let optypes0 = 0b00000000;
            let optypes1 =0b00001111;
            let decoded = decode_instruction(vec![
                i, optypes0, optypes1,
                0x01, 0x02, 0x03, 0x04,
                0x01, 0x02, 0x03, 0x04,
                0x01, 0x02, 0x03, 0x04,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 6,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Large { value: [0x03, 0x04] }),
                    None, None
                ]
            };
            assert_eq!(decoded, expected);

            // provide 4 large constants - mix with variable and short
            let optypes0 = 0b00010000;
            let optypes1 =0b10001111;
            let decoded = decode_instruction(vec![
                i, optypes0, optypes1,
                0x01, 0x02,
                0x03,
                0x04,0x01, 0x02, 0x03,
                0x04,
                0x01, 0x02,
            ]).unwrap();
            let expected = ZInstruction {
                opcode: opcode,
                operand_count: 6,
                operands: [
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    Some(ZOperand::Small { value: 0x03 }),
                    Some(ZOperand::Large { value: [0x04, 0x01] }),
                    Some(ZOperand::Large { value: [0x02, 0x03] }),
                    Some(ZOperand::Variable { value: 0x04 }),
                    Some(ZOperand::Large { value: [0x01, 0x02] }),
                    None, None
                ]
            };
            assert_eq!(decoded, expected);

        }
    }

}