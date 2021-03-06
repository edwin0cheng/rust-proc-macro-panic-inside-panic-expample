extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::bridge::{server, TokenTree};
use std::collections::Bound;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use std::vec::IntoIter;

use proc_macro::{Delimiter, Level, LineColumn, Spacing};

//#[derive(Clone)]
//pub struct TokenStream;
type TokenStream = proc_macro2::TokenStream;

pub struct TokenStreamBuilder {
    acc: TokenStream,
}

impl TokenStreamBuilder {
    fn new() -> TokenStreamBuilder {
        TokenStreamBuilder {
            acc: TokenStream::new(),
        }
    }

    fn push(&mut self, stream: TokenStream) {
        self.acc.extend(stream.into_iter())
    }

    fn build(self) -> TokenStream {
        self.acc
    }
}

#[derive(Clone)]
pub struct TokenStreamIter {
    trees: IntoIter<proc_macro2::TokenTree>,
}

//#[derive(Clone)]
//pub struct Group;
type Group = proc_macro2::Group;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct MyPunct(u32);

#[derive(Clone)]
struct MyPunctData(proc_macro2::Punct);

impl Hash for MyPunctData {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        unimplemented!()
    }
}

impl Eq for MyPunctData {}

impl PartialEq for MyPunctData {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct MyIdent(u32);

#[derive(Clone)]
struct MyIdentData(proc_macro2::Ident);

impl Hash for MyIdentData {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        unimplemented!()
    }
}

impl Eq for MyIdentData {}

impl PartialEq for MyIdentData {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

//#[derive(Clone)]
//pub struct Literal;
type Literal = proc_macro2::Literal;

#[derive(Clone)]
pub struct SourceFile;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct MySpan(u32);

#[derive(Copy, Clone)]
struct MySpanData(proc_macro2::Span);

impl Hash for MySpanData {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        unimplemented!()
    }
}

impl Eq for MySpanData {}

impl PartialEq for MySpanData {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(Default)]
struct MySpanInterner {
    spans: HashMap<MySpanData, u32>,
    span_data: Vec<MySpanData>,
}

impl MySpanInterner {
    fn intern(&mut self, data: &MySpanData) -> u32 {
        if let Some(index) = self.spans.get(data) {
            return *index;
        }

        let index = self.spans.len() as u32;
        self.span_data.push(*data);
        self.spans.insert(*data, index);

        index
    }

    fn get(&self, index: u32) -> &MySpanData {
        &self.span_data[index as usize]
    }
}

#[derive(Default)]
struct MyIdentInterner {
    idents: HashMap<MyIdentData, u32>,
    ident_data: Vec<MyIdentData>,
}

impl MyIdentInterner {
    fn intern(&mut self, data: &MyIdentData) -> u32 {
        if let Some(index) = self.idents.get(data) {
            return *index;
        }

        let index = self.idents.len() as u32;
        self.ident_data.push(data.clone());
        self.idents.insert(data.clone(), index);

        index
    }

    fn get(&self, index: u32) -> &MyIdentData {
        &self.ident_data[index as usize]
    }

    fn get_mut(&mut self, index: u32) -> &mut MyIdentData {
        self.ident_data
            .get_mut(index as usize)
            .expect("Should be consistent")
    }
}

#[derive(Default)]
struct MyPunctInterner {
    puncts: HashMap<MyPunctData, u32>,
    punct_data: Vec<MyPunctData>,
}

impl MyPunctInterner {
    fn intern(&mut self, data: &MyPunctData) -> u32 {
        if let Some(index) = self.puncts.get(data) {
            return *index;
        }

        let index = self.puncts.len() as u32;
        self.punct_data.push(data.clone());
        self.puncts.insert(data.clone(), index);

        index
    }

    fn get(&self, index: u32) -> &MyPunctData {
        &self.punct_data[index as usize]
    }

    fn get_mut(&mut self, index: u32) -> &mut MyPunctData {
        self.punct_data
            .get_mut(index as usize)
            .expect("Should be consistent")
    }
}

