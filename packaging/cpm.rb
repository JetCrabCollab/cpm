class Cpm < Formula
  desc "Crab Package Manager - A modern package manager for JavaScript and Rust"
  homepage "https://github.com/JetCrabCollab/cpm"
  url "https://github.com/JetCrabCollab/cpm/releases/download/v0.4.0/cpm-macos-x86_64.tar.gz"
  sha256 "placeholder-sha256"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/JetCrabCollab/cpm/releases/download/v0.4.0/cpm-macos-aarch64.tar.gz"
    sha256 "placeholder-sha256-arm"
  end

  def install
    bin.install "cpm"
    man1.install "man/cpm.1" if File.exist?("man/cpm.1")
  end

  test do
    system "#{bin}/cpm", "--version"
  end
end
