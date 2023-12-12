use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::sync::Arc;

use Category::*;

use crate::text::Text;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Category {
    /// 开括号
    BracketOpen,
    /// 闭括号
    BracketClosed,
    /// 分隔符
    Delimiter,
    /// 未识别
    Unknown,
    /// 已识别
    Identifier,
    /// 已处理（失效）
    Invalid,
}

struct Inner(Category, Text, bool);

#[derive(Clone)]
struct InnerRef(Arc<RefCell<Inner>>);

impl InnerRef {
    fn category(&self) -> Category {
        self.0.borrow().0
    }

    fn text(&self) -> Text {
        self.0.borrow().1.clone()
    }

    fn enclosed(&self) -> bool {
        self.0.borrow().2
    }

    fn set_category(&mut self, category: Category) {
        self.0.borrow_mut().0 = category
    }
}

impl PartialEq for InnerRef {
    fn eq(&self, other: &Self) -> bool {
        (&*self.0.borrow() as *const Inner) == (&*other.0.borrow() as *const Inner)
    }
}

impl Debug for InnerRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.enclosed() {
            write!(f, "[{:?}({})]", self.category(), self.text())
        } else {
            write!(f, "{:?}({})", self.category(), self.text())
        }
    }
}

#[derive(Clone)]
pub(crate) struct Token(Option<InnerRef>);

impl Token {
    fn new(category: Category, text: Text, enclosed: bool) -> Self {
        let inner = Inner(category, text, enclosed);
        Self(Some(InnerRef(Arc::new(RefCell::new(inner)))))
    }

    pub fn bracket_open(text: Text, enclosed: bool) -> Self {
        Token::new(BracketOpen, text, enclosed)
    }

    pub fn bracket_closed(text: Text, enclosed: bool) -> Self {
        Token::new(BracketClosed, text, enclosed)
    }

    pub fn delimiter(text: Text, enclosed: bool) -> Self {
        Token::new(Delimiter, text, enclosed)
    }

    pub fn unknown(text: Text, enclosed: bool) -> Self {
        Token::new(Unknown, text, enclosed)
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn eq_category(&self, category: Category) -> bool {
        self.0
            .as_ref()
            .map(|it| it.category() == category)
            .unwrap_or(false)
    }

    pub fn is_unknown(&self) -> bool {
        self.eq_category(Unknown)
    }

    pub fn is_open_bracket(&self) -> bool {
        self.eq_category(BracketOpen)
    }

    pub fn is_closed_bracket(&self) -> bool {
        self.eq_category(BracketClosed)
    }

    pub fn is_delimiter(&self) -> bool {
        self.eq_category(Delimiter)
    }

    pub fn is_valid(&self) -> bool {
        self.0
            .as_ref()
            .map(|it| it.category() != Invalid)
            .unwrap_or(false)
    }

    pub fn enclosed(&self) -> bool {
        self.0.as_ref().map(|it| it.enclosed()).unwrap_or(false)
    }

    pub fn to_text(&self) -> Text {
        self.0.as_ref().map(|it| it.text()).unwrap_or_default()
    }

    pub fn set_unknown(&mut self) {
        self.0.as_mut().map(|it| it.set_category(Unknown));
    }

    pub fn set_identifier(&mut self) {
        self.0.as_mut().map(|it| it.set_category(Identifier));
    }
}

impl From<InnerRef> for Token {
    fn from(value: InnerRef) -> Self {
        Token(Some(value))
    }
}

impl From<&InnerRef> for Token {
    fn from(value: &InnerRef) -> Self {
        Token(Some(value.clone()))
    }
}

impl From<Option<&InnerRef>> for Token {
    fn from(value: Option<&InnerRef>) -> Self {
        Token(value.map(|it| it.clone()))
    }
}

impl PartialEq<&InnerRef> for Token {
    fn eq(&self, other: &&InnerRef) -> bool {
        match self.0.as_ref() {
            None => false,
            Some(it) => *it == **other,
        }
    }
}

impl PartialEq<&Token> for Token {
    fn eq(&self, other: &&Token) -> bool {
        *self == **other
    }
}

impl PartialEq<&mut Token> for Token {
    fn eq(&self, other: &&mut Token) -> bool {
        *self == **other
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        match other.0.as_ref() {
            None => self.0.is_none(),
            Some(rhs) => *self == rhs,
        }
    }
}

impl Add for Token {
    type Output = Token;
    fn add(self, mut rhs: Token) -> Self::Output {
        self + &mut rhs
    }
}

impl Add<&mut Token> for Token {
    type Output = Token;
    fn add(self, rhs: &mut Token) -> Self::Output {
        match (self.0.as_ref(), rhs.0.as_mut()) {
            (Some(lhs), Some(rhs)) => {
                rhs.set_category(Invalid);
                Token::new(lhs.category(), lhs.text() + rhs.text(), lhs.enclosed())
            }
            (None, Some(rhs)) => {
                rhs.set_category(Invalid);
                Token::new(rhs.category(), rhs.text(), rhs.enclosed())
            }
            _ => self,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            None => write!(f, "None"),
            Some(it) => write!(f, "{:?}", *it),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Tokens(Vec<InnerRef>);

impl Tokens {
    fn find_idx(&self, token: &Token) -> Option<usize> {
        self.0.iter().position(|it| *token == it)
    }

    fn find_first<F>(&self, f: F) -> Token
    where
        F: Fn(&InnerRef) -> bool,
    {
        Token::from(self.0.iter().find(|it| f(*it)))
    }

    fn find_prev<F>(&self, idx: &Token, f: F) -> Token
    where
        F: Fn(&InnerRef) -> bool,
    {
        let idx = self.find_idx(idx);
        let prev = idx.map(|mid| self.0.split_at(mid).0);
        let item = prev.and_then(|it| it.iter().rfind(|it| f(*it)));
        Token::from(item)
    }

    fn find_next<F>(&self, idx: &Token, f: F) -> Token
    where
        F: Fn(&InnerRef) -> bool,
    {
        let idx = self.find_idx(idx);
        let next = idx.map(|mid| self.0.split_at(mid).1);
        let next = next.map(|it| if it.is_empty() { it } else { &it[1..] });
        let item = next.and_then(|it| it.iter().find(|it| f(*it)));
        Token::from(item)
    }

    fn sub_vec(&self, begin: usize, end: usize) -> impl Iterator<Item = Token> + '_ {
        let len = end.saturating_sub(begin);
        self.0.iter().skip(begin).take(len).map(Token::from)
    }
}

impl Tokens {
    pub fn new(text: Text) -> Self {
        let token = Token::unknown(text, false);
        let vec = Vec::from([token.0.unwrap()]);
        Tokens(vec)
    }

    /// 首个开括号
    pub fn first_open_bracket(&self) -> Token {
        self.find_first(|it| it.category() == BracketOpen)
    }

    /// 首个未识别 token
    pub fn first_unknown(&self) -> Token {
        self.find_first(|it| it.category() == Unknown)
    }

    /// tokens 的 [0, len) 切片
    pub fn all_tokens(&self) -> impl Iterator<Item = Token> + '_ {
        self.0.iter().map(Token::from)
    }

    /// tokens 的未识别 token 切片
    pub fn unknown_tokens(&self) -> impl Iterator<Item = Token> + '_ {
        let it = self.0.iter().filter(|it| it.category() == Unknown);
        it.map(Token::from)
    }

