extern crate proc_macro;
use proc_macro::{
  Delimiter::{Bracket, Parenthesis},
  Group, Literal, Punct,
  Spacing::Alone,
  TokenStream, TokenTree as TT,
};

struct Uint2Collector {
  collected: Vec<u32>,
  next: u32,
  pos: u8,
}
impl Uint2Collector {
  fn new() -> Self {
    Self { collected: Vec::with_capacity(1), next: 0, pos: 0 }
  }

  fn push(&mut self, x: u8) {
    assert!(x < 3);
    self.next |= (x as u32) << self.pos;
    self.pos = if self.pos == 30 {
      self.collected.push(self.next);
      self.next = 0;
      0
    } else {
      self.pos + 2
    };
  }

  fn into_iter(mut self) -> impl Iterator<Item = u32> {
    if self.next != 0 {
      self.collected.push(self.next);
    }
    let idx = self.collected.iter().rposition(|x| *x != 0).map_or(0, |x| x + 1);
    self.collected.into_iter().take(idx)
  }
}

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
  let mut control_stream = Uint2Collector::new();
  let mut cur_reloc = None;

  while let Some(&c) = lit.as_bytes().first() {
    match c {
      b' ' | b'\t' | b'\n' | b'\r' => {
        if let Some(cur_reloc) = cur_reloc.take() {
          assert!(cur_reloc + 4 == bytes.len(), "incorrect relocation size");
        }
        lit = &lit[1..];
      }
      b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
        if let Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') = lit.as_bytes().get(1) {
          if cur_reloc.is_none() {
            control_stream.push(0);
          }
          bytes.push(u8::from_str_radix(&lit[..2], 16).unwrap());
          lit = &lit[2..];
        } else {
          panic!("incomplete byte value");
        }
      }
      b'x' if cur_reloc.is_none() => {
        if let Some(b'x') = lit.as_bytes().get(1) {
          control_stream.push(1);
          bytes.push(0);
          lit = &lit[2..];
        } else {
          panic!("incomplete byte mask");
        }
      }
      b'x' => panic!("masked byte in relocation"),
      b'$' => match cur_reloc {
        Some(_) => panic!("already reading relocation"),
        None => {
          control_stream.push(2);
          cur_reloc = Some(bytes.len());
          lit = &lit[1..];
        }
      },
      _ => panic!("unexpected character `{}`", lit.chars().next().unwrap()),
    }
  }

  if let Some(cur_reloc) = cur_reloc {
    assert!(cur_reloc + 4 == bytes.len(), "incorrect relocation size");
  }

  let hash = bytes.iter().fold(0x01000193u32, |hash, x| {
    (hash ^ *x as u32).wrapping_mul(0x01000193u32)
  });

  TokenStream::from_iter([TT::Group(Group::new(
    Parenthesis,
    TokenStream::from_iter([
      TT::Literal(Literal::u16_suffixed(bytes.len() as u16)),
      TT::Punct(Punct::new(',', Alone)),
      TT::Literal(Literal::u32_suffixed(hash)),
      TT::Punct(Punct::new(',', Alone)),
      TT::Punct(Punct::new('&', Alone)),
      TT::Group(Group::new(
        Bracket,
        TokenStream::from_iter(control_stream.into_iter().flat_map(|x| {
          [
            TT::Literal(Literal::u32_suffixed(x)),
            TT::Punct(Punct::new(',', Alone)),
          ]
        })),
      )),
    ]),
  ))])
}
