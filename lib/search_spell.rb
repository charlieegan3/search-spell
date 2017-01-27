require "search_spell/version"

require "fiddle"

module SearchSpell
  def self.query(term)
    library = Fiddle::dlopen('target/release/libapp.so')
    Fiddle::Function.new(
      library['initialize_speller'],
      [],
      Fiddle::TYPE_VOIDP
    ).call

    Speller.new.query(term)
  end
end
