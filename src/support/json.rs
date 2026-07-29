use std::collections::BTreeMap;

const MAX_JSON_DEPTH: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum JsonValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub(crate) fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(value) => value.parse().ok(),
            _ => None,
        }
    }

    pub(crate) fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct JsonError {
    pub(crate) offset: usize,
    pub(crate) message: &'static str,
}

pub(crate) fn parse(bytes: &[u8]) -> Result<JsonValue, JsonError> {
    let mut parser = Parser { bytes, offset: 0 };
    let value = parser.parse_value(0)?;
    parser.skip_whitespace();
    if parser.offset != bytes.len() {
        return Err(parser.error("unexpected trailing content"));
    }
    Ok(value)
}

pub(crate) fn escape_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            value if value < '\u{20}' => {
                use std::fmt::Write as _;
                let _ = write!(output, "\\u{:04x}", value as u32);
            }
            value => output.push(value),
        }
    }
    output.push('"');
    output
}

struct Parser<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl Parser<'_> {
    fn parse_value(&mut self, depth: usize) -> Result<JsonValue, JsonError> {
        if depth > MAX_JSON_DEPTH {
            return Err(self.error("JSON nesting limit exceeded"));
        }
        self.skip_whitespace();
        match self.peek() {
            Some(b'n') => self.parse_literal(b"null", JsonValue::Null),
            Some(b't') => self.parse_literal(b"true", JsonValue::Bool(true)),
            Some(b'f') => self.parse_literal(b"false", JsonValue::Bool(false)),
            Some(b'"') => self.parse_string().map(JsonValue::String),
            Some(b'[') => self.parse_array(depth),
            Some(b'{') => self.parse_object(depth),
            Some(b'-' | b'0'..=b'9') => self.parse_number().map(JsonValue::Number),
            Some(_) => Err(self.error("unexpected JSON token")),
            None => Err(self.error("unexpected end of JSON")),
        }
    }

    fn parse_literal(&mut self, literal: &[u8], value: JsonValue) -> Result<JsonValue, JsonError> {
        if self.bytes.get(self.offset..self.offset + literal.len()) == Some(literal) {
            self.offset += literal.len();
            Ok(value)
        } else {
            Err(self.error("invalid JSON literal"))
        }
    }

    fn parse_object(&mut self, depth: usize) -> Result<JsonValue, JsonError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut object = BTreeMap::new();
        if self.consume(b'}') {
            return Ok(JsonValue::Object(object));
        }
        loop {
            self.skip_whitespace();
            if self.peek() != Some(b'"') {
                return Err(self.error("object key must be a string"));
            }
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            let value = self.parse_value(depth + 1)?;
            if object.insert(key, value).is_some() {
                return Err(self.error("duplicate object key"));
            }
            self.skip_whitespace();
            if self.consume(b'}') {
                break;
            }
            self.expect(b',')?;
        }
        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self, depth: usize) -> Result<JsonValue, JsonError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.consume(b']') {
            return Ok(JsonValue::Array(values));
        }
        loop {
            values.push(self.parse_value(depth + 1)?);
            self.skip_whitespace();
            if self.consume(b']') {
                break;
            }
            self.expect(b',')?;
        }
        Ok(JsonValue::Array(values))
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        self.expect(b'"')?;
        let mut output = String::new();
        let mut segment_start = self.offset;
        loop {
            let byte = self
                .peek()
                .ok_or_else(|| self.error("unterminated string"))?;
            match byte {
                b'"' => {
                    self.push_utf8_segment(&mut output, segment_start, self.offset)?;
                    self.offset += 1;
                    return Ok(output);
                }
                b'\\' => {
                    self.push_utf8_segment(&mut output, segment_start, self.offset)?;
                    self.offset += 1;
                    let escaped = self
                        .next()
                        .ok_or_else(|| self.error("unterminated escape"))?;
                    match escaped {
                        b'"' => output.push('"'),
                        b'\\' => output.push('\\'),
                        b'/' => output.push('/'),
                        b'b' => output.push('\u{08}'),
                        b'f' => output.push('\u{0c}'),
                        b'n' => output.push('\n'),
                        b'r' => output.push('\r'),
                        b't' => output.push('\t'),
                        b'u' => self.parse_unicode_escape(&mut output)?,
                        _ => return Err(self.error("invalid string escape")),
                    }
                    segment_start = self.offset;
                }
                0x00..=0x1f => return Err(self.error("control character in string")),
                _ => self.offset += 1,
            }
        }
    }

    fn parse_unicode_escape(&mut self, output: &mut String) -> Result<(), JsonError> {
        let first = self.parse_hex_quad()?;
        let code_point = if (0xd800..=0xdbff).contains(&first) {
            if self.next() != Some(b'\\') || self.next() != Some(b'u') {
                return Err(self.error("high surrogate without low surrogate"));
            }
            let second = self.parse_hex_quad()?;
            if !(0xdc00..=0xdfff).contains(&second) {
                return Err(self.error("invalid low surrogate"));
            }
            0x1_0000 + (((first - 0xd800) as u32) << 10) + (second - 0xdc00) as u32
        } else if (0xdc00..=0xdfff).contains(&first) {
            return Err(self.error("unexpected low surrogate"));
        } else {
            first as u32
        };
        let character = char::from_u32(code_point).ok_or_else(|| self.error("invalid Unicode"))?;
        output.push(character);
        Ok(())
    }

    fn parse_hex_quad(&mut self) -> Result<u16, JsonError> {
        let mut value = 0_u16;
        for _ in 0..4 {
            let digit = self
                .next()
                .ok_or_else(|| self.error("short Unicode escape"))?;
            let digit_value =
                hex_value(digit).ok_or_else(|| self.error("invalid Unicode escape"))?;
            value = value
                .checked_mul(16)
                .and_then(|value| value.checked_add(u16::from(digit_value)))
                .ok_or_else(|| self.error("invalid Unicode escape"))?;
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<String, JsonError> {
        let start = self.offset;
        self.consume(b'-');
        match self.peek() {
            Some(b'0') => self.offset += 1,
            Some(b'1'..=b'9') => {
                self.offset += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.offset += 1;
                }
            }
            _ => return Err(self.error("invalid number")),
        }
        if self.consume(b'.') {
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.error("invalid fraction"));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.offset += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.offset += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.offset += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.error("invalid exponent"));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.offset += 1;
            }
        }
        String::from_utf8(self.bytes[start..self.offset].to_vec())
            .map_err(|_| self.error("number is not UTF-8"))
    }

    fn push_utf8_segment(
        &self,
        output: &mut String,
        start: usize,
        end: usize,
    ) -> Result<(), JsonError> {
        let segment = std::str::from_utf8(&self.bytes[start..end]).map_err(|_| JsonError {
            offset: start,
            message: "string is not UTF-8",
        })?;
        output.push_str(segment);
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.offset += 1;
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), JsonError> {
        if self.consume(expected) {
            Ok(())
        } else {
            Err(self.error("unexpected JSON delimiter"))
        }
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<u8> {
        let value = self.peek()?;
        self.offset += 1;
        Some(value)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.offset).copied()
    }

    fn error(&self, message: &'static str) -> JsonError {
        JsonError {
            offset: self.offset,
            message,
        }
    }
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonValue, escape_string, parse};

    #[test]
    fn parses_nested_json_and_unicode() {
        let value =
            parse(br#"{"schemaVersion":1,"name":"Werkstatt \uD83D\uDD25","items":[true,null]}"#)
                .unwrap();
        let object = value.as_object().unwrap();
        assert_eq!(object["schemaVersion"].as_u64(), Some(1));
        assert_eq!(object["name"].as_str(), Some("Werkstatt 🔥"));
        assert!(matches!(object["items"], JsonValue::Array(_)));
    }

    #[test]
    fn rejects_duplicate_keys_trailing_content_and_excessive_nesting() {
        assert!(parse(br#"{"x":1,"x":2}"#).is_err());
        assert!(parse(br#"{"x":1} false"#).is_err());

        let nested = format!("{}null{}", "[".repeat(65), "]".repeat(65));
        let error = parse(nested.as_bytes()).unwrap_err();
        assert_eq!(error.message, "JSON nesting limit exceeded");
    }

    #[test]
    fn escapes_strings_deterministically() {
        assert_eq!(escape_string("a\n\"b"), "\"a\\n\\\"b\"");
    }
}
