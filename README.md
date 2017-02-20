# telegram-rs
> Unofficial Telegram API Library in Rust

**Work in Progress**

 - [ ] Serialize type in reference to https://core.telegram.org/mtproto/serialize (**in progress**)
    - [ ] `telegram_derive` for `proc_macro_derive` for `Serialize`
 - [ ] Deserialize type in reference to https://core.telegram.org/mtproto/serialize (**in progress**)
    - [ ] `telegram_derive` for `proc_macro_derive` for `Deserialize`
 - [ ] Generate type definitions for type constructors from https://core.telegram.org/schema and https://core.telegram.org/schema/mtproto (**in progress**) (`telegram_codegen`)
    - [x] `telegram_codegen`
    - [ ] Instead of a blanket `::_` for typenames we need to check if we're in a module and do `super::_` or `_`
 - [ ] Generate methods for method definitions from https://core.telegram.org/schema and https://core.telegram.org/schema/mtproto
 - [ ] Build high-level client interface

## License

telegram-rs is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0). 

telegram-rs was written in a "black box", strictly adhering to public documentation provided on https://core.telegram.org/api. At no point was the 
open source Telegram application referenced.

telegram-rs is using the open API provided by Telegram. 

telegram-rs is an unofficial API library and is in no way supported or endorsed by Telegram.

See LICENSE-APACHE and LICENSE-MIT for more details.
