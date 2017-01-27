require "rusty_ruby_thingy/version"

require "fiddle"

module RustyRubyThingy
  def self.test_it_out
    library = Fiddle::dlopen('target/release/libapp.so')
    Fiddle::Function.new(library['initialize_thing'], [], Fiddle::TYPE_VOIDP).call

    Thing.new.say_hello
  end
end
