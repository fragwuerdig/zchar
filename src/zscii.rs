pub const ZSCIITAB: [Option<char>;255] = [
    // null
    Some(' '),
    
    // 1-7 - not defined
    None, None, None, None, None, None, None,
    
    // 8 not defined for output
    None,
    
    // 9 tabulation
    Some('\t'),
    
    // 10 - not defined
    None,
    
    // 11 - sentence space
    Some(' '),
    
    // 12 - not defined
    None,
    
    // 13 - new line
    
    Some('\n'),
    
    // 14-26 - not defined
    None, None, None, None, None, None, None, None, None, None, None, None, None,
    
    // 27 - escape on for input
    None,
    
    // 28-31 - not defined
    None, None, None, None,
    
    // 32-126 - standard ASCII
    Some(' '), Some('!'), Some('"'), Some('#'), Some('$'), Some('%'), Some('&'),
    Some('\''), Some('('), Some(')'), Some('*'), Some('+'), Some(','), Some('-'),
    Some('.'), Some('/'), Some('0'), Some('1'), Some('2'), Some('3'), Some('4'),
    Some('5'), Some('6'), Some('7'), Some('8'), Some('9'), Some(':'), Some(';'),
    Some('<'), Some('='), Some('>'), Some('?'), Some('@'), Some('A'), Some('B'),
    Some('C'), Some('D'), Some('E'), Some('F'), Some('G'), Some('H'), Some('I'),
    Some('J'), Some('K'), Some('L'), Some('M'), Some('N'), Some('O'), Some('P'),
    Some('Q'), Some('R'), Some('S'), Some('T'), Some('U'), Some('V'), Some('W'),
    Some('X'), Some('Y'), Some('Z'), Some('['), Some('\\'), Some(']'), Some('^'),
    Some('_'), Some('`'), Some('a'), Some('b'), Some('c'), Some('d'), Some('e'),
    Some('f'), Some('g'), Some('h'), Some('i'), Some('j'), Some('k'), Some('l'),
    Some('m'), Some('n'), Some('o'), Some('p'), Some('q'), Some('r'), Some('s'),
    Some('t'), Some('u'), Some('v'), Some('w'), Some('x'), Some('y'), Some('z'),
    Some('{'), Some('|'), Some('}'), Some('~'),
    
    //127-128 - not defined
    None, None,
    
    // 129-154 - not defined for input 
    None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None,
    
    // 155-223 - unicode translation table (default)
    Some('ä'), Some('ö'), Some('ü'), Some('Ä'), Some('Ö'), Some('Ü'), Some('ß'),
    Some('»'), Some('«'), Some('ë'), Some('ï'), Some('ÿ'), Some('Ë'), Some('Ï'),
    Some('á'), Some('é'), Some('í'), Some('ó'), Some('ú'), Some('ý'), Some('Á'),
    Some('É'), Some('Í'), Some('Ó'), Some('Ú'), Some('Ý'), Some('à'), Some('è'),
    Some('ì'), Some('ò'), Some('ù'), Some('À'), Some('È'), Some('Ì'), Some('Ò'),
    Some('Ù'), Some('â'), Some('ê'), Some('î'), Some('ô'), Some('û'), Some('Â'),
    Some('Ê'), Some('Î'), Some('Ô'), Some('Û'), Some('å'), Some('Å'), Some('ø'),
    Some('Ø'), Some('ã'), Some('ñ'), Some('õ'), Some('Ã'), Some('Ñ'), Some('Õ'),
    Some('æ'), Some('Æ'), Some('ç'), Some('Ç'), Some('þ'), Some('ð'), Some('Þ'),
    Some('Ð'), Some('£'), Some('œ'), Some('Œ'), Some('¡'), Some('¿'),
    
    // 224-251 - unicode translation table (default)
    None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
    
    // 252-254 - not defined for output
    None, None, None,
];

pub const A0: [char;26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z'
    ];

pub const A1: [char;26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z'
];

pub const A2: [char;25] = [
    '^', '0', '1', '2', '3', '4', '5', '6',
    '7', '8', '9', '.', ',', '!', '?', '_',
    '#', '\'', '"', '/', '\\', '-', ':', '(', ')'
];

pub const S: [char;69] = [
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