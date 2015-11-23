<a name="0.7.3"></a>
### 0.7.3 (2015-11-23)


#### Bug Fixes

* **Cargo:**  remove wildcard dependency versions ([ded09d3b](https://github.com/nickel-org/nickel.rs/commit/ded09d3b10304832e392a98724934b5f35f57119))



<a name="0.7.2"></a>
### 0.7.2 (2015-11-18)


#### Performance

* **hyper:**  re-enable keep-alive by default ([dfcb3baf](https://github.com/nickel-org/nickel.rs/commit/dfcb3baf7fb046ec7bfe14186ddb4a758bbb381c))

#### Bug Fixes

* **static_files:**  deny suspicious paths ([bbba1ebf](https://github.com/nickel-org/nickel.rs/commit/bbba1ebf6eaa81e325edb9fb46b20d25d0ad52ed))



<a name="0.7.1"></a>
### 0.7.1 (2015-11-02)


#### Bug Fixes

* ***:**  add default typeparam for MiddlewareResult ([d135b4e9](https://github.com/nickel-org/nickel.rs/commit/d135b4e9a254e8a4722a10bad7c99121cf0c7dc2))
* **send_file:**  don't overwrite the ContentType header if it exists ([8eb8c5fd](https://github.com/nickel-org/nickel.rs/commit/8eb8c5fd92d18f3455e1fcac426501aabba64f42), closes [#285](https://github.com/nickel-org/nickel.rs/issues/285))
* **serverdata:**  extend lifetime of return value of {Request|Response}::server_data ([eff2ac7c](https://github.com/nickel-org/nickel.rs/commit/eff2ac7c46a71a40aceb1f444fa9309ef2ec692a))

#### Features

* **request:**  add extension to get Request::referer ([38b21cb4](https://github.com/nickel-org/nickel.rs/commit/38b21cb4f98af367ab99dac940b8420298e31e71))
* **response:**
  *  add next_middleware method for convenience ([32554795](https://github.com/nickel-org/nickel.rs/commit/3255479526621754f344d1ba6b8d422bb95794fa))
  *  add extension for simple redirects ([d8629ad2](https://github.com/nickel-org/nickel.rs/commit/d8629ad220a93e14993b2fb4ec251d96d2e2a532))



<a name="0.7.0"></a>
## 0.7.0 (2015-09-15)

#### BREAKING CHANGES

See [8f411c9d](https://github.com/nickel-org/nickel.rs/commit/8f411c9d8c437ec7108893daad65eaf3dcf241e9), [2dd98e8c](https://github.com/nickel-org/nickel.rs/commit/2dd98e8c24922c54a7b811dd5877460ccf4433b8), [d4a6d815](https://github.com/nickel-org/nickel.rs/commit/d4a6d815351b7c27b88efb68aace16fc3b3b8578) for more information.

#### Bug Fixes

* **lifetimes:**  correct lifetime pattern to separate out server state from mw state ([d4a6d815](https://github.com/nickel-org/nickel.rs/commit/d4a6d815351b7c27b88efb68aace16fc3b3b8578))
* **middleware:**  satisfy lifetime requirements for RFC 1214 ([fc35544c](https://github.com/nickel-org/nickel.rs/commit/fc35544ccc5f672285e77ef731df520498994126))
* **travis:**  resume publishing docs ([34a08aed](https://github.com/nickel-org/nickel.rs/commit/34a08aedfc1ce8740084db9487c8ad51bce3155a))

#### Features

* **macros:**  allow hinting the server data type in middleware macro ([dda02d65](https://github.com/nickel-org/nickel.rs/commit/dda02d654ca77d60033c1a8462425ee66a9e43f6))
* **nickel:**  Add Mount middleware. ([0f258361](https://github.com/nickel-org/nickel.rs/commit/0f258361b0ca670321e1f621189a716c94633773))
* **render:**  improve ergonomics of send_file ([55d97f4e](https://github.com/nickel-org/nickel.rs/commit/55d97f4e2736714ce9a90760981342c240b5f182))
* **response:**  allow Plugins for Response ([ffb5f212](https://github.com/nickel-org/nickel.rs/commit/ffb5f21293a51dc29a3f7109560931ca810afe7e))
* **router:**  add chainable routes ([2dd98e8c](https://github.com/nickel-org/nickel.rs/commit/2dd98e8c24922c54a7b811dd5877460ccf4433b8))
* **server:**  add some shared data across all requests ([8f411c9d](https://github.com/nickel-org/nickel.rs/commit/8f411c9d8c437ec7108893daad65eaf3dcf241e9))
* **ssl:**  make Hyper's SSL feature optional ([ea2081a7](https://github.com/nickel-org/nickel.rs/commit/ea2081a79877022186012d5212f9b78ac4a2f451))



<a name="0.6.0"></a>
## 0.6.0 (2015-07-16)


#### Bug Fixes

* **travis**  allow failures on travis nightly builds ([a5a32efc](https://github.com/nickel-org/nickel.rs/commit/a5a32efcc31e874018a73bb3b91e8d578f851023))
* **request**  request.param(_) now returns an Option ([f218ce1a](https://github.com/nickel-org/nickel.rs/commit/f218ce1a44bed27172b936d0dfdd230db7d184cd))
* **responder**  relax required lifetime for responding with string slices ([b220a6f3](https://github.com/nickel-org/nickel.rs/commit/b220a6f3c04f09357955a5d405c74a62651f20e0))
* **static_files**  ignore the querystring during static file lookup ([88f9632e](https://github.com/nickel-org/nickel.rs/commit/88f9632e373b64761461e255a32d0aff2f1e008c))



<a name="0.5.0"></a>
## 0.5.0 (2015-05-20)


#### Features

* **middleware**  impl Responder for StatusCode ([2797359e](https://github.com/nickel-org/nickel.rs/commit/2797359e89421a9e5649325f092644bc216623ab))
* **response**
  *  allow `Response::set` to be used with MediaType ([24e218a0](https://github.com/nickel-org/nickel.rs/commit/24e218a07c5067d2f596abf779851ba280bcf5c8))
  *  add generic `set` method ([182517b8](https://github.com/nickel-org/nickel.rs/commit/182517b89a232a715b2fde82efbe507cef96037e))
  *  add `headers` to access current Response headers ([41b67346](https://github.com/nickel-org/nickel.rs/commit/41b67346a5a54fbbdbbd3b2f734952a34cd9c214))
* **macro**
  *  add basic nickel_try! macro ([a9705ad3](https://github.com/nickel-org/nickel.rs/commit/a9705ad388c5f9dc9b7b454989d67f99bbca708e))
  *  allow _ pattern for request in macro parameters ([227d0828](https://github.com/nickel-org/nickel.rs/commit/227d0828fd500d157caecedc67b2d9a9b1b775a3))
  *  stop ignoring unused macro parameters ([d4e2420e](https://github.com/nickel-org/nickel.rs/commit/d4e2420e82161fb2a4c21fa11e12def36b979f19))
  *  make router! a recursive macro to increase flexibility ([ce218afd](https://github.com/nickel-org/nickel.rs/commit/ce218afdb84d5fe68ded56636c8029068f4f8f4d))
* **Response**  Response::send can now take anything implementing ResponseFinalizer ([62a118b1](https://github.com/nickel-org/nickel.rs/commit/62a118b16023d2c06fe3a42b5ef1c35991b57070))
* **hyper**  update to hyper 0.4 ([f35f7f06](https://github.com/nickel-org/nickel.rs/commit/f35f7f061cb3bbae7604975fc4ced7841de296c1))
* **router**  add convenience methods for OPTIONS and PATCH ([471dc1bc](https://github.com/nickel-org/nickel.rs/commit/471dc1bc5f367a83bc77950402f04df69982f90a), closes [#202](https://github.com/nickel-org/nickel.rs/issues/202))
* **travis**  test the README example in travis builds ([c5986a11](https://github.com/nickel-org/nickel.rs/commit/c5986a11797b0696bf3d47f3afc24714af1dfec4))



<a name="v0.4.0"></a>
## v0.4.0 (2015-05-16)


#### Bug Fixes

* **response**
  *  fix template partials ([bd39ca50](https://github.com/nickel-org/nickel.rs/commit/bd39ca50eb00369a28acfc1fd92565b817a334ef), closes [#160](https://github.com/nickel-org/nickel.rs/issues/160))
  *  loosen bounds for render path ([ffe1a0a3](https://github.com/nickel-org/nickel.rs/commit/ffe1a0a364c1b1afbf0632d9f3d4aa5716563919), closes [#209](https://github.com/nickel-org/nickel.rs/issues/209))
* **server**  loosen bounds required for `listen` ([1f30e68a](https://github.com/nickel-org/nickel.rs/commit/1f30e68a941266a9bcae2b71e0923b8ab500f625))

#### Features

* **travis**  test the README example in travis builds ([c5986a11](https://github.com/nickel-org/nickel.rs/commit/c5986a11797b0696bf3d47f3afc24714af1dfec4))
* **router**  add convenience methods for OPTIONS and PATCH ([471dc1bc](https://github.com/nickel-org/nickel.rs/commit/471dc1bc5f367a83bc77950402f04df69982f90a), closes [#202](https://github.com/nickel-org/nickel.rs/issues/202))



<a name="v0.3.0"></a>
## v0.3.0 (2015-05-01)


#### Bug Fixes

* **query**  allow multiple query params alive at once ([1e453409](https://github.com/nickel-org/nickel.rs/commit/1e453409ffbefddcb6015fb47ab388c7bc21c698))
* **json**  return descriptive errors on failures in json_as ([b4309bec](https://github.com/nickel-org/nickel.rs/commit/b4309beceb2d9abdb255665fcf95d7d923183e0b))
* **nightly**  add nightly feature flag ([4bcd44aa](https://github.com/nickel-org/nickel.rs/commit/4bcd44aaa558751d070c1d598963362abbea56f4))
* **macros**  allow middleware! to bind response mutably ([136d4557](https://github.com/nickel-org/nickel.rs/commit/136d4557d26d40d2a2231effa72a739b73e94c27))
* **server**  ensure panic on invalid listen addr ([e218287c](https://github.com/nickel-org/nickel.rs/commit/e218287c96064ee7af4e5f899ae186b048696048))
* **travis**  update for breaking change to travis-cargo ([7cb48149](https://github.com/nickel-org/nickel.rs/commit/7cb48149ae1b6ce46654f467f3f6644913b8b531))
* **middleware**  fix the default handler so that custom error handlers are used ([45036f75](https://github.com/nickel-org/nickel.rs/commit/45036f75f23c1760c9f7c2f3bae71cfb40c04b56))

#### Features

* **unboxed_closures**  allow unboxed closures via middleware macro ([c635813a](https://github.com/nickel-org/nickel.rs/commit/c635813aad9ec3016171b936f694f46dcb725ef6))
* **travis**  test beta and nightly on travis ([efcb1604](https://github.com/nickel-org/nickel.rs/commit/efcb1604f0e58711f60713916e929c34cb7c4368))



<a name="0.2.1"></a>
## 0.2.1 (2015-04-13)


#### Bug Fixes

* **docs**  fix doc generation script ([cee6a43a](https://github.com/nickel-org/nickel.rs/commit/cee6a43a9de3f2b18616f3f6bb0e365b19e68516))
* **rustup**  adjust for splitn change ([9a4bd5fe](https://github.com/nickel-org/nickel.rs/commit/9a4bd5fea96f13acae6c1ca690e2bb541f588d7f))
* **macros**  remove nickel_macros crate ([60ad3e2f](https://github.com/nickel-org/nickel.rs/commit/60ad3e2f06c9a5e885f547b1e3186c630648c750))



<a name="0.2.0"></a>
## 0.2.0 (2015-04-12)


#### Bug Fixes

* **server**  adjust to upstream changes ([f97f6f43](https://github.com/nickel-org/nickel.rs/commit/f97f6f43be2da88c0af8e361b44b1b3d9a7c318c))
* **query_string**
  *  Update to work with latest rust-plugin. ([b07efe94](https://github.com/nickel-org/nickel.rs/commit/b07efe94974a23aec1c50e0a8f158ee14612cec8))
  *  improve error message for missing querystore ([8bef3153](https://github.com/nickel-org/nickel.rs/commit/8bef315302f2d5ed9a107c22896bebd72e61a843))
* **send_file**  dont deadlock when send_file has a bad path ([68bf0c41](https://github.com/nickel-org/nickel.rs/commit/68bf0c410b7aa58f43d07a8e3ce3a431aa8dd181))
* **middleware_handler**  use from_usize instead of from_uint ([fc30f617](https://github.com/nickel-org/nickel.rs/commit/fc30f6174294324f9045e80131e971b0ab2f7331))
* **example**
  *  return proper 404 for unmatched routes ([37a851b9](https://github.com/nickel-org/nickel.rs/commit/37a851b955d310d1532d40f494692da16d5b9c62))
  *  use real example path for static files ([9cca213a](https://github.com/nickel-org/nickel.rs/commit/9cca213a45c6ad2e4aa975b61d0c855dc231d87e))
  *  remove binary file ([1f9174d9](https://github.com/nickel-org/nickel.rs/commit/1f9174d9d2980509d6b841171656986cf2063fbf))
* **Cargo**  Temporarily switch to Simons fork of rust-mustache. ([012bfbbd](https://github.com/nickel-org/nickel.rs/commit/012bfbbd885b63e2490b01ff9879d1beedc81157))
* **Readme.md**
  *  remove left over comment ([fdedb4f7](https://github.com/nickel-org/nickel.rs/commit/fdedb4f742b930e00df1d535a31982b35b320785))
  *  remove left over comment from original source ;-) ([cee94ac8](https://github.com/nickel-org/nickel.rs/commit/cee94ac81a518b4b58973c905c870b0cd7842d19))
* **response**
  *  set content-type to text/html if unset ([75357b4c](https://github.com/nickel-org/nickel.rs/commit/75357b4c082917f160a9a3560632dcb088cbf31d))
  *  use feature std_misc for Entry::{Occupied, Vacant} ([f0d1e2b2](https://github.com/nickel-org/nickel.rs/commit/f0d1e2b2e8d24b829b0dc5e53077dcdcf81c3689))
  *  Remove warning by not shadowing lifetime. ([197573e8](https://github.com/nickel-org/nickel.rs/commit/197573e8a9140cd5b6e3e0339f93e5469c5a1bc2))
  *  Solve deadlock ([15ae83f6](https://github.com/nickel-org/nickel.rs/commit/15ae83f622c29e5f4d1f3921f7dd64c6cc309faf))
  *  adjust to new lifetime rules ([1c7b20f1](https://github.com/nickel-org/nickel.rs/commit/1c7b20f16d01274c112ed43e2324fd6d53e587bb))
* **Copy**  Copy is now opt-in ([f766b9d1](https://github.com/nickel-org/nickel.rs/commit/f766b9d134633f2e23056d72e1debecdae8a2de0))
* **BytesContainer**  introduce AsBytes to replace BytesContainer ([8ccfec01](https://github.com/nickel-org/nickel.rs/commit/8ccfec014800592fe57f9a97edf054ddbc8554f0))
* **TreeMap**  fix TreeMap path ([d2b3d199](https://github.com/nickel-org/nickel.rs/commit/d2b3d199bdac390a30d9b31d082d76daeef95436))
* **macros**  remove warnings from lint name change ([1b0f5dff](https://github.com/nickel-org/nickel.rs/commit/1b0f5dff3616a3be5b07113fa138db3a78622fdd))
* **url**  parse_str(s) is now parse(s.as_bytes()) ([1afabe59](https://github.com/nickel-org/nickel.rs/commit/1afabe59674df2a602a417633ab96b85b73c4b0e))
* **floor.rs**  fixes typo ([8c2b387b](https://github.com/nickel-org/nickel.rs/commit/8c2b387ba469076e028950eec3ba1cc4ac67b208))
* **routes**  allow custom :format variable in route ([add8301f](https://github.com/nickel-org/nickel.rs/commit/add8301f26b65d309bf143cb2bc18d618ffc3da5))
* **SendStr**  MaybeOwned is depreciated in favor of Cow pointers ([a8481f59](https://github.com/nickel-org/nickel.rs/commit/a8481f596b788d8515664cba32e9584ef3eac302))
* **clone**  Copy now requires Clone ([81af6d79](https://github.com/nickel-org/nickel.rs/commit/81af6d790e37c0d55e14b6051c14b3b7e29d6c73))
* **HashMap**
  *  remove warnings ([fc93f5a0](https://github.com/nickel-org/nickel.rs/commit/fc93f5a0eee4ffa327c1d359f4f8202e92c6151a))
  *  fix HashMap path ([33cc43d1](https://github.com/nickel-org/nickel.rs/commit/33cc43d13e4dea6148d44533a8bb20b511d35479))
* **slicing_syntax**  allow slicing_syntax. fixes #95 ([c1bb3e17](https://github.com/nickel-org/nickel.rs/commit/c1bb3e17306c8d499c9657020254b4ae9abe7f0a))
* **docs**
  *  fix doc generation script ([cee6a43a](https://github.com/nickel-org/nickel.rs/commit/cee6a43a9de3f2b18616f3f6bb0e365b19e68516))
  *  don't run tests for examples ([ad090877](https://github.com/nickel-org/nickel.rs/commit/ad090877966aea089e4a4570222f032fffbb5f1d))
* **Makefile**
  *  remove Makefile in favor of Cargo ([a771ed8f](https://github.com/nickel-org/nickel.rs/commit/a771ed8f6975f07caf118676d9db7d08723fd727))
  *  update the `run` target ([780b991c](https://github.com/nickel-org/nickel.rs/commit/780b991c6144fbd5170e3b6c2c60059158c97301))
  *  adjust to path changes in rust-http ([5afcea33](https://github.com/nickel-org/nickel.rs/commit/5afcea3379856bb0c203e4e18fb87a183e3c4630))
* **mustache**  temporally switch to fork ([c5a7cc4b](https://github.com/nickel-org/nickel.rs/commit/c5a7cc4bc6c43ac8c8d6d7944f0a6b3c3ae8804d))
* **doctest**  compile doctest for listen but dont run it ([f811ff78](https://github.com/nickel-org/nickel.rs/commit/f811ff78bbb8fb4d634101439a7de493890c04a2))
* **plugin**  update plugin syntax ([8c2e6a9b](https://github.com/nickel-org/nickel.rs/commit/8c2e6a9bdbce0a3ec53d559b86f5060ee1ea6c29))
* **json_body_parser**
  *  Update to work with latest rust-plugin. ([2fe55fad](https://github.com/nickel-org/nickel.rs/commit/2fe55fad0e9e28f139d269932ff5d3fc842c371a))
  *  make request body parsing work with the latest rust-http ([a37f5806](https://github.com/nickel-org/nickel.rs/commit/a37f58064af89edeb3084b4771019224e094582c))
* **lib.rs**
  *  expose Middleware trait ([77d45cff](https://github.com/nickel-org/nickel.rs/commit/77d45cffd6fdf129d75a1efaea49ee83c7695af2), closes [#44](https://github.com/nickel-org/nickel.rs/issues/44))
  *  fixes typo ([1d7b18b0](https://github.com/nickel-org/nickel.rs/commit/1d7b18b020d5da457f1273973516cb3c1eecbb67))
* **beta**  remove final unstable code ([fcc84c65](https://github.com/nickel-org/nickel.rs/commit/fcc84c65683cba76c02432d734b00b1eed87d1ac))
* **nickel**
  *  handle inference bug ([17c42a27](https://github.com/nickel-org/nickel.rs/commit/17c42a2727b223d0224145959b7f1a8bca55b84b))
  *  don't force default_router to be last middleware ([0037d54f](https://github.com/nickel-org/nickel.rs/commit/0037d54f9e9e5fd12f4e949cceb16b574d728157))
* **RequestUri**  adjust for upstream namespacing changes ([9ab86848](https://github.com/nickel-org/nickel.rs/commit/9ab86848af6801edf93b8dedf170dbfd52fbd4eb))
* **router**
  *  reintroduce PR #91 ([4e0b37d1](https://github.com/nickel-org/nickel.rs/commit/4e0b37d18e77186e05893e7d8c54c2a50ea13472))
  *  Use new find_equiv signature. ([20a40959](https://github.com/nickel-org/nickel.rs/commit/20a409594ca4b4f015ef1a3d24f19e960e2bfb71))
  *  handle changes to trait resolution ([6c7a4145](https://github.com/nickel-org/nickel.rs/commit/6c7a4145f23ee219d43b72f429bc6105488a96e9))
  *  don't match /barr for /bar routes ([ccfd3846](https://github.com/nickel-org/nickel.rs/commit/ccfd384697e730905f458d3bb6973d4014d75488), closes [#60](https://github.com/nickel-org/nickel.rs/issues/60))
  *  remove superflous line ([1a5d7e68](https://github.com/nickel-org/nickel.rs/commit/1a5d7e68cc50140d21efb8432ae0d3ab46f3251d))
  *  fixes screwed up indentation ([1096b171](https://github.com/nickel-org/nickel.rs/commit/1096b17129be3470833728a910855e18f565f231))
  *  remove accidentially added comment ([9a3844d5](https://github.com/nickel-org/nickel.rs/commit/9a3844d501db7059d1394b196e20508b25953a91))
  *  ignore request params ([9c1527f9](https://github.com/nickel-org/nickel.rs/commit/9c1527f9489153b81686c44066fa00d228d5b925))
  *  fix test ([b63b776d](https://github.com/nickel-org/nickel.rs/commit/b63b776dc94248be988a51ec28452ed23bccb766))
* **rusthttp**  temporary upstream repo change ([9e3da438](https://github.com/nickel-org/nickel.rs/commit/9e3da438c3753775ff1a52ecbe35c417dbc77411))
* *****
  *  fix depreciated code ([bdae4e7e](https://github.com/nickel-org/nickel.rs/commit/bdae4e7e1d4e4fbc9cca3d0dfc09451c0e85af73))
  *  std::path is now std::old_path ([2a8777d5](https://github.com/nickel-org/nickel.rs/commit/2a8777d522ff500f3beb7d67b47197a986e3ba45))
  *  FromStr now returns a Result, not an Option. ([ce33ef2b](https://github.com/nickel-org/nickel.rs/commit/ce33ef2ba22c3278caa130fb4523ff7639ccca6c))
  *  Show is deprecated in favor of Debug ([b758c514](https://github.com/nickel-org/nickel.rs/commit/b758c51495a305e11b2942f01378990e20362a07))
  *  IO module was renamed. ([41787233](https://github.com/nickel-org/nickel.rs/commit/41787233b27a18d04d7e09b54d473fd32c78f078))
  *  change according to latest rust ([ae2c49e6](https://github.com/nickel-org/nickel.rs/commit/ae2c49e61a0f3754c0042fb84ef4bcf43140e975))
  *  don't force StaticFileHandler to be last middleware ([448a8ec1](https://github.com/nickel-org/nickel.rs/commit/448a8ec178b400431e9ce0cbe3d9902b9ac6be97), closes [#61](https://github.com/nickel-org/nickel.rs/issues/61))
  *  change according to latest rust update ([759b8467](https://github.com/nickel-org/nickel.rs/commit/759b84672a43ab125add6acdab2b30efc0960473))
* **HttpDate**  mismatch types, from time::Tm to HttpDate ([c6bc3f45](https://github.com/nickel-org/nickel.rs/commit/c6bc3f450b9ded6dd0fce54fd5b8139f1bc82fd5))
* **readme**
  *  simple parameter access in feature list ([75e88467](https://github.com/nickel-org/nickel.rs/commit/75e88467182cd78c35c1252d510136d32ad82c13))
  *  use nickel.rs as the website link ([7cf57fbf](https://github.com/nickel-org/nickel.rs/commit/7cf57fbf13ecb3277c9ffd52bede2642c3bf422d))
* **mimes**
  *  adjust for enum namespacing changes ([f335216b](https://github.com/nickel-org/nickel.rs/commit/f335216b8902475da455fdeb217801120041e5d4))
  *  Correct Hal mime type ([c2c304ea](https://github.com/nickel-org/nickel.rs/commit/c2c304ea71f3da583858753af54682ae6e3a143c))
* **enums**  Add pub use/namespacing where required ([ed2a3a31](https://github.com/nickel-org/nickel.rs/commit/ed2a3a31156fa118305b76875b53ada722c637d2))
* **mime**  set Content-Type header when appropriate ([dc525433](https://github.com/nickel-org/nickel.rs/commit/dc525433d950f72eca5eca70e9cfacede39197ea))
* **Readme**
  *  fix travis badge ([2f5a6321](https://github.com/nickel-org/nickel.rs/commit/2f5a6321edbd96ea3d2cb94216a8f6b70ae33067))
  *  remove superflous extern from example ([4b932458](https://github.com/nickel-org/nickel.rs/commit/4b9324580fc95dfafb33e22f4910ce8a6af71261))
  *  Floor -> floor ([9b2dde9b](https://github.com/nickel-org/nickel.rs/commit/9b2dde9b1202dda112826d184dd8d3d531295ff5))
  *  rename Floor to floor ([0b29259e](https://github.com/nickel-org/nickel.rs/commit/0b29259e1e88a4e9ec41c21a865895fb52d884f5))
  *  corrected URL in comment ([bf7d56e4](https://github.com/nickel-org/nickel.rs/commit/bf7d56e4cf429b83c3370774b57ded16ff48c468))
  *  fixes outdated example URL ([2a53d54a](https://github.com/nickel-org/nickel.rs/commit/2a53d54a62fc02961a18fc897818c6ca71c320f8))
* **request_handler**  replace fail! with panic! ([e032380c](https://github.com/nickel-org/nickel.rs/commit/e032380c50d3766849d9ffaf289b88e2b59cda18))
* **examples**  AtomicUint is deprecated in favor of AtomicUsize ([03d7fa14](https://github.com/nickel-org/nickel.rs/commit/03d7fa14b0a31aa5e0c975eba765225078f78a79))
* **main.rs**  kill unused import ([445bbbdf](https://github.com/nickel-org/nickel.rs/commit/445bbbdf55a3813fafe59977c0cb4d7829938886))
* **Cargo.toml**  remove invalid keyword ([f810f54e](https://github.com/nickel-org/nickel.rs/commit/f810f54ec7b713b3912871c113c886b61399607e))
* **rustup**
  *  Coherence and Error changes ([dc4d943a](https://github.com/nickel-org/nickel.rs/commit/dc4d943a6707549535204669dc8333d088f22cce))
  *  remove warnings from trivial_casts for fn pointers ([268bcadd](https://github.com/nickel-org/nickel.rs/commit/268bcadde3106fc76d5b5f4506b36c6b32bbade7))
  *  forced feature gates and generic conversions landed ([aa36bd67](https://github.com/nickel-org/nickel.rs/commit/aa36bd670348e1233f6df24d984def162bcf2df2))
  *  range is depreciated ([8fb6bf31](https://github.com/nickel-org/nickel.rs/commit/8fb6bf317613592748935fcb762b002299ef73e4))
  *  some depreciations and hyper changes ([a0e65e98](https://github.com/nickel-org/nickel.rs/commit/a0e65e98a364b4605b66114314be343d6a856bc3))
* **anymap**
  *  remove anymap from Cargo.toml ([e95b0a9d](https://github.com/nickel-org/nickel.rs/commit/e95b0a9d3ed61a79a646d6a45497b7a33eb17c8b))
  *  temporary switch to unofficial anymap ([8d4cc07e](https://github.com/nickel-org/nickel.rs/commit/8d4cc07e9060861a47ef4254d6c4f2acdadc82f5), closes [#66](https://github.com/nickel-org/nickel.rs/issues/66))
* **middleware**
  *  add explicit static lifetimes ([fb208eea](https://github.com/nickel-org/nickel.rs/commit/fb208eead77e80809fdd9c9bb4a48e2cb1cd4f4e))
  *  use rev() rather than shifting handlers ([14be6308](https://github.com/nickel-org/nickel.rs/commit/14be6308ca2d296cfe3673c3bacceb3aa77764e6))
* **StaticFilesHandler**  don't try to send dirs ([6eda5d9c](https://github.com/nickel-org/nickel.rs/commit/6eda5d9c070e7dfd3c3894610e1df07edc530211))
* **tests**  make compiler happy about unused variables ([8023571c](https://github.com/nickel-org/nickel.rs/commit/8023571c89333019fe22864ede026b792aaf9300))

#### Features

* **Readme.me**  added basic notes ([7691dc6d](https://github.com/nickel-org/nickel.rs/commit/7691dc6d352916f9ac4996b5f41387ce27d1b865))
* **routes**  add implicit optional format param to routes ([153179d5](https://github.com/nickel-org/nickel.rs/commit/153179d58e4b122f1e95d6d5809fc9841c9d9502))
* **Cargo.toml**  adjust for crates.io live go ([c8e8519c](https://github.com/nickel-org/nickel.rs/commit/c8e8519c24011818e6d30e9b840a228daa3d56ea))
* **macros**  add convenience macro for establishing routes ([2ac2587d](https://github.com/nickel-org/nickel.rs/commit/2ac2587d5fdbecfb9dfe5cf8e454cc5632589ce9))
* *****
  *  Allow usage of default router ([466d512d](https://github.com/nickel-org/nickel.rs/commit/466d512de875992938bda26f388d53e74b358065), closes [#59](https://github.com/nickel-org/nickel.rs/issues/59))
  *  make router a middleware ([c5ca1b40](https://github.com/nickel-org/nickel.rs/commit/c5ca1b400ec260b9173f106f4db476e3d0f777bc))
  *  adds error handling support ([7183cbcf](https://github.com/nickel-org/nickel.rs/commit/7183cbcf99af0557fe17787aeecc48cfbce39d02), closes [#48](https://github.com/nickel-org/nickel.rs/issues/48))
  *  added cargo support ([57067243](https://github.com/nickel-org/nickel.rs/commit/5706724331c204b4f58758ee560b2323c0b58f02))
* **travis**
  *  use container builds for faster travis tests ([d4cc3128](https://github.com/nickel-org/nickel.rs/commit/d4cc3128e886eff95552259e3bb0743cb4839be9))
  *  add integration ([87d9ec85](https://github.com/nickel-org/nickel.rs/commit/87d9ec8575f421dc84b472b31cdbf9afe681794e))
* **response**
  *  implement chaining status_code API ([6609f444](https://github.com/nickel-org/nickel.rs/commit/6609f4441c3db54e75d00ccded89ba2bee4ecc2b))
  *  implement chaining API for content_type ([c7569bb3](https://github.com/nickel-org/nickel.rs/commit/c7569bb3a4327f722ab79cd357924a7d500e9ea8), closes [#45](https://github.com/nickel-org/nickel.rs/issues/45), [#41](https://github.com/nickel-org/nickel.rs/issues/41))
  *  add mustache support ([3754e89f](https://github.com/nickel-org/nickel.rs/commit/3754e89f2cb89323b972cbcd9f7601d9c731aade))
  *  use BytesContainer for send ([109bcfe9](https://github.com/nickel-org/nickel.rs/commit/109bcfe9082cf333ba25ad8b3870b6a49d673115))
  *  allow setting content type ([a321188d](https://github.com/nickel-org/nickel.rs/commit/a321188d922daac5fb1763acd4b29dbd9a91522f))
* **Readme.md**  explain more detailed what the project is ([737bf965](https://github.com/nickel-org/nickel.rs/commit/737bf9653a8ae075a68bc92b7b85036518c31c38))
* **router**
  *  allow Regex paths ([9fd6e06b](https://github.com/nickel-org/nickel.rs/commit/9fd6e06b0e934d355dcd944521039619ea9f7358))
  *  implement POST, PUT and DELETE ([0595dc10](https://github.com/nickel-org/nickel.rs/commit/0595dc10caff43d18d833daa4f5a213f0e4393d3))
  *  adds wildcard route matching ([22a45670](https://github.com/nickel-org/nickel.rs/commit/22a45670a18592bbbf30b2805f44e076eaa07d45), closes [#5](https://github.com/nickel-org/nickel.rs/issues/5))
* **urlencoded**  add query string middleware ([6de1d279](https://github.com/nickel-org/nickel.rs/commit/6de1d279531dd20b43eb1ea5838ece848886bc3c))
* **nickel**
  *  Added a println of IP:Port when the server starts listening for requests. ([b0d6011c](https://github.com/nickel-org/nickel.rs/commit/b0d6011c3a9a0c79b7de12a4d089e195899ba691))
  *  Support for sharing data between requests per route basis ([5b1fbba9](https://github.com/nickel-org/nickel.rs/commit/5b1fbba96bad5fdf1b202a8fc901e0fb954c813c))
* **ResponseFinalizer**  add default mimetypes ([07e602c6](https://github.com/nickel-org/nickel.rs/commit/07e602c6b00385b6c78cd815f6b07f3d9e9cea8f))
* **docs**  add documentation generation ([655351cb](https://github.com/nickel-org/nickel.rs/commit/655351cbb063040d12f17a745d013bbc14a132c7))
* **favicon**  add support for favicons ([c00b329a](https://github.com/nickel-org/nickel.rs/commit/c00b329a19abd61c258174e1ca750869d753d353))
* **middleware**
  *  lazy querystring and json parsers ([b8bb31d0](https://github.com/nickel-org/nickel.rs/commit/b8bb31d0efe47f105f6701f73efe0ecd4a6c83de))
  *  introduce concept of middleware ([8a4f6831](https://github.com/nickel-org/nickel.rs/commit/8a4f6831fceb94db579e835a2026c82765301f9d), closes [#14](https://github.com/nickel-org/nickel.rs/issues/14))
* **static-files**  add basic support ([2bb6f833](https://github.com/nickel-org/nickel.rs/commit/2bb6f8331391c445a8a0fb2cb43b5b536b89781a))
* **Readme**
  *  link to documentation ([98d9b891](https://github.com/nickel-org/nickel.rs/commit/98d9b891609f61b5fd0b94fa9ce402e69d055d85))
  *  some tweaks ([cc6112d2](https://github.com/nickel-org/nickel.rs/commit/cc6112d20bb4bfd1baa0a51d12cd3d4f96f58ad5))
  *  improve the onboarding experience ([597233d1](https://github.com/nickel-org/nickel.rs/commit/597233d1ae766be0c351a9609602cd79a0578296))
* **Router**  implemented routes with variables ([6d0a3758](https://github.com/nickel-org/nickel.rs/commit/6d0a375830c3e28ed0898ff12b0dbf6e789c1609))
* **json_body_parser**  add json support ([fec76019](https://github.com/nickel-org/nickel.rs/commit/fec76019ac596f939be73f2481a60ed997a495da))
* **tests**  add test command to Makefile ([0c1b8d2c](https://github.com/nickel-org/nickel.rs/commit/0c1b8d2cc633d79080893a52b4da9598ad84febd))
* **hyper**  use hyper instead of rust-http ([5c6ff040](https://github.com/nickel-org/nickel.rs/commit/5c6ff0405b0a329e1d730cb6ff214412bc961b7c))



