# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "search_spell/version"

Gem::Specification.new do |spec|
  spec.name          = "search_spell"
  spec.version       = SearchSpell::VERSION
  spec.authors       = ["Charlie Egan"]
  spec.email         = ["me@charlieegan3.com"]

  spec.summary       = %q{Get search suggestions in parallel with from Ruby with Rust}
  spec.description   = %q{Uses ruru to implement a Ruby speller class to collect results for typos from search engines.}
  spec.homepage      = "https://charlieegan3.com"
  spec.license       = "MIT"

  spec.files         = `git ls-files -z`.split("\x0").reject do |f|
    f.match(%r{^(test|spec|features)/})
  end
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_development_dependency "bundler", "~> 1.14"
  spec.add_development_dependency "rake", "~> 10.0"
  spec.add_development_dependency "rspec", "~> 3.0"

  spec.extensions << 'ext/Rakefile'
  spec.add_runtime_dependency 'thermite', '~> 0'
end
