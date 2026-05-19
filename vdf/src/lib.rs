use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VdfValue {
    Str(String),
    Obj(BTreeMap<String, VdfValue>),
}

impl VdfValue {
    pub fn get(&self, key: &str) -> Option<&VdfValue> {
        match self {
            VdfValue::Obj(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut VdfValue> {
        match self {
            VdfValue::Obj(map) => map.get_mut(key),
            _ => None,
        }
    }

    pub fn get_str(&self) -> Option<&str> {
        match self {
            VdfValue::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_obj(&self) -> Option<&BTreeMap<String, VdfValue>> {
        match self {
            VdfValue::Obj(map) => Some(map),
            _ => None,
        }
    }

    pub fn get_obj_mut(&mut self) -> Option<&mut BTreeMap<String, VdfValue>> {
        match self {
            VdfValue::Obj(map) => Some(map),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Str(String),
    OpenBrace,
    CloseBrace,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::with_capacity(input.len() / 10);
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == '/' {
            chars.next();
            if chars.peek() == Some(&'/') {
                for nc in chars.by_ref() {
                    if nc == '\n' || nc == '\r' {
                        break;
                    }
                }
                continue;
            } else {
                return Err("unexpected single slash '/'".into());
            }
        }

        if c == '{' {
            chars.next();
            tokens.push(Token::OpenBrace);
            continue;
        }

        if c == '}' {
            chars.next();
            tokens.push(Token::CloseBrace);
            continue;
        }

        if c == '"' {
            chars.next();
            let mut s = String::with_capacity(16);
            let mut escaped = false;
            let mut matched = false;
            for nc in chars.by_ref() {
                if escaped {
                    s.push(nc);
                    escaped = false;
                } else if nc == '\\' {
                    escaped = true;
                } else if nc == '"' {
                    matched = true;
                    break;
                } else {
                    s.push(nc);
                }
            }
            if !matched {
                return Err("unterminated double quote".into());
            }
            tokens.push(Token::Str(s));
            continue;
        }

        let mut s = String::with_capacity(16);
        while let Some(&nc) = chars.peek() {
            if nc.is_whitespace() || nc == '{' || nc == '}' || nc == '"' || nc == '/' {
                break;
            }
            s.push(chars.next().unwrap());
        }
        if s.is_empty() {
            return Err(format!("unexpected character: {:?}", c));
        }
        tokens.push(Token::Str(s));
    }

    Ok(tokens)
}

fn parse_map(tokens: &[Token], index: &mut usize) -> Result<BTreeMap<String, VdfValue>, String> {
    let mut map = BTreeMap::new();
    while *index < tokens.len() {
        match &tokens[*index] {
            Token::CloseBrace => {
                break;
            }
            Token::Str(key) => {
                *index += 1;
                if *index >= tokens.len() {
                    return Err(format!("expected value for key '{}', reached EOF", key));
                }
                match &tokens[*index] {
                    Token::Str(val) => {
                        map.insert(key.clone(), VdfValue::Str(val.clone()));
                        *index += 1;
                    }
                    Token::OpenBrace => {
                        *index += 1;
                        let sub_map = parse_map(tokens, index)?;
                        if *index >= tokens.len() || tokens[*index] != Token::CloseBrace {
                            return Err(format!("expected '}}' to close block for key '{}'", key));
                        }
                        *index += 1;
                        map.insert(key.clone(), VdfValue::Obj(sub_map));
                    }
                    Token::CloseBrace => {
                        return Err(format!("unexpected '}}' as value for key '{}'", key));
                    }
                }
            }
            Token::OpenBrace => {
                return Err("unexpected '{' where key was expected".into());
            }
        }
    }
    Ok(map)
}

pub fn parse(input: &str) -> Result<BTreeMap<String, VdfValue>, String> {
    let tokens = tokenize(input)?;
    let mut index = 0;
    let map = parse_map(&tokens, &mut index)?;
    if index < tokens.len() {
        return Err("unexpected tokens after parsing root map completed".into());
    }
    Ok(map)
}

pub fn stringify(map: &BTreeMap<String, VdfValue>) -> String {
    let mut out = String::with_capacity(1024);
    stringify_internal(map, 0, &mut out);
    out
}

fn stringify_internal(map: &BTreeMap<String, VdfValue>, indent: usize, out: &mut String) {
    let spaces = "\t".repeat(indent);
    for (key, val) in map {
        match val {
            VdfValue::Str(s) => {
                out.push_str(&spaces);
                out.push('"');
                write_escaped_str(key, out);
                out.push_str("\"\t\t\"");
                write_escaped_str(s, out);
                out.push_str("\"\n");
            }
            VdfValue::Obj(sub_map) => {
                out.push_str(&spaces);
                out.push('"');
                write_escaped_str(key, out);
                out.push_str("\"\n");

                out.push_str(&spaces);
                out.push_str("{\n");

                stringify_internal(sub_map, indent + 1, out);

                out.push_str(&spaces);
                out.push_str("}\n");
            }
        }
    }
}

fn write_escaped_str(s: &str, out: &mut String) {
    out.reserve(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
}