    /// tokens 的 [l, r) 切片
    pub fn sub_tokens<'a>(&'a self, l: &Token, r: &Token) -> impl Iterator<Item = Token> + 'a {
        let begin = self.find_idx(l).unwrap_or(usize::MAX);
        let end = self.find_idx(r).unwrap_or(0);
        self.sub_vec(begin, end)
    }

    /// tokens 的 [start, len) 切片
    pub fn sub_tokens_start<'a>(&'a self, start: &Token) -> impl Iterator<Item = Token> + 'a {
        let begin = self.find_idx(start).unwrap_or(usize::MAX);
        let end = self.0.len();
        self.sub_vec(begin, end)
    }

    /// 查找前一个未识别 token
    pub fn find_prev_unknown(&self, idx: &Token) -> Token {
        self.find_prev(idx, |it| it.category() == Unknown)
    }

    /// 查找前一个合法 token
    pub fn find_prev_valid(&self, idx: &Token) -> Token {
        self.find_prev(idx, |it| it.category() != Invalid)
    }

    /// 查找前一个非分隔符 token
    pub fn find_prev_not_delimiter(&self, idx: &Token) -> Token {
        self.find_prev(idx, |it| {
            it.category() != Invalid && it.category() != Delimiter
        })
    }

    /// 查找下一个未识别 token
    pub fn find_next_unknown(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| it.category() == Unknown)
    }

    /// 查找下一个合法 token
    pub fn find_next_valid(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| it.category() != Invalid)
    }

    /// 查找下一个非分隔符 token
    pub fn find_next_not_delimiter(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| {
            it.category() != Invalid && it.category() != Delimiter
        })
    }

    /// 查找下一个括号内非分隔符 token
    pub fn find_next_enclosed_not_delimiter(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| {
            it.category() != Invalid && it.category() != Delimiter && it.enclosed()
        })
    }

    /// 查找下一个括号或者已识别 token
    pub fn find_next_bracket_or_identifier(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| {
            it.category() == Identifier
                || it.category() == BracketOpen
                || it.category() == BracketClosed
        })
    }

    /// 在 idx 的位置上将原先的 token 替换为新的多个 token
    pub fn replace<T, I>(&mut self, idx: &Token, tokens: T)
    where
        T: IntoIterator<IntoIter = I>,
        I: DoubleEndedIterator<Item = Token> + Sized,
    {
        if let Some(idx) = self.find_idx(idx) {
            self.0.remove(idx);
            for token in tokens.into_iter().rev() {
                if let Some(token) = token.0 {
                    self.0.insert(idx, token)
                }
            }
        }
    }
}
