require "spec_helper"

RSpec.describe RustyRubyThingy do
  it "has a version number" do
    expect(RustyRubyThingy::VERSION).not_to be nil
  end

  it "does something useful" do
    expect(RustyRubyThingy.test_it_out("http://charlieegan3.com")).to include("charlieegan3")
  end
end
