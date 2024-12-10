non-root auth test when using `.require_auth` inside smart contract `__constructor`.

error log
```chatinput
Event log (newest first):
   0: [Diagnostic Event] topics:[error, Error(Context, InvalidAction)], data:["constructor invocation has failed with error", Error(Auth, InvalidAction)]
   1: [Failed Diagnostic Event (not emitted)] contract:CBRIAA73VOIKPZYM5G3LGPF3NGCFXLR3IW22MKEYJAB3QBOMTUTRCASK, topics:[log], data:["VM call trapped with HostError", __constructor, Error(Auth, InvalidAction)]
   2: [Failed Diagnostic Event (not emitted)] contract:CBRIAA73VOIKPZYM5G3LGPF3NGCFXLR3IW22MKEYJAB3QBOMTUTRCASK, topics:[error, Error(Auth, InvalidAction)], data:"escalating error to VM trap from failed host function call: require_auth"
   3: [Failed Diagnostic Event (not emitted)] contract:CBRIAA73VOIKPZYM5G3LGPF3NGCFXLR3IW22MKEYJAB3QBOMTUTRCASK, topics:[error, Error(Auth, InvalidAction)], data:["[recording authorization only] encountered authorization not tied to the root contract invocation for an address. Use `require_auth()` in the top invocation or enable non-root authorization.", CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM]
   4: [Diagnostic Event] topics:[fn_call, CBRIAA73VOIKPZYM5G3LGPF3NGCFXLR3IW22MKEYJAB3QBOMTUTRCASK, __constructor], data:CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM
```

reproduce:
```bash
make
```

or 
```bash
cargo clean
soroban contract build --package smart-contract
cargo test
```