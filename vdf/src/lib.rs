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
        match c {
            x if x.is_whitespace() => {
                chars.next();
                continue;
            }

            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    for nc in chars.by_ref() {
                        if nc == '\n' || nc == '\r' {
                            break;
                        }
                    }
                    continue;
                } else {
                    return Err("unexpected single slash '/'".to_string());
                }
            }
            '{' => {
                chars.next();
                tokens.push(Token::OpenBrace);
                continue;
            }

            '}' => {
                chars.next();
                tokens.push(Token::CloseBrace);
                continue;
            }

            '"' => {
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
                    return Err("unterminated double quote".to_string());
                }
                tokens.push(Token::Str(s));
                continue;
            }

            _ => {}
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
                return Err("unexpected '{' where key was expected".to_string());
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
        return Err("unexpected tokens after parsing root map completed".to_string());
    }
    Ok(map)
}

fn escaped_len(s: &str) -> usize {
    s.chars()
        .map(|c| match c {
            '"' | '\\' | '\n' | '\r' | '\t' => 2,
            _ => c.len_utf8(),
        })
        .sum()
}

fn size_hint_map(map: &BTreeMap<String, VdfValue>, indent: usize) -> usize {
    let mut size = 0;

    for (key, val) in map {
        match val {
            VdfValue::Str(s) => {
                // tabs for indent
                size += indent;
                // opening + separator + closing formatting:
                // "\"" + "\"\t\t\"" + "\"\n"
                size += 6;
                size += escaped_len(key);
                size += escaped_len(s);
            }
            VdfValue::Obj(sub_map) => {
                // indent + "\"" + key + "\"\n"
                size += indent + 3 + escaped_len(key);
                // indent + "{\n"
                size += indent + 2;
                // nested content
                size += size_hint_map(sub_map, indent + 1);
                // indent + "}\n"
                size += indent + 2;
            }
        }
    }

    size
}

pub fn stringify(map: &BTreeMap<String, VdfValue>) -> String {
    let mut out = String::with_capacity(size_hint_map(map, 0));
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
