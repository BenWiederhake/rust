#![allow(unused)]
#![warn(clippy::missing_fields_in_debug)]

use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

struct NamedStruct1Ignored {
    data: u8,
    hidden: u32,
}

impl fmt::Debug for NamedStruct1Ignored {
    // unused field: hidden
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NamedStruct1Ignored")
            .field("data", &self.data)
            .finish()
    }
}

struct NamedStructMultipleIgnored {
    data: u8,
    hidden: u32,
    hidden2: String,
    hidden3: Vec<Vec<i32>>,
    hidden4: ((((u8), u16), u32), u64),
}

impl fmt::Debug for NamedStructMultipleIgnored {
    // unused fields: hidden, hidden2, hidden4
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NamedStructMultipleIgnored")
            .field("data", &self.data)
            .field("hidden3", &self.hidden3)
            .finish()
    }
}

struct Unit;

// ok
impl fmt::Debug for Unit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Unit").finish()
    }
}

struct UnnamedStruct1Ignored(String);

impl fmt::Debug for UnnamedStruct1Ignored {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("UnnamedStruct1Ignored").finish()
    }
}

struct UnnamedStructMultipleIgnored(String, Vec<u8>, i32);

// tuple structs are not linted
impl fmt::Debug for UnnamedStructMultipleIgnored {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("UnnamedStructMultipleIgnored")
            .field(&self.1)
            .finish()
    }
}

struct NamedStructNonExhaustive {
    a: u8,
    b: String,
}

// ok
impl fmt::Debug for NamedStructNonExhaustive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NamedStructNonExhaustive")
            .field("a", &self.a)
            .finish_non_exhaustive() // should not warn here
    }
}

struct MultiExprDebugImpl {
    a: u8,
    b: String,
}

// ok
impl fmt::Debug for MultiExprDebugImpl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = formatter.debug_struct("MultiExprDebugImpl");
        f.field("a", &self.a);
        f.finish()
    }
}

enum SingleVariantEnumUnnamed {
    A(u8),
}

// ok
impl fmt::Debug for SingleVariantEnumUnnamed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
        }
    }
}

enum MultiVariantEnum {
    A(u8),
    B { a: u8, b: String },
    C,
}

impl fmt::Debug for MultiVariantEnum {
    // match arm Self::B ignores `b`
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
            Self::B { a, b } => formatter.debug_struct("B").field("a", &a).finish(),
            Self::C => formatter.debug_struct("C").finish(),
        }
    }
}

enum MultiVariantEnumOk {
    A(u8),
    B { a: u8, b: String },
    C,
}

// ok
impl fmt::Debug for MultiVariantEnumOk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
            Self::B { a, b } => formatter.debug_struct("B").field("a", &a).field("b", &b).finish(),
            Self::C => formatter.debug_struct("C").finish(),
        }
    }
}

enum MultiVariantEnumNonExhaustive {
    A(u8),
    B { a: u8, b: String },
    C,
}

// ok
impl fmt::Debug for MultiVariantEnumNonExhaustive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
            Self::B { a, b } => formatter.debug_struct("B").field("b", &b).finish_non_exhaustive(),
            Self::C => formatter.debug_struct("C").finish(),
        }
    }
}

enum MultiVariantRest {
    A(u8),
    B { a: u8, b: String },
    C,
}

impl fmt::Debug for MultiVariantRest {
    // `a` field ignored due to rest pattern
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
            Self::B { b, .. } => formatter.debug_struct("B").field("b", &b).finish(),
            Self::C => formatter.debug_struct("C").finish(),
        }
    }
}

enum MultiVariantRestNonExhaustive {
    A(u8),
    B { a: u8, b: String },
    C,
}

// ok
impl fmt::Debug for MultiVariantRestNonExhaustive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
            Self::B { b, .. } => formatter.debug_struct("B").field("b", &b).finish_non_exhaustive(),
            Self::C => formatter.debug_struct("C").finish(),
        }
    }
}

enum Wildcard {
    A(u8),
    B(String),
}

// ok
impl fmt::Debug for Wildcard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(n) => formatter.debug_tuple("A").field(&n).finish(),
            _ => todo!(),
        }
    }
}

enum Empty {}

// ok
impl fmt::Debug for Empty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

#[derive(Debug)]
struct DerivedStruct {
    a: u8,
    b: i32,
}

#[derive(Debug)]
enum DerivedEnum {
    A(i32),
    B { a: String },
}

// https://github.com/rust-lang/rust-clippy/pull/10616#discussion_r1166846953

struct Inner {
    a: usize,
    b: usize,
}

struct HasInner {
    inner: Inner,
}

impl HasInner {
    fn get(&self) -> &Inner {
        &self.inner
    }
}

impl fmt::Debug for HasInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.get();

        f.debug_struct("HasInner")
            .field("a", &inner.a)
            .field("b", &inner.b)
            .finish()
    }
}

// https://github.com/rust-lang/rust-clippy/pull/10616#discussion_r1170306053
struct Foo {
    a: u8,
    b: u8,
}

impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Foo").field("a", &self.a).field("b", &()).finish()
    }
}

// https://github.com/rust-lang/rust-clippy/pull/10616#discussion_r1175473620
mod comment1175473620 {
    use super::*;

    struct Inner {
        a: usize,
        b: usize,
    }
    struct Wrapper(Inner);

    impl Deref for Wrapper {
        type Target = Inner;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl fmt::Debug for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Wrapper")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish()
        }
    }
}

// https://github.com/rust-lang/rust-clippy/pull/10616#discussion_r1175488757
// PhantomData is an exception and does not need to be included
struct WithPD {
    a: u8,
    b: u8,
    c: PhantomData<String>,
}

impl fmt::Debug for WithPD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WithPD")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

fn main() {}
