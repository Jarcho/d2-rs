extern crate proc_macro;
use proc_macro::{
  Delimiter::{Bracket, Parenthesis},
  Group, Literal, Punct,
  Spacing::Alone,
  TokenStream, TokenTree as TT,
};
use xxhash_rust::xxh3::xxh3_64;

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
  let mut last_reloc = None;

  while let Some(&c) = lit.as_bytes().first() {
    match c {
      b' ' | b'\t' | b'\n' | b'\r' => {
        if let Some(last_reloc) = last_reloc.take() {
          if last_reloc + 4 != bytes.len() as u16 {
            panic!("incorrect relocation size");
          }
          relocs.push(last_reloc);
        }
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
      b'$' => match last_reloc {
        Some(_) => panic!("already reading relocation"),
        None => {
          last_reloc = Some(bytes.len() as u16);
          lit = &lit[1..];
        }
      },
      _ => panic!("unexpected character `{}`", lit.chars().next().unwrap()),
    }
  }

  if bytes.len() > u16::MAX as usize {
    panic!("input too large")
  }
  if let Some(last_reloc) = last_reloc.take() {
    if last_reloc + 4 != bytes.len() as u16 {
      panic!("incorrect relocation size");
    }
    relocs.push(last_reloc);
  }

  TokenStream::from_iter([TT::Group(Group::new(
    Parenthesis,
    TokenStream::from_iter([
      TT::Literal(Literal::u16_suffixed(bytes.len() as u16)),
      TT::Punct(Punct::new(',', Alone)),
      TT::Literal(Literal::u64_suffixed(xxh3_64(&bytes))),
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
