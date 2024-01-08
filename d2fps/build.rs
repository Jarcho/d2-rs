fn main() {
  embed_resource::compile("d2fps.rc", embed_resource::NONE);
  println!("cargo:rerun-if-changed=d2fps.rc");
}
