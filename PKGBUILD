
pkgname=uninstaller
pkgver=1.0.0
pkgrel=1
pkgdesc="A minimal and modern terminal package uninstaller written in Rust"
arch=('x86_64')
url="https://github.com/Neuwj-00/uninstaller"
license=('GPL3')
depends=('gcc-libs')
makedepends=('cargo')


source=("Cargo.toml"
        "Cargo.lock"
        "src/") 
sha256sums=('SKIP' 'SKIP' 'SKIP')

build() {
 
  cd "$srcdir"
  cargo build --release --locked
}

package() {
  cd "$srcdir"
  install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
