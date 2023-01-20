require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("silverpoint/silverpoint") do |r|
  # r.force_install_rust_toolchain = "nightly"
end
