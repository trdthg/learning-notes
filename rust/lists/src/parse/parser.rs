use std::collections::HashMap;


use super::token::{self, Token, TokenState, TokenType};

// 兼容更多keyword
// 区别变量类型


#[derive(Clone, Copy)]
pub enum ParseState {
    Begin,
    InSelect,
    InFrom,
    InWhere,
    InInsert,
    InValues,
    InUpdate,
    InDelete,
    InSet,
    InOrder,
    End,
}

#[derive(Debug)]
pub struct Parser {
    method: String,
    table: String,
    map_c: Vec<Ope>,
    map_s: Vec<Ope>,
}

#[derive(Debug)]
pub struct Ope {
    key: Option<String>,
    operation: Option<String>,
    value: Option<String>,
}

impl Parser {
    pub fn parse(token_stream: Vec<Token>) ->Parser {
        let mut method: &str = "";
        let mut table: &str = "";
        let mut map_c: Vec<Ope> = Vec::new();
        let mut map_s: Vec<Ope> = Vec::new();

        let mut parsestate = ParseState::Begin;

        for (i, token) in token_stream.iter().enumerate() {
            let value = token.value.as_str();
            let tokentype = &token.tokentype;
            match tokentype {
                TokenType::KeyWord => match value {
                    "select" => {
                        method = "select";
                        parsestate = ParseState::InSelect;
                    }
                    "insert" => {
                        method = "insert";
                        parsestate = ParseState::InInsert;
                    }
                    "update" => {
                        method = "update";
                        parsestate = ParseState::InUpdate;
                    }
                    "delete" => {
                        method = "delete";
                        parsestate = ParseState::InDelete;
                    }
                    "from" => {
                        parsestate = ParseState::InFrom;
                    }
                    "where" => {
                        parsestate = ParseState::InWhere;
                    }
                    "values" => {
                        parsestate = ParseState::InValues;
                    }
                    "set" => {
                        parsestate = ParseState::InSet;
                    }
                    _ => {}
                }
                
                
                TokenType::String => match parsestate {
                    ParseState::InInsert => {
                        let a = 1;
                        table = &value;
                    }
                    ParseState::InFrom => {
                        table = &value;
                    }
                    ParseState::InUpdate => {
                        table = &value;
                    }
                    _ => {}
                }
                TokenType::Operation => match value {
                    "*" => match parsestate {
                        ParseState::InSelect => {}
                        _ => {}
                    }
                    "=" => match parsestate {
                        ParseState::InWhere => {
                            match tokentype {
                                TokenType::Operation => {
                                    if value == "=" {
                                        if let (Some(k), Some(v)) = (token_stream.get(i - 1), token_stream.get(i + 1)) {
                                            map_c.push(Ope{operation: Some("=".to_string()), key: Some(k.value.clone()), value: Some(v.value.clone())});
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        ParseState::InSet => {
                            match tokentype {
                                TokenType::Operation => {
                                    if value == "=" {
                                        if let (Some(k), Some(v)) = (token_stream.get(i - 1), token_stream.get(i + 1)) {
                                            map_s.push(Ope{operation: Some("=".to_string()), key: Some(k.value.clone()), value: Some(v.value.clone())});
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        
                        _ => {}
                    }
                    
                    _ => {}
                }
                TokenType::Boundary => match value {
                    "(" => match parsestate {
                        ParseState::InInsert => {
                            let mut mark = i;
                            loop {
                                if let Some(t) = token_stream.get(mark) {
                                    match t.tokentype {
                                        TokenType::String => {
                                            map_s.push(Ope{operation: None, key: Some(t.value.clone()), value: None})
                                        }
                                        TokenType::Boundary => {
                                            if t.value.as_str() == ")" {
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                    mark += 1;
                                }
                            }
                        }
                        ParseState::InValues => {
                            let mut mark = i;
                            let mut map_s_i = 0;
                            loop {
                                if let Some(t) = token_stream.get(mark) {
                                    match t.tokentype {
                                        TokenType::String => {
                                            let target = map_s.get_mut(map_s_i).unwrap();
                                            target.value = Some(t.value.clone());
                                            map_s_i += 1;
                                        }
                                        TokenType::Boundary => {
                                            if t.value.as_str() == ")" {
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                    mark += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                    _ => {}
                }
                TokenType::Number => {}
            }
        }

        Parser {
            method: method.to_string(), 
            table: table.to_string(), 
            map_s: map_s,
            map_c: map_c,
        }
    }
}

#[test]
fn test() {
    let sql = "SELECT  * from  adwdw where   a   =ad  and b=ad ";
    let sql = "   \ninsert into user(id,name)values(1,\"saadwdd\")where id=1; ";
    let sql = "update  user  set          id=1,name=\"acbeix\" where xxx=\"debsxnk\"";
    let sql = "delete from user where id = 12";
    let mut token_stream = token::trim_to_token_stream(&token::trim_code(sql));
    println!("{:#?}", token_stream);
    let mut parser: Parser = Parser::parse(token_stream);
    println!("{:#?}", parser);
}
