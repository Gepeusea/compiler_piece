
use core::panic;
use std::iter::Peekable;


#[derive(Debug, Clone, Copy)]
enum Sign {
    Plus,
    Minus,
    Multiply,
    Division,
}

#[derive(Debug)]
enum Token {
    Signs(Sign), //done 
    Equal(char), //done
    Number(f64), //done
    Ident(String), //done
    Bracket(char), //done
    Endofline(char), //done 
}

fn is_ident (inp : &char, ident : &String) -> bool {
    if (inp.is_alphabetic()) || 
        ((inp.is_digit(10) || (*inp == '.')) &&  !(ident.is_empty())) ||
        matches!(inp, '@' | '_') {
            return true;
    } else {
        return false;
    }
}

fn lexer(input: &str) -> Vec<Token> {
    let mut vec : Vec<Token> = Vec::new();
    let mut number : String = String::new();
    let mut ident : String = String::new();
    let mut iter = input.chars().peekable();
    while let Some(cur_char) = iter.next() { 
        if cur_char.is_whitespace() {
            continue;

        } else if matches!(cur_char, '(' | ')'){
            vec.push(Token::Bracket(cur_char)); 

        } else if cur_char == ';'{
            vec.push(Token::Endofline(cur_char)); 

        } else if cur_char == '='{
            vec.push(Token::Equal(cur_char)); 

        } else if matches!(cur_char, '+' | '-' | '*' | '/' | '%'){
            vec.push(match cur_char {
                    '+' => Token::Signs(Sign::Plus),
                    '-' => Token::Signs(Sign::Minus),
                    '*' => Token::Signs(Sign::Multiply),
                    '/' => Token::Signs(Sign::Division),
                    _=> continue,
        }) 

        } else if is_ident(&cur_char, &ident){
            ident.push(cur_char);
            if let Some(next_char) = iter.peek() {
                if !(is_ident(&next_char, &ident)) {
                    vec.push(Token::Ident(ident)); 
                    ident = String::new();
                }
            } else {
                vec.push(Token::Ident(ident)); 
                ident = String::new();
            }

        } else if cur_char.is_digit(10) || cur_char == '.'{
            number.push(cur_char);
            if let Some(next_char) = iter.peek() {
                if !(next_char.is_digit(10) || *next_char == '.') {
                    let floatnumb: f64 = number.parse().unwrap();
                    vec.push(Token::Number(floatnumb)); 
                    number = String::new();
                }
            } else {
                let floatnumb: f64 = number.parse().unwrap();
                vec.push(Token::Number(floatnumb)); 
                number= String::new();
            }
        }
    }
    vec

} 

#[derive(Debug, Clone)]
enum Ast {
    Branching(Sign, Box<Ast>, Box<Ast>),
    Number(f64),
    Ident(String),
}

fn priority(sign: Sign) -> i32{
    match sign{
        Sign::Plus => 1,
        Sign::Minus => 1,
        Sign::Division => 2,
        Sign::Multiply => 2,
    }
}

// функция возращает ветвление дерева, в результате выполнения программа вернет готовое дерево
fn operation(mut left : Ast, tokeniter : &mut Peekable<impl Iterator<Item = Token>>) -> Ast{
    if let Some(operator) = tokeniter.next(){
        if let Token::Signs(cur_sign) = operator{
            let right = operand(tokeniter);
            if let Some(next_token) = tokeniter.peek(){  // "подглядываем" на следующий символ, не доставая его
                if let Token::Signs(next_sign) = next_token{
                    if priority(cur_sign) >= priority(*next_sign){
                        left = Ast::Branching(cur_sign, Box::new(left), Box::new(right));
                        operation(left, tokeniter)
                    } else {
                        Ast::Branching(cur_sign, Box::new(left), Box::new(operation(right, tokeniter)))
                    }
                } else if let Token::Bracket(_bracket) = next_token{
                    tokeniter.next();
                    Ast::Branching(cur_sign, Box::new(left), Box::new(right))
                } else if let Token::Endofline(_end) = next_token{
                    tokeniter.next();
                    Ast::Branching(cur_sign, Box::new(left), Box::new(right))
                } else {
                    Ast::Branching(cur_sign, Box::new(left), Box::new(right))
                }
            } else {
                Ast::Branching(cur_sign, Box::new(left), Box::new(right))
            }
        } else {
            panic!("next is not a sign!");
        }
    } else {
        panic!("not a sign!");
    }
}

// функция возвращает лист дерева
// при обработке открывающей скобки начинаем обрабатывать выражение внутри
fn operand(tokeniter: &mut Peekable<impl Iterator<Item = Token>>) -> Ast{
    if let Some(cur_block) = tokeniter.next(){
        if let Token::Number(number) = cur_block{
            Ast::Number(number)
        } else  if let Token::Ident(ident) = cur_block{
            Ast::Ident(ident)
        } else if let Token::Bracket(_bracket) = cur_block{
            operation(operand(tokeniter), tokeniter)
        } else {
            panic!("not a number")
        }
    } else {
        panic!("error")
    }
}


fn main() {
    // на вход подается выражение и разбивается на лексемы
    let tokenized = lexer("p3r3m = 2 * (1 + 3); ");
    println!("{:?}", tokenized);
    let mut tokeniter = tokenized.into_iter().peekable();
    // итеррируемся по токенам, составляя дерево абстрактного синтаксиса
    if let Some(variable_name) = tokeniter.next(){ 
        if let Some(equal_sign) = tokeniter.next(){ 
            println!("{:?}, {:?}, {:?}", variable_name, equal_sign, operation(operand(&mut tokeniter), &mut tokeniter))
        } else {
            panic!("the input has an invalid structure")
        }
    } else {
        panic!("the input has an invalid structure")
    }
    }
