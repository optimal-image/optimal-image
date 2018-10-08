#!/usr/bin/env bash

version=${VIPS_VERSION}
tarball=https://github.com/libvips/libvips/releases/download/v${version}/vips-${version}.tar.gz

set -e

if [ -d "$HOME/vips/bin" ]; then
    installed_version=$($HOME/vips/bin/vips --version | awk -F- '{print $2}')
    echo "Version expected: $version"
    echo "Version actual  : $installed_version"

    if [[ "$installed_version" == "$version" ]]; then
        echo "Using cached vips directory"
        exit 0
    fi
fi

echo "Installing vips $version"

rm -rf $HOME/vips
mkdir $HOME/vips

curl -L ${tarball} | tar xz
cd vips-${version}
CXXFLAGS=-D_GLIBCXX_USE_CXX11_ABI=0 ./configure \
    --prefix="$HOME/vips" \
    --disable-debug \
    --disable-static \
    --disable-introspection \
    --disable-dependency-tracking \
    --enable-silent-rules \
    --without-python \
    --without-orc \
    --without-fftw \
    $*

make && make install

cd ../
rm -rf vips-${version}
