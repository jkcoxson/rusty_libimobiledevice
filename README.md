# rusty_libimobiledevice
A Rust wrapper and abstraction for [Libimobiledevice](https://github.com/libimobiledevice/libimobiledevice)

## Building
Build or install the following packages:
- libplist
- libusbmuxd
- libimobiledevice
- libimobiledevice-glue
- gnutls
- openssl

If you are on Windows, you will also need:
- unistring
- Iphlpapi
- libintl
- libiconv
- gmp
- libtasn1
- libidn2
- p11-kit
- hogweed
- nettle

Either include these packages in a path where the linker can link them, or place them in ``./override/[arch]``.

**Note:** Package managers do not ship static libraries for some of these (I'm looking at you, libimobiledevice), 
so you will need to build them yourself.

## Usage
Add the crate and path to your cargo.toml, and add either ``static`` or ``dynamic`` to the features list. 
This will determine how the library is linked.

To generate bindings yourself, use the ``pls-generate`` feature. Otherwise, the crate will use pre-generated
bindings.

The crate has a module called ``libimobiledevice`` and ``unsafe_bindings``. ``libimobiledevice`` is a safe wrapper for the C library, and can be used in safe code.
``unsafe_bindings`` will have to be wrapped in a block or with a custom implimentation.

Examples will be provided once this crate has more coverage of the library.
