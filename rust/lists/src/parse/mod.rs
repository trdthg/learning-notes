pub mod parser;
pub mod token;

pub fn pre(sql: &str) -> String {
    unimplemented!();
}

pub fn parse(sql: &str) {}

#[test]
fn test() {
    let select: &str = r"
        select c1,c2,c3 
        from  t1,t2,　t3 
        where condi1=5 and condi6=6 or condi7=7 
        order　by g1,g2";
    let insert: &str = r"
        INSERT INTO table(id, username) values(1, 'awd')
    ";

    parse(select);
}
