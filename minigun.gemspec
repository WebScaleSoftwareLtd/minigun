# frozen_string_literal: true

require_relative "lib/minigun/version"

Gem::Specification.new do |spec|
  spec.name = "minigun"
  spec.version = Minigun::VERSION
  spec.authors = ["Astrid Gealer"]
  spec.email = ["astrid@gealer.email"]

  spec.summary = "A HTTP/2 client for Ruby"
  spec.homepage = "https://github.com/webscalesoftwareltd/minigun"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.3.0"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/minigun/extconf.rb"]

  # needed until rubygems supports Rust support is out of beta
  spec.add_dependency "rb_sys", "~> 0.9.39"

  # only needed when developing or packaging your gem
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
