use ddmw_util::app;

fn main() {
  let conf = app::load_conf(None);

  println!("{:?}", conf);
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
