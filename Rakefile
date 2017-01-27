require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "thermite/tasks"

RSpec::Core::RakeTask.new(:spec)

task :default => :test

Thermite::Tasks.new

desc 'Run Rust & Ruby testsuites'
task test: ['thermite:build', 'thermite:test'] do
  puts "run rspec"
  puts "run cargo test"
end
