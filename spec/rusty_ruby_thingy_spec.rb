require "spec_helper"

RSpec.describe RustyRubyThingy do
  it "has a version number" do
    expect(RustyRubyThingy::VERSION).not_to be nil
  end

  it "does something useful" do
    expect(RustyRubyThingy.test_it_out).to eq "hello"
  end
end