extern crate proc_macro;
use proc_macro::{
  Delimiter::{Bracket, Parenthesis},
  Group, Literal, Punct,
  Spacing::Alone,
  TokenStream, TokenTree as TT,
};

#[proc_macro]
pub fn patch_source(i: TokenStream) -> TokenStream {
  let mut i = i.into_iter();
  let Some(TT::Literal(lit)) = i.next() else {
    panic!("expected string literal");
  };
  let lit = lit.to_string();
  let Some(lit) = lit.strip_prefix('"') else {
    panic!("expected string literal");
  };
  let Some(mut lit) = lit.strip_suffix('"') else {
    panic!("expected string literal");
  };

  let mut bytes = Vec::with_capacity(256);
  let mut relocs = Vec::with_capacity(16);

  while let Some(&c) = lit.as_bytes().first() {
    match c {
      b' ' | b'\t' | b'\n' | b'\r' => {
        lit = &lit[1..];
      }
      b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
        if let Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') = lit.as_bytes().get(1) {
          bytes.push(u8::from_str_radix(&lit[..2], 16).unwrap());
          lit = &lit[2..];
        } else {
          panic!("incomplete byte value");
        }
      }
      b'$' => {
        relocs.push(bytes.len() as u16);
        lit = &lit[1..];
      }
      _ => panic!("unexpected character `{}`", lit.chars().next().unwrap()),
    }
  }

  if bytes.len() > u16::MAX as usize {
    panic!("input too large")
  }
  if let Some(&reloc) = relocs.last() {
    if bytes.len() < reloc as usize + 4 {
      panic!("incomplete relocation");
    }
  }

  TokenStream::from_iter([TT::Group(Group::new(
    Parenthesis,
    TokenStream::from_iter([
      TT::Punct(Punct::new('&', Alone)),
      TT::Group(Group::new(
        Bracket,
        TokenStream::from_iter(bytes.iter().flat_map(|&x| {
          [
            TT::Literal(Literal::u8_suffixed(x)),
            TT::Punct(Punct::new(',', Alone)),
          ]
        })),
      )),
      TT::Punct(Punct::new(',', Alone)),
      TT::Punct(Punct::new('&', Alone)),
      TT::Group(Group::new(
        Bracket,
        TokenStream::from_iter(relocs.iter().flat_map(|&x| {
          [
            TT::Literal(Literal::u16_suffixed(x)),
            TT::Punct(Punct::new(',', Alone)),
          ]
        })),
      )),
    ]),
  ))])
}