#[derive(Default)]
pub struct Rustc {
    span_interner: MySpanInterner,
    ident_interner: MyIdentInterner,
    punct_interner: MyPunctInterner,
    //    def_side: MySpan,
    //    call_site: MySpan,
}

impl server::Types for Rustc {
    type TokenStream = TokenStream;
    type TokenStreamBuilder = TokenStreamBuilder;
    type TokenStreamIter = TokenStreamIter;
    type Group = Group;
    type Punct = MyPunct;
    type Ident = MyIdent;
    type Literal = Literal;
    type SourceFile = SourceFile;
    type Diagnostic = proc_macro::Diagnostic;
    type Span = MySpan;
    type MultiSpan = Vec<proc_macro::Span>;
}

impl server::TokenStream for Rustc {
    fn new(&mut self) -> Self::TokenStream {
        unimplemented!()
    }

    fn is_empty(&mut self, _stream: &Self::TokenStream) -> bool {
        unimplemented!()
    }
    fn from_str(&mut self, _src: &str) -> Self::TokenStream {
        unimplemented!()
    }
    fn to_string(&mut self, _stream: &Self::TokenStream) -> String {
        unimplemented!()
    }
    fn from_token_tree(
        &mut self,
        _tree: TokenTree<Self::Group, Self::Punct, Self::Ident, Self::Literal>,
    ) -> Self::TokenStream {
        unimplemented!()
    }

    fn into_iter(&mut self, _stream: Self::TokenStream) -> Self::TokenStreamIter {
        unimplemented!()
    }
}

impl server::TokenStreamBuilder for Rustc {
    fn new(&mut self) -> Self::TokenStreamBuilder {
        unimplemented!()
    }
    fn push(&mut self, _builder: &mut Self::TokenStreamBuilder, _stream: Self::TokenStream) {
        unimplemented!()
    }
    fn build(&mut self, _builder: Self::TokenStreamBuilder) -> Self::TokenStream {
        unimplemented!()
    }
}

impl server::TokenStreamIter for Rustc {
    fn next(
        &mut self,
        _iter: &mut Self::TokenStreamIter,
    ) -> Option<TokenTree<Self::Group, Self::Punct, Self::Ident, Self::Literal>> {
        unimplemented!()
    }
}

impl server::Group for Rustc {
    fn new(&mut self, _delimiter: Delimiter, _stream: Self::TokenStream) -> Self::Group {
        unimplemented!()
    }
    fn delimiter(&mut self, _group: &Self::Group) -> Delimiter {
        unimplemented!()
    }
    fn stream(&mut self, _group: &Self::Group) -> Self::TokenStream {
        unimplemented!()
    }
    fn span(&mut self, _group: &Self::Group) -> Self::Span {
        unimplemented!()
    }

    fn set_span(&mut self, _group: &mut Self::Group, _span: Self::Span) {
        unimplemented!()
    }

    fn span_open(&mut self, _group: &Self::Group) -> Self::Span {
        unimplemented!()
    }

    fn span_close(&mut self, _group: &Self::Group) -> Self::Span {
        unimplemented!()
    }
}

impl server::Punct for Rustc {
    fn new(&mut self, _ch: char, _spacing: Spacing) -> Self::Punct {
        unimplemented!()
    }

    fn as_char(&mut self, _punct: Self::Punct) -> char {
        unimplemented!()
    }
    fn spacing(&mut self, _punct: Self::Punct) -> Spacing {
        unimplemented!()
    }
    fn span(&mut self, _punct: Self::Punct) -> Self::Span {
        unimplemented!()
    }
    fn with_span(&mut self, _punct: Self::Punct, _span: Self::Span) -> Self::Punct {
        unimplemented!()
    }
}

impl server::Ident for Rustc {
    fn new(&mut self, _string: &str, _span: Self::Span, _is_raw: bool) -> Self::Ident {
        unimplemented!()
    }

