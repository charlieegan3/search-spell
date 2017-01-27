require "spec_helper"

RSpec.describe SearchSpell do
  it "has a version number" do
    expect(SearchSpell::VERSION).not_to be nil
  end

  it "gets two results from two search engines" do
    expect(SearchSpell.query("misstake")).to eq ["mistake", "mistake"]
  end
end
