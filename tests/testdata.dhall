let inter = < none | pahi | cwa >

let test =
      { Type = { interpreter : inter, exitsWith : Natural, fname : Text }
      , default = { interpreter = inter.none, exitsWith = 1, fname = "invalid" }
      }

let home = env:HOME as Text ? "/"

let sslRootLocation =
      env:NIX_SSL_CERT_FILE as Text ? "/etc/ssl/certs/ca-certificates.crt"

in  { cases =
      [ test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "allyourlogs.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "allyourargs.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "allyourargs.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "allyourlogs.wasm"
        }
      , test::{ interpreter = inter.cwa, exitsWith = 0, fname = "shaman.wasm" }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "olinfetch.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "olin-tests.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "olin-tests.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "cpustrain.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/allyourargs.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/allyourbase.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/allyourlogs.wasm"
        }
      , test::{ interpreter = inter.cwa, exitsWith = 0, fname = "zig/cat.wasm" }
      , test::{ interpreter = inter.cwa, exitsWith = 0, fname = "zig/coi.wasm" }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/exit0.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 1
        , fname = "zig/exit1.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/httptest.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/runtime_name.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/shaman.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "zig/triangle.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/allyourargs.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/allyourbase.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/allyourlogs.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/cat.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/coi.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/exit0.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 1
        , fname = "zig/exit1.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/httptest.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/runtime_name.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/shaman.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "zig/triangle.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "magic_conch.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "magic_conch.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "blake2stress.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "blake2stress.wasm"
        }
      , test::{ interpreter = inter.cwa, exitsWith = 0, fname = "fibber.wasm" }
      , test::{ interpreter = inter.pahi, exitsWith = 0, fname = "fibber.wasm" }
      , test::{ interpreter = inter.cwa, exitsWith = 0, fname = "bigjson.wasm" }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "bigjson.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "k8sparse.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "k8sparse.wasm"
        }
      ]
    , env =
      [ "MAGIC_CONCH=yes"
      , "OTHER_VAL=no"
      , "HOME=${home}"
      , "NIX_SSL_CERT_FILE=${sslRootLocation}"
      ]
    }