    fn span(&mut self, _ident: Self::Ident) -> Self::Span {
        unimplemented!()
    }
    fn with_span(&mut self, _ident: Self::Ident, _span: Self::Span) -> Self::Ident {
        unimplemented!()
    }
}

impl server::Literal for Rustc {
    fn debug(&mut self, _literal: &Self::Literal) -> String {
        unimplemented!()
    }

    fn integer(&mut self, _n: &str) -> Self::Literal {
        unimplemented!()
    }

    fn typed_integer(&mut self, _n: &str, _kind: &str) -> Self::Literal {
        unimplemented!()
    }

    fn float(&mut self, _n: &str) -> Self::Literal {
        unimplemented!()
    }

    fn f32(&mut self, _n: &str) -> Self::Literal {
        unimplemented!()
    }

    fn f64(&mut self, _n: &str) -> Self::Literal {
        unimplemented!()
    }

    fn string(&mut self, _string: &str) -> Self::Literal {
        unimplemented!()
    }

    fn character(&mut self, _ch: char) -> Self::Literal {
        unimplemented!()
    }

    fn byte_string(&mut self, _bytes: &[u8]) -> Self::Literal {
        unimplemented!()
    }

    fn span(&mut self, _literal: &Self::Literal) -> Self::Span {
        unimplemented!()
    }

    fn set_span(&mut self, _literal: &mut Self::Literal, _span: Self::Span) {
        unimplemented!()
    }

    fn subspan(
        &mut self,
        _literal: &Self::Literal,
        _start: Bound<usize>,
        _end: Bound<usize>,
    ) -> Option<Self::Span> {
        unimplemented!()
    }
}

impl server::SourceFile for Rustc {
    fn eq(&mut self, _file1: &Self::SourceFile, _file2: &Self::SourceFile) -> bool {
        unimplemented!()
    }
    fn path(&mut self, _file: &Self::SourceFile) -> String {
        unimplemented!()
    }
    fn is_real(&mut self, _file: &Self::SourceFile) -> bool {
        unimplemented!()
    }
}

impl server::Diagnostic for Rustc {
    fn new(&mut self, _level: Level, _msg: &str, _spans: Self::MultiSpan) -> Self::Diagnostic {
        unimplemented!()
    }

    fn sub(
        &mut self,
        _diag: &mut Self::Diagnostic,
        _level: Level,
        _msg: &str,
        _spans: Self::MultiSpan,
    ) {
        unimplemented!()
    }

    fn emit(&mut self, _diag: Self::Diagnostic) {
        unimplemented!()
    }
}

impl server::Span for Rustc {
    fn debug(&mut self, _span: Self::Span) -> String {
        unimplemented!()
    }

    fn def_site(&mut self) -> Self::Span {
        unimplemented!()
    }

    fn call_site(&mut self) -> Self::Span {
        unimplemented!()
    }

    fn source_file(&mut self, _span: Self::Span) -> Self::SourceFile {
        unimplemented!()
    }

    fn source_text(&mut self, _span: Self::Span) -> Option<String> {
        unimplemented!()
    }

    fn parent(&mut self, _span: Self::Span) -> Option<Self::Span> {
        unimplemented!()
    }
    fn source(&mut self, _span: Self::Span) -> Self::Span {
        unimplemented!()
    }
    fn start(&mut self, _span: Self::Span) -> LineColumn {
        unimplemented!()
    }
    fn end(&mut self, _span: Self::Span) -> LineColumn {
        unimplemented!()
    }
    fn join(&mut self, _first: Self::Span, _second: Self::Span) -> Option<Self::Span> {
        unimplemented!()
    }
    fn resolved_at(&mut self, _span: Self::Span, _at: Self::Span) -> Self::Span {
        unimplemented!()
    }
}

impl server::MultiSpan for Rustc {
    fn new(&mut self) -> Self::MultiSpan {
        unimplemented!();
    }

    fn push(&mut self, _other: &mut Self::MultiSpan, _span: Self::Span) {
        unimplemented!();
    }
}
