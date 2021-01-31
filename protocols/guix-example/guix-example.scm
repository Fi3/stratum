(define-module (gnu packages gigi)
  #:use-module (guix licenses)
  #:use-module (guix build-system cargo)
  #:use-module (guix download)
  #:use-module (guix packages)
  #:use-module (gnu packages crates-io)
  #:use-module (gnu packages rust-apps)
  #:use-module (gnu packages crates-graphics)
  #:use-module (gnu packages documentation)
  #:use-module (gnu packages fontutils)
  #:use-module (gnu packages version-control))

(package
  (name "guix-example")
  (version "0.1.0")
  (source
   (origin
     (method url-fetch)
     (uri "./target/package/guix-example-0.1.0.crate")
     (sha256
      (base32 "0g7qvv9fxqd6zbgzs9ysyidg7h0n3srqrfijkynxr8623in97pd3"))))
  (build-system cargo-build-system)
  (arguments
   `(#:cargo-inputs
     (("rust-serde" ,rust-serde-1)
      ("rust-serde-derive" ,rust-serde-derive-1)
      ("rust-serde-json" ,rust-serde-json-1)
      ("rust-cbindgen" ,rust-cbindgen-0.16)
      )
     #:phases
     (modify-phases %standard-phases
           (replace 'install
             (lambda* (#:key inputs outputs skip-build? features #:allow-other-keys)
               (let* ((out (assoc-ref outputs "out")))
                 (mkdir-p out)
             
                 (install-file "./guix-example.h" (string-append out "/include/"))
                 (install-file "./target/release/libguix_example.so" (string-append out "/lib/"))
                 )
               #true))
           )

   ))
  (home-page "..")
  (synopsis "..")
  (description
   "..")
  (license gpl3+))
