# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

- - -
## [v1.2.87](https://github.com/specdown/specdown/compare/v1.2.86..v1.2.87) - 2023-09-29
#### Bug Fixes
- bump clap from 4.4.5 to 4.4.6 - ([556adc3](https://github.com/specdown/specdown/commit/556adc33eeade89a0d2a94abba19703f44946e3a)) - dependabot[bot]
#### Continuous Integration
- Update .mergify.yml - ([20b49a3](https://github.com/specdown/specdown/commit/20b49a3f17a085c7770c391dcbf055bb61ec14a3)) - Tom Oram
- bump armakuni/github-actions from 0.14.1 to 0.14.2 - ([5a3c4e4](https://github.com/specdown/specdown/commit/5a3c4e44dbe9942d9a3d458ec111bb97cbda593e)) - dependabot[bot]

- - -

## [v1.2.86](https://github.com/specdown/specdown/compare/v1.2.85..v1.2.86) - 2023-09-29
#### Bug Fixes
- Checkout repo with token - ([7d12107](https://github.com/specdown/specdown/commit/7d12107d1a732f82cb95eff107790bb1bf5a7e98)) - Tom Oram

- - -

## [v1.2.85](https://github.com/specdown/specdown/compare/v1.2.84..v1.2.85) - 2023-09-29
#### Bug Fixes
- Removed -C flags which accidentally got left in - ([be50151](https://github.com/specdown/specdown/commit/be501510d69b2d19abb4f2b02679944446d72d36)) - Tom Oram

- - -

## [v1.2.84](https://github.com/specdown/specdown/compare/v1.2.83..v1.2.84) - 2023-09-29
#### Bug Fixes
- Add v prefix to git tags - ([cf2accf](https://github.com/specdown/specdown/commit/cf2accf9dcb7332aa87e012476adf8347d713494)) - Tom Oram

- - -

## [v1.2.83](https://github.com/specdown/specdown/compare/v1.2.82..v1.2.83) - 2023-09-29
#### Bug Fixes
- Update generate-formula job - ([2bc92b1](https://github.com/specdown/specdown/commit/2bc92b10c7be31a1d9ceff57a626b0e9f3f42bdb)) - Tom Oram

- - -

## [v1.2.82](https://github.com/specdown/specdown/compare/v1.2.81..v1.2.82) - 2023-09-28
#### Bug Fixes
- Dependencies for generate-formula - ([fae6b31](https://github.com/specdown/specdown/commit/fae6b315d80a0afbb0c6c2b92eb10f2d4443e3a1)) - Tom Oram

- - -

## [v1.2.81](https://github.com/specdown/specdown/compare/v1.2.80..v1.2.81) - 2023-09-28
#### Bug Fixes
- Move back to the Armakuni workflow - ([b6eaf61](https://github.com/specdown/specdown/commit/b6eaf611ebec717c74fab0c3cdcd770851b824af)) - Tom Oram

- - -

## [v1.2.80](https://github.com/specdown/specdown/compare/v1.2.79..v1.2.80) - 2023-09-28
#### Bug Fixes
- Put correct bins path back in - ([301aa5e](https://github.com/specdown/specdown/commit/301aa5e0a42a33342335cb6af2ea8b54175b31c0)) - Tom Oram

- - -

## [v1.2.79](https://github.com/specdown/specdown/compare/v1.2.78..v1.2.79) - 2023-09-28
#### Bug Fixes
- Try downloading artifacts after the checkout - ([42aaccb](https://github.com/specdown/specdown/commit/42aaccb6f9a8d75aac417c99c2f2e25a6f087da0)) - Tom Oram

- - -

## [v1.2.78](https://github.com/specdown/specdown/compare/v1.2.77..v1.2.78) - 2023-09-28
#### Bug Fixes
- Try a different artifact path - ([bd41b19](https://github.com/specdown/specdown/commit/bd41b19555ec84f09a0788e908c10287bd9877ec)) - Tom Oram

- - -

## [v1.2.77](https://github.com/specdown/specdown/compare/v1.2.76..v1.2.77) - 2023-09-28
#### Bug Fixes
- Strip the v from the version - ([e508bc0](https://github.com/specdown/specdown/commit/e508bc0fad150e85067e591856bcde2b61b1ae86)) - Tom Oram
- Bump version of check-conventional-commits - ([2e5b14c](https://github.com/specdown/specdown/commit/2e5b14c537ab8e62ed5a79862b72535675f5bf1b)) - Tom Oram
- Bump version in the Cargo files - ([f2dc805](https://github.com/specdown/specdown/commit/f2dc80503b8922f83c0955d471c9088e9f5d8a4f)) - Tom Oram
- Correct version in Cargo.lock - ([4905ab2](https://github.com/specdown/specdown/commit/4905ab21c734beaee29917484d80253647c8c4e1)) - Tom Oram
- Add debug output to release pipeline - ([72cc0f2](https://github.com/specdown/specdown/commit/72cc0f22b40aa1e43385280a021b89cf5c7fdceb)) - Tom Oram
#### Continuous Integration
- Fix version number - ([306bb79](https://github.com/specdown/specdown/commit/306bb7929d3bd65dade1654bf02b6ebbd0b26336)) - Tom Oram

- - -

## [v1.2.76](https://github.com/specdown/specdown/compare/v1.2.75..v1.2.76) - 2023-09-28
#### Bug Fixes
- Force version bump - ([074ff70](https://github.com/specdown/specdown/commit/074ff70881f566d626bdd4bf128dcde0e6c97da3)) - Tom Oram
#### Continuous Integration
- Fix branch name - ([6ab2068](https://github.com/specdown/specdown/commit/6ab2068c683bd8a228afae0a91a9c8fc3080d0d9)) - Tom Oram
- Fix version for armakuni/github-actions - ([7f6322a](https://github.com/specdown/specdown/commit/7f6322aba0b6335e738e58ed3595608b79b91789)) - Tom Oram
- Fix changelog generation - ([aebc486](https://github.com/specdown/specdown/commit/aebc4865a28cd2f500b405b0a09f22618a13c64b)) - Tom Oram
- Use Armakuni's tag-and-release workflow - ([a1d051e](https://github.com/specdown/specdown/commit/a1d051eb68dfd7ecea0cb9211ac0468f83e6ee33)) - Tom Oram
- Extract publish website into a workflow - ([f7e4cc8](https://github.com/specdown/specdown/commit/f7e4cc888ba4ed70a67342c0827362ba1b4e6714)) - Tom Oram
- Remove reserved GITHUB_TOKEN secret - ([465af78](https://github.com/specdown/specdown/commit/465af78dc148682375ffa75701821e65c093321d)) - Tom Oram
- Extract tests to a workflow - ([6ea0012](https://github.com/specdown/specdown/commit/6ea0012f24a5773d6e51988814e6c7f582462be4)) - Tom Oram
- bump actions/cache from 3.3.1 to 3.3.2 - ([bed3a4f](https://github.com/specdown/specdown/commit/bed3a4f7edf503b0c2740de4084ce3ef557284d8)) - dependabot[bot]
- bump ncipollo/release-action from 1.12.0 to 1.13.0 - ([3bbd232](https://github.com/specdown/specdown/commit/3bbd232b94ee9f550ee5b496173048851e42ed82)) - dependabot[bot]

- - -


## [v1.2.74](https://github.com/specdown/specdown/releases/tag/v1.2.74) - 2023-09-27

- [`dd05d8d`](https://github.com/specdown/specdown/commit/dd05d8d2fd5d64de8878c625548f6f8b731657c8) Merge pull request #305 from specdown/dependabot/cargo/clap-4.4.5
- [`9c703f9`](https://github.com/specdown/specdown/commit/9c703f92a62af1e2297d769203cae4b6a2bb5cca) fix: bump clap from 4.3.24 to 4.4.5
- [`eb83e86`](https://github.com/specdown/specdown/commit/eb83e8615be13608fab4c580e9bc105a47e3e2c1) style: Run clippy
- [`2c7b59b`](https://github.com/specdown/specdown/commit/2c7b59b1067a4e14282b1667668b6ec3ebd147a0) docs: Update homebrew repo name

## [v1.2.73](https://github.com/specdown/specdown/releases/tag/v1.2.73) - 2023-08-24

- [`557ba07`](https://github.com/specdown/specdown/commit/557ba076f5e4a608fc39ae3b88cef58839472447) Merge pull request #295 from specdown/dependabot/cargo/clap-4.3.24
- [`16006ea`](https://github.com/specdown/specdown/commit/16006eaf6f6bf1f35ddb83644b1374d3bb0db552) fix: bump clap from 4.3.23 to 4.3.24

## [v1.2.72](https://github.com/specdown/specdown/releases/tag/v1.2.72) - 2023-08-21

- [`d6fd303`](https://github.com/specdown/specdown/commit/d6fd303059d08a35de60560ca8f0dfe43c77b804) Merge pull request #293 from specdown/dependabot/cargo/tempfile-3.8.0
- [`5fe31af`](https://github.com/specdown/specdown/commit/5fe31afcfa6bb39ccb7f6810d0f5acbe21482065) Merge branch master into dependabot/cargo/tempfile-3.8.0
- [`0b91966`](https://github.com/specdown/specdown/commit/0b919669910752e3ffd5f6ac433958a39fce492c) Merge branch master into dependabot/cargo/tempfile-3.8.0
- [`b7d49b6`](https://github.com/specdown/specdown/commit/b7d49b654254d62da633bcb5fc41e7490f3874fe) fix: bump tempfile from 3.7.1 to 3.8.0

## [v1.2.71](https://github.com/specdown/specdown/releases/tag/v1.2.71) - 2023-08-21

- [`7c68f11`](https://github.com/specdown/specdown/commit/7c68f11e5d2bb4ecc3527f2165c0390645998bb6) Merge pull request #294 from specdown/dependabot/cargo/clap-4.3.23
- [`6ef52dd`](https://github.com/specdown/specdown/commit/6ef52dd34b63ccb6ebb220d1d271d06894b02eb9) fix: bump clap from 4.3.22 to 4.3.23

## [v1.2.70](https://github.com/specdown/specdown/releases/tag/v1.2.70) - 2023-08-18

- [`f2098e9`](https://github.com/specdown/specdown/commit/f2098e97c255f4751dadcfd28abc2d6598c54215) Merge pull request #292 from specdown/dependabot/cargo/clap-4.3.22
- [`4897780`](https://github.com/specdown/specdown/commit/489778038a86d4fbc2d5c7b906bc492b1c6a8c37) fix: bump clap from 4.3.21 to 4.3.22

## [v1.2.69](https://github.com/specdown/specdown/releases/tag/v1.2.69) - 2023-08-08

- [`4f19261`](https://github.com/specdown/specdown/commit/4f19261d4e0aea2269df3ce5496f92d362ae3519) Merge pull request #289 from specdown/dependabot/cargo/clap-4.3.21
- [`99c924e`](https://github.com/specdown/specdown/commit/99c924ecda405bd17e4b82b58cfd21ae68cc212d) fix: bump clap from 4.3.19 to 4.3.21

## [v1.2.68](https://github.com/specdown/specdown/releases/tag/v1.2.68) - 2023-08-07

- [`fcad8f2`](https://github.com/specdown/specdown/commit/fcad8f2ae805926b8ba24a9983210597ed915e87) Merge pull request #288 from specdown/dependabot/cargo/crossterm-0.27.0
- [`71d3168`](https://github.com/specdown/specdown/commit/71d3168e577134870d09a6acdc0585c5b5476391) Merge branch master into dependabot/cargo/crossterm-0.27.0
- [`3bebd50`](https://github.com/specdown/specdown/commit/3bebd50c8becb89d316ee6aee8086a8bd4ecffa5) Merge branch master into dependabot/cargo/crossterm-0.27.0
- [`859c8c7`](https://github.com/specdown/specdown/commit/859c8c7ce498d1487cb5248e07656d201f9b0839) fix: bump crossterm from 0.26.1 to 0.27.0

## [v1.2.67](https://github.com/specdown/specdown/releases/tag/v1.2.67) - 2023-08-07

- [`8a81e7b`](https://github.com/specdown/specdown/commit/8a81e7be8851ad1c4925abe2356240f1d844b8e8) Merge pull request #287 from specdown/dependabot/cargo/tempfile-3.7.1
- [`bd61ced`](https://github.com/specdown/specdown/commit/bd61cedbfa584965a5c41dbdf65a004aac75bda0) fix: bump tempfile from 3.7.0 to 3.7.1

## [v1.2.66](https://github.com/specdown/specdown/releases/tag/v1.2.66) - 2023-07-24

- [`ccab29b`](https://github.com/specdown/specdown/commit/ccab29b579fb8a18f61c0ff5143395c14a54101e) Merge pull request #286 from specdown/dependabot/cargo/clap-4.3.19
- [`d4b8b30`](https://github.com/specdown/specdown/commit/d4b8b30d335393b28bc6ecf67ef30d328a4c8807) fix: bump clap from 4.3.17 to 4.3.19

## [v1.2.65](https://github.com/specdown/specdown/releases/tag/v1.2.65) - 2023-07-21

- [`1abe844`](https://github.com/specdown/specdown/commit/1abe8445ad83cd5bcc687a2fee54c20aa0c8a355) Merge pull request #285 from specdown/dependabot/cargo/tempfile-3.7.0
- [`0d7c9ee`](https://github.com/specdown/specdown/commit/0d7c9eefe14f189521532d674b70a463c6951c2b) fix: bump tempfile from 3.6.0 to 3.7.0

## [v1.2.64](https://github.com/specdown/specdown/releases/tag/v1.2.64) - 2023-07-20

- [`d06c971`](https://github.com/specdown/specdown/commit/d06c9713d8b07be3868a6a0067526635bfcb5cd7) Merge pull request #284 from specdown/dependabot/cargo/clap-4.3.17
- [`b76aa89`](https://github.com/specdown/specdown/commit/b76aa8929062b3d1beaf780ab686c49ef4d04d18) fix: bump clap from 4.3.16 to 4.3.17

## [v1.2.63](https://github.com/specdown/specdown/releases/tag/v1.2.63) - 2023-07-19

- [`09dfeaa`](https://github.com/specdown/specdown/commit/09dfeaa0558c18d606e442da7cc0a717b1aa8f62) Merge pull request #283 from specdown/dependabot/cargo/clap-4.3.16
- [`3cf0e65`](https://github.com/specdown/specdown/commit/3cf0e65cda88d515b80346817a8397d9c27473e1) fix: bump clap from 4.3.15 to 4.3.16

## [v1.2.62](https://github.com/specdown/specdown/releases/tag/v1.2.62) - 2023-07-18

- [`ecc09d9`](https://github.com/specdown/specdown/commit/ecc09d9820abc42ce25e5b00654b6096907e86c2) Merge pull request #282 from specdown/dependabot/cargo/clap-4.3.15
- [`ede1fdf`](https://github.com/specdown/specdown/commit/ede1fdf9d4927ddfde41a3eb29f437de0dd95f3a) fix: bump clap from 4.3.12 to 4.3.15

## [v1.2.61](https://github.com/specdown/specdown/releases/tag/v1.2.61) - 2023-07-17

- [`14a91fc`](https://github.com/specdown/specdown/commit/14a91fc3b129026622df0a4e2b87b90081d43981) Merge pull request #281 from specdown/dependabot/cargo/indoc-2.0.3
- [`c354090`](https://github.com/specdown/specdown/commit/c35409042262fb3c860d283dd347753bc04c2e61) Merge branch master into dependabot/cargo/indoc-2.0.3
- [`46e38fd`](https://github.com/specdown/specdown/commit/46e38fd961c3edd6d5f18ba70688a1cdac47ac62) Merge pull request #280 from specdown/dependabot/cargo/clap-4.3.12
- [`254ac40`](https://github.com/specdown/specdown/commit/254ac4084ac645c180341994d4a78d9e0d079f05) Merge branch master into dependabot/cargo/clap-4.3.12
- [`d944b98`](https://github.com/specdown/specdown/commit/d944b98656dc0a5d4e353ccfda9403078e03f137) Merge branch master into dependabot/cargo/clap-4.3.12
- [`ecc5145`](https://github.com/specdown/specdown/commit/ecc5145ba0ed85b5a5945152d729e2ca19dd0242) fix: bump indoc from 2.0.2 to 2.0.3
- [`73a879d`](https://github.com/specdown/specdown/commit/73a879dcca60d3674648aed25c6b6162f1ef730e) fix: bump clap from 4.3.11 to 4.3.12

## [v1.2.60](https://github.com/specdown/specdown/releases/tag/v1.2.60) - 2023-07-17

- [`f0bd462`](https://github.com/specdown/specdown/commit/f0bd462b65a5919363a7c969300051d329a2b4b8) Merge pull request #279 from specdown/dependabot/cargo/assert_cmd-2.0.12
- [`1424364`](https://github.com/specdown/specdown/commit/1424364efd8eade2c7e46936c6efce4f2aa63da7) fix: bump assert_cmd from 2.0.11 to 2.0.12
- [`8201cce`](https://github.com/specdown/specdown/commit/8201cce3655f28be031e5d50795e2fff585aa14a) Merge pull request #278 from specdown/dependabot/cargo/clap-4.3.11
- [`cf644e1`](https://github.com/specdown/specdown/commit/cf644e15cc75d413788081cdbb6c35d005c2452e) fix: bump clap from 4.3.10 to 4.3.11

## [v1.2.59](https://github.com/specdown/specdown/releases/tag/v1.2.59) - 2023-07-04

- [`475b06c`](https://github.com/specdown/specdown/commit/475b06cf1c94ed79f68191d1f40ddd19c8bd0c27) Merge pull request #277 from specdown/dependabot/cargo/indoc-2.0.2
- [`e46d2c2`](https://github.com/specdown/specdown/commit/e46d2c230daf69c0faf7fed37d40fb0776d7f515) fix: bump indoc from 2.0.1 to 2.0.2

## [v1.2.58](https://github.com/specdown/specdown/releases/tag/v1.2.58) - 2023-06-30

- [`3770af2`](https://github.com/specdown/specdown/commit/3770af22782d28cc88f3f1a8901e4b21c2c00785) Merge pull request #276 from specdown/dependabot/cargo/clap-4.3.10
- [`95809a3`](https://github.com/specdown/specdown/commit/95809a31e7eaff2eb27762e7d3d2e7024db22a76) fix: bump clap from 4.3.9 to 4.3.10

## [v1.2.57](https://github.com/specdown/specdown/releases/tag/v1.2.57) - 2023-06-29

- [`2a97e22`](https://github.com/specdown/specdown/commit/2a97e22b251c654bfae19287bed81712ab7c5dd0) Merge pull request #275 from specdown/dependabot/cargo/clap-4.3.9
- [`82ddc07`](https://github.com/specdown/specdown/commit/82ddc07308f8395d571f26e40f0502cc21b78954) fix: bump clap from 4.3.8 to 4.3.9

## [v1.2.56](https://github.com/specdown/specdown/releases/tag/v1.2.56) - 2023-06-26

- [`2f45438`](https://github.com/specdown/specdown/commit/2f4543858ef9da9c63dbc8c8eed0c08e8c27f435) Merge pull request #274 from specdown/dependabot/cargo/clap-4.3.8
- [`da5609d`](https://github.com/specdown/specdown/commit/da5609d2f814bdf8e4205274885f3820e58f1b65) fix: bump clap from 4.3.5 to 4.3.8

## [v1.2.55](https://github.com/specdown/specdown/releases/tag/v1.2.55) - 2023-06-21

- [`8f87d95`](https://github.com/specdown/specdown/commit/8f87d95544165acd052977c6991128cc9b87b5d4) Merge pull request #273 from specdown/dependabot/cargo/clap-4.3.5
- [`5ca9469`](https://github.com/specdown/specdown/commit/5ca9469fa6672d9df898c9b2a578100f360586c5) fix: bump clap from 4.3.4 to 4.3.5

## [v1.2.54](https://github.com/specdown/specdown/releases/tag/v1.2.54) - 2023-06-15

- [`22f6fca`](https://github.com/specdown/specdown/commit/22f6fca3370ea7536664eb126a76ae7247e7ce82) Merge pull request #272 from specdown/dependabot/cargo/clap-4.3.4
- [`4d026b1`](https://github.com/specdown/specdown/commit/4d026b1e66a892b1bffc6133beec0335fc6b3d08) fix: bump clap from 4.3.3 to 4.3.4

## [v1.2.53](https://github.com/specdown/specdown/releases/tag/v1.2.53) - 2023-06-12

- [`70ef398`](https://github.com/specdown/specdown/commit/70ef39865ed7b3104c81873e3dade55c14ea2ef7) Merge pull request #271 from specdown/dependabot/cargo/clap-4.3.3
- [`1b11e7e`](https://github.com/specdown/specdown/commit/1b11e7e0a067c700f103a9304ca1fc416ea4ee6c) fix: bump clap from 4.3.2 to 4.3.3

## [v1.2.52](https://github.com/specdown/specdown/releases/tag/v1.2.52) - 2023-06-07

- [`39d8e63`](https://github.com/specdown/specdown/commit/39d8e63f3979d69f8820c03a52edcb9748c788f1) Merge pull request #270 from specdown/dependabot/cargo/tempfile-3.6.0
- [`ddfc62c`](https://github.com/specdown/specdown/commit/ddfc62c6c6516ca95c3351f7e0b042c4b38d6970) fix: bump tempfile from 3.5.0 to 3.6.0

## [v1.2.51](https://github.com/specdown/specdown/releases/tag/v1.2.51) - 2023-06-06

- [`407a899`](https://github.com/specdown/specdown/commit/407a899777a704534423f80b680f31d88239804e) Merge pull request #269 from specdown/dependabot/cargo/clap-4.3.2
- [`0c68c43`](https://github.com/specdown/specdown/commit/0c68c43c7c4d165ef6ac662a7d7c8ab78557ab2d) fix: bump clap from 4.3.1 to 4.3.2

## [v1.2.50](https://github.com/specdown/specdown/releases/tag/v1.2.50) - 2023-06-05

- [`fb52249`](https://github.com/specdown/specdown/commit/fb52249aec1c42d90b11fe0a12922fae86eac7f6) Merge pull request #268 from specdown/dependabot/cargo/clap-4.3.1
- [`e45458c`](https://github.com/specdown/specdown/commit/e45458c16d1a986ed4514c35c5e81daea4db940a) fix: bump clap from 4.3.0 to 4.3.1

## [v1.2.49](https://github.com/specdown/specdown/releases/tag/v1.2.49) - 2023-05-22

- [`4222429`](https://github.com/specdown/specdown/commit/422242970e6198c05ab4d5a5026819d96cc39472) Merge pull request #267 from specdown/dependabot/cargo/clap-4.3.0
- [`492005b`](https://github.com/specdown/specdown/commit/492005bcbec1eb58a686b6d06b747228f0e53250) fix: bump clap from 4.2.7 to 4.3.0

## [v1.2.48](https://github.com/specdown/specdown/releases/tag/v1.2.48) - 2023-05-12

- [`08ed2c0`](https://github.com/specdown/specdown/commit/08ed2c013654d6d5b544efb0189be437ffd47e19) Merge pull request #266 from PurpleBooth/patch-3
- [`42a7777`](https://github.com/specdown/specdown/commit/42a7777e2f7744ed17912cc5099911cfa33c7268) fix(homebrew): Correct the completion command

## [v1.2.47](https://github.com/specdown/specdown/releases/tag/v1.2.47) - 2023-05-03

- [`3b49e57`](https://github.com/specdown/specdown/commit/3b49e579a76aeee880109f5788fbf490bd3e873e) Merge pull request #265 from specdown/dependabot/cargo/clap-4.2.7
- [`bc82ced`](https://github.com/specdown/specdown/commit/bc82cedd51fe3df8540a3cf648a86130d4544a25) fix: bump clap from 4.2.5 to 4.2.7

## [v1.2.46](https://github.com/specdown/specdown/releases/tag/v1.2.46) - 2023-04-28

- [`e0368cc`](https://github.com/specdown/specdown/commit/e0368cca726dfa0531f76d1dc0ac6b057f0f4188) Merge pull request #264 from specdown/dependabot/cargo/clap-4.2.5
- [`d243526`](https://github.com/specdown/specdown/commit/d243526479b8f7826391943445cdd1b0948f5f12) fix: bump clap from 4.2.4 to 4.2.5

## [v1.2.45](https://github.com/specdown/specdown/releases/tag/v1.2.45) - 2023-04-20

- [`1b5d480`](https://github.com/specdown/specdown/commit/1b5d480fc9a108588fe82422d0b6133ea0a84928) Merge pull request #263 from specdown/dependabot/cargo/clap-4.2.4
- [`0cdc23d`](https://github.com/specdown/specdown/commit/0cdc23d1b3cfe82362eaf5d6139729d8865c686d) fix: bump clap from 4.2.3 to 4.2.4

## [v1.2.44](https://github.com/specdown/specdown/releases/tag/v1.2.44) - 2023-04-19

- [`6809db8`](https://github.com/specdown/specdown/commit/6809db862d408c903bbf61ad4af3e95bf5bb374f) Merge pull request #262 from specdown/dependabot/cargo/clap-4.2.3
- [`d4d549d`](https://github.com/specdown/specdown/commit/d4d549d15ba2472ce096544303f2f23562638159) fix: bump clap from 4.2.2 to 4.2.3

## [v1.2.43](https://github.com/specdown/specdown/releases/tag/v1.2.43) - 2023-04-14

- [`1a9c7f6`](https://github.com/specdown/specdown/commit/1a9c7f6317eefca0e51751859eee9ba1d8fb46ac) Merge pull request #261 from specdown/dependabot/cargo/clap-4.2.2
- [`e5537d7`](https://github.com/specdown/specdown/commit/e5537d749f9ce24f1510e5607eff8407276d9d35) fix: bump clap from 4.1.14 to 4.2.2

## [v1.2.42](https://github.com/specdown/specdown/releases/tag/v1.2.42) - 2023-04-14

- [`e62dbc9`](https://github.com/specdown/specdown/commit/e62dbc9c63a08cb0a94bb924af97d04aa418ae1c) Merge pull request #260 from specdown/dependabot/cargo/assert_cmd-2.0.11
- [`f69ae19`](https://github.com/specdown/specdown/commit/f69ae1938aec39d4b92b7c5eb2971910c7af512c) fix: bump assert_cmd from 2.0.10 to 2.0.11
- [`5225dc0`](https://github.com/specdown/specdown/commit/5225dc052f829978f1189d237d333002c1fef8fe) Merge pull request #232 from specdown/dependabot/github_actions/cuchi/jinja2-action-1.2.1
- [`b9938a3`](https://github.com/specdown/specdown/commit/b9938a3e77929efe233a0e628c3b76490baa4072) Merge pull request #249 from specdown/dependabot/github_actions/actions/cache-3.3.1
- [`b9bcc25`](https://github.com/specdown/specdown/commit/b9bcc255451a51fb6688e8d1bbf8ce5c8aa0073b) ci: bump actions/cache from 3.0.11 to 3.3.1
- [`4d1fbcf`](https://github.com/specdown/specdown/commit/4d1fbcf16383f34e5c55595006bb74282b3d0ef8) ci: bump cuchi/jinja2-action from 1.2.0 to 1.2.1

## [v1.2.41](https://github.com/specdown/specdown/releases/tag/v1.2.41) - 2023-04-11

- [`e3e91f7`](https://github.com/specdown/specdown/commit/e3e91f77e47b41bfb56a899e94096a804a00149d) Merge pull request #254 from specdown/dependabot/github_actions/dlavrenuek/conventional-changelog-action-1.2.3
- [`13ac922`](https://github.com/specdown/specdown/commit/13ac922fa87eb35193c0fbf246d864454daf2fd5) ci: bump dlavrenuek/conventional-changelog-action from 1.2.2 to 1.2.3
- [`b00f84b`](https://github.com/specdown/specdown/commit/b00f84b51e26e50ce094d931b37e8c311b77b30d) ci: Fix GITHUB_OUTPUT format
- [`2670c21`](https://github.com/specdown/specdown/commit/2670c2119f9b47b584785ab14340ed7f34b932a8) ci: Ignore ANSI colour code
- [`93de8d5`](https://github.com/specdown/specdown/commit/93de8d59ab42976fcbdf3cc53e1dce90c24bf14c) Merge pull request #223 from specdown/dependabot/github_actions/ncipollo/release-action-1.12.0
- [`56e9b0f`](https://github.com/specdown/specdown/commit/56e9b0ff14c3bd4f1d75901d67bcd82c8d1c9e78) Merge pull request #258 from specdown/dependabot/cargo/tempfile-3.5.0
- [`98fe842`](https://github.com/specdown/specdown/commit/98fe842b14385a389249749cdb18a02560e89cfa) fix: bump tempfile from 3.4.0 to 3.5.0
- [`27a9409`](https://github.com/specdown/specdown/commit/27a94091cfb6cc6ea2709140ecb4db444fb4e496) Merge pull request #257 from specdown/dependabot/cargo/clap-4.1.14
- [`7a4e2c5`](https://github.com/specdown/specdown/commit/7a4e2c5c387f736eb7bf08eaee9ace0550fcff93) fix: bump clap from 4.1.13 to 4.1.14
- [`99bbfeb`](https://github.com/specdown/specdown/commit/99bbfebc541d79161da0fe8bcc74e7ef9f8fcd96) Merge pull request #255 from specdown/dependabot/cargo/clap-4.1.13
- [`c16ad24`](https://github.com/specdown/specdown/commit/c16ad24c53b83ce2df98cd720fe6089d85135204) fix: bump clap from 4.1.11 to 4.1.13
- [`9200f70`](https://github.com/specdown/specdown/commit/9200f7005efcf64f395d2caabff37e8bcd04ebdd) Merge pull request #253 from specdown/dependabot/cargo/clap-4.1.11
- [`1a3b677`](https://github.com/specdown/specdown/commit/1a3b67738104636c9941c13381c05b4d9ba4a2c7) fix: bump clap from 4.1.9 to 4.1.11
- [`fe58564`](https://github.com/specdown/specdown/commit/fe58564f35137923a8365dd8909d5925b93bfdcf) Merge pull request #252 from specdown/dependabot/cargo/clap-4.1.9
- [`288676e`](https://github.com/specdown/specdown/commit/288676ecc9e84953284a6340059aa3e2180cb73c) Merge branch master into dependabot/cargo/clap-4.1.9
- [`078fa54`](https://github.com/specdown/specdown/commit/078fa54a41eab3b0d1b53f222fa4045468ccf932) Merge pull request #251 from specdown/dependabot/cargo/assert_cmd-2.0.10
- [`3bb61e9`](https://github.com/specdown/specdown/commit/3bb61e9ec2b6e264ddd0e31e3e9442c719202265) fix: bump clap from 4.1.8 to 4.1.9
- [`34f0070`](https://github.com/specdown/specdown/commit/34f0070ff8977908772745e87af26bfd8f5a4347) fix: bump assert_cmd from 2.0.9 to 2.0.10
- [`2a4d536`](https://github.com/specdown/specdown/commit/2a4d5361d963f4bfbfa438965e78244dcc1b4431) Merge pull request #250 from specdown/dependabot/cargo/assert_cmd-2.0.9
- [`a72448b`](https://github.com/specdown/specdown/commit/a72448b151d017390cad9493bc6e4ac869be6070) fix: bump assert_cmd from 2.0.8 to 2.0.9
- [`7807282`](https://github.com/specdown/specdown/commit/780728221906e9720a5bd9b2c4dc1f14b9ab2e70) Merge pull request #248 from PurpleBooth/fix-formula
- [`d15204a`](https://github.com/specdown/specdown/commit/d15204adafae11d6aaa99080fbf788d48c99f655) Merge pull request #247 from PurpleBooth/fix-actionlint-advice
- [`1822a7c`](https://github.com/specdown/specdown/commit/1822a7c4d42fb052d7b1b6ed36721f794421c481) Merge pull request #246 from PurpleBooth/patch-2
- [`18d1359`](https://github.com/specdown/specdown/commit/18d1359b5743ad8a143494e1c58b5877f9177ade) chore: Update homebrew formula to use new completion helper
- [`7c71328`](https://github.com/specdown/specdown/commit/7c71328b4d5cdc98272c70cd7d0923234642a6e7) ci: Remove deprecated Mergify options
- [`669ef1d`](https://github.com/specdown/specdown/commit/669ef1da53f99cfe14fbda15d9a31a6c3baec8a4) ci: Update to new form of outputs
- [`0297f0a`](https://github.com/specdown/specdown/commit/0297f0a7ce2cf5b51bf861cff905180d2b5735ef) ci: bump ncipollo/release-action from 1.11.2 to 1.12.0

## [v1.2.40](https://github.com/specdown/specdown/releases/tag/v1.2.40) - 2023-03-11

- [`2b90ec4`](https://github.com/specdown/specdown/commit/2b90ec4ccaf20b07940f28ec6e8cd9dbde25435f) Merge pull request #245 from PurpleBooth/fix-clippy-advice
- [`e3c1f42`](https://github.com/specdown/specdown/commit/e3c1f42a0a0923ff10c40bfe5ba27e2c4ca74519) chore: Fix feedback from clippy and bump versions
- [`8fec92c`](https://github.com/specdown/specdown/commit/8fec92c04c6670f53d5ba0ee8710d79e918cf490) Merge pull request #231 from specdown/dependabot/cargo/termdiff-3.1.1
- [`c7c0a0f`](https://github.com/specdown/specdown/commit/c7c0a0fdcc36b6fffcd51a7002ff98a4da3d62c1) fix: bump termdiff from 3.1.0 to 3.1.1

## [v1.2.39](https://github.com/specdown/specdown/releases/tag/v1.2.39) - 2023-01-16

- [`df6b2a5`](https://github.com/specdown/specdown/commit/df6b2a5c6fb729936345ea591cb6edbf9fbb10c2) Merge pull request #230 from specdown/dependabot/cargo/nom-7.1.3
- [`870a51c`](https://github.com/specdown/specdown/commit/870a51c4965377a6d4c5b6c854fadec7b8c6968c) fix: bump nom from 7.1.2 to 7.1.3
- [`f54376e`](https://github.com/specdown/specdown/commit/f54376ec9778569ef86bcedd36fb1ab1306c2927) Merge pull request #228 from specdown/dependabot/cargo/nom-7.1.2
- [`406a8c1`](https://github.com/specdown/specdown/commit/406a8c1a6f0f15b6afb5a7352eadc826a31b3a1a) fix: bump nom from 7.1.1 to 7.1.2

## [v1.2.38](https://github.com/specdown/specdown/releases/tag/v1.2.38) - 2022-12-19

- [`7951925`](https://github.com/specdown/specdown/commit/79519254b5970fff49e9dd49dd8a74b760f4e02d) Merge pull request #224 from specdown/dependabot/cargo/indoc-1.0.8
- [`9f0e064`](https://github.com/specdown/specdown/commit/9f0e064a92c30c42e2feb191b0be153e1cf5ca2d) fix: bump indoc from 1.0.7 to 1.0.8
- [`ca30dc3`](https://github.com/specdown/specdown/commit/ca30dc34dd390acb88b63293035b9d87a472d58e) Merge pull request #220 from specdown/dependabot/cargo/clap-4.0.19
- [`a685f9a`](https://github.com/specdown/specdown/commit/a685f9ad6b04f6a6dc450cba82e07cc0d2e903f6) fix: bump clap from 3.2.22 to 4.0.19
- [`2494b31`](https://github.com/specdown/specdown/commit/2494b3112bb59871b9b862d441a2774023300e39) Merge pull request #219 from PurpleBooth/upgrade-clap
- [`8ce18aa`](https://github.com/specdown/specdown/commit/8ce18aabea8e2a1dada8d82f2921d967f07a96a5) fix: Update clap
- [`3e2b433`](https://github.com/specdown/specdown/commit/3e2b43365d9baee28cb9bd50764554d1749577cb) Merge pull request #222 from specdown/dependabot/github_actions/ncipollo/release-action-1.11.2
- [`7c9d3a1`](https://github.com/specdown/specdown/commit/7c9d3a1a8e392cc5556be67db11c2f092f5d539a) ci: bump ncipollo/release-action from 1.10.0 to 1.11.2
- [`16edbce`](https://github.com/specdown/specdown/commit/16edbce15c58d32e00a03b20c3b33f36b712781d) Merge pull request #211 from specdown/dependabot/github_actions/dlavrenuek/conventional-changelog-action-1.2.2
- [`0ac5521`](https://github.com/specdown/specdown/commit/0ac5521ccfc564da1f1d57fe09d9e4d03a860180) Merge pull request #210 from specdown/dependabot/github_actions/actions/cache-3.0.11
- [`0c9d52b`](https://github.com/specdown/specdown/commit/0c9d52b34ab2465f5b44a4a68b0084de0622e132) ci: bump actions/cache from 3.0.4 to 3.0.11
- [`632175b`](https://github.com/specdown/specdown/commit/632175b49f5687b27da0ba1446a51c7e2b9185cc) ci: bump dlavrenuek/conventional-changelog-action from 1.2.1 to 1.2.2

## [v1.2.37](https://github.com/specdown/specdown/releases/tag/v1.2.37) - 2022-11-11

- [`5fb087c`](https://github.com/specdown/specdown/commit/5fb087cbfbed97ea4a21dcd1bfd638e8f380dac3) Merge pull request #218 from PurpleBooth/follow-clippy-advice
- [`32b5e65`](https://github.com/specdown/specdown/commit/32b5e6526619f2a96d490154f05c55093902c35f) fix: Follow clippy advice
- [`84d4b2f`](https://github.com/specdown/specdown/commit/84d4b2f57c49961e3053567aa8098e5bb3b1aad8) Merge pull request #216 from specdown/dependabot/cargo/clap_derive-4.0.18
- [`4fb3a61`](https://github.com/specdown/specdown/commit/4fb3a61a4e6b010928b4ae0044bbe3260a9451e8) Merge branch master into dependabot/cargo/clap_derive-4.0.18
- [`032da4c`](https://github.com/specdown/specdown/commit/032da4c487fda718b02c83a7059a473b540d1434) Merge branch master into dependabot/cargo/clap_derive-4.0.18
- [`241eefe`](https://github.com/specdown/specdown/commit/241eefef2816ba6b0f975b52f3f91949e64b8dfb) fix: bump clap_derive from 4.0.13 to 4.0.18

## [v1.2.36](https://github.com/specdown/specdown/releases/tag/v1.2.36) - 2022-10-21

- [`7a48652`](https://github.com/specdown/specdown/commit/7a48652bc396ed0148ba5af7543d8d1c5568e166) Merge pull request #215 from specdown/dependabot/cargo/assert_cmd-2.0.5
- [`1548ee8`](https://github.com/specdown/specdown/commit/1548ee85b4cb2d789d391066e8c20eedbcc97951) fix: bump assert_cmd from 2.0.4 to 2.0.5
- [`09780b5`](https://github.com/specdown/specdown/commit/09780b54c9b03cafe6d5f23fc29d39760bfd9684) Merge pull request #207 from specdown/dependabot/cargo/clap_derive-4.0.13
- [`1e10d8b`](https://github.com/specdown/specdown/commit/1e10d8bc979554356cc2463b9029ea080e815f74) fix: bump clap_derive from 4.0.10 to 4.0.13

## [v1.2.35](https://github.com/specdown/specdown/releases/tag/v1.2.35) - 2022-10-06

- [`c10771f`](https://github.com/specdown/specdown/commit/c10771fefc807e9e0bba89bff97b07275147cac6) Merge pull request #203 from specdown/dependabot/cargo/clap_derive-4.0.10
- [`8ee8c3c`](https://github.com/specdown/specdown/commit/8ee8c3ca67b155129740a993de7be1f886156f16) fix: bump clap_derive from 4.0.9 to 4.0.10

## [v1.2.34](https://github.com/specdown/specdown/releases/tag/v1.2.34) - 2022-10-04

- [`0f2a1e7`](https://github.com/specdown/specdown/commit/0f2a1e77bd557f5b304963fe200b1278d35aaf63) Merge pull request #199 from specdown/dependabot/cargo/clap_derive-4.0.9
- [`81539c2`](https://github.com/specdown/specdown/commit/81539c2d33ca8e3f31396f5362cff9f073d3bf23) fix: bump clap_derive from 4.0.8 to 4.0.9

## [v1.2.33](https://github.com/specdown/specdown/releases/tag/v1.2.33) - 2022-10-03

- [`674ffc5`](https://github.com/specdown/specdown/commit/674ffc5537bd7c0d8974008de7bb00d83b80e7bb) Merge pull request #195 from specdown/dependabot/cargo/clap_derive-4.0.8
- [`ef9cfff`](https://github.com/specdown/specdown/commit/ef9cfff140aa575f6f30f8950e52afb94918ba49) fix: bump clap_derive from 4.0.1 to 4.0.8

## [v1.2.32](https://github.com/specdown/specdown/releases/tag/v1.2.32) - 2022-09-29

- [`f44fcff`](https://github.com/specdown/specdown/commit/f44fcff31489b0c5459a384b2507fc3f6805af4d) Merge pull request #191 from specdown/dependabot/cargo/clap_derive-4.0.1
- [`5def082`](https://github.com/specdown/specdown/commit/5def0828e208c9be9189cbfb5f9b8252f39875bf) fix: bump clap_derive from 3.2.18 to 4.0.1

## [v1.2.31](https://github.com/specdown/specdown/releases/tag/v1.2.31) - 2022-09-19

- [`8a0b66a`](https://github.com/specdown/specdown/commit/8a0b66aa8573a79f438f03c09d78148acd9705b4) Merge pull request #189 from specdown/dependabot/cargo/clap-3.2.22
- [`0fa90f1`](https://github.com/specdown/specdown/commit/0fa90f13a80ea63f49da88132fea11475ba51f48) fix: bump clap from 3.2.21 to 3.2.22

## [v1.2.30](https://github.com/specdown/specdown/releases/tag/v1.2.30) - 2022-09-13

- [`111f8f1`](https://github.com/specdown/specdown/commit/111f8f1d701137f7b65ee350f85c92aebf996469) Merge pull request #188 from specdown/dependabot/cargo/clap-3.2.21
- [`6fca67b`](https://github.com/specdown/specdown/commit/6fca67bf1775ec8f58c9a4e10d203894c032d3a5) fix: bump clap from 3.2.20 to 3.2.21

## [v1.2.29](https://github.com/specdown/specdown/releases/tag/v1.2.29) - 2022-09-08

- [`46aba3e`](https://github.com/specdown/specdown/commit/46aba3efd89bd5484caf09f176846ffb620976a5) Merge pull request #186 from specdown/dependabot/cargo/clap-3.2.20
- [`03cc057`](https://github.com/specdown/specdown/commit/03cc057273333609e742a38608da47d1fdc4611c) fix: bump clap from 3.2.16 to 3.2.20
- [`6ef2ed1`](https://github.com/specdown/specdown/commit/6ef2ed10aed65408b9c62f7d2fd94e9b8bb6ed18) Merge pull request #187 from specdown/linting
- [`5f4d68a`](https://github.com/specdown/specdown/commit/5f4d68a7b64d57b4cb27fa921c4e39f8cf348493) style: Run clippy

## [v1.2.28](https://github.com/specdown/specdown/releases/tag/v1.2.28) - 2022-08-11

- [`d311210`](https://github.com/specdown/specdown/commit/d311210c13baaa5da009105cc749685c2143197f) Merge pull request #180 from specdown/dependabot/cargo/crossterm-0.25.0
- [`8e64eb9`](https://github.com/specdown/specdown/commit/8e64eb9fb8dd7ea4791efad0cc5c0dc560705d8b) fix: bump crossterm from 0.24.0 to 0.25.0

## [v1.2.27](https://github.com/specdown/specdown/releases/tag/v1.2.27) - 2022-08-02

- [`2c68c17`](https://github.com/specdown/specdown/commit/2c68c17c19f6c21e78046358831276482c0f9c49) Merge pull request #178 from specdown/dependabot/cargo/indoc-1.0.7
- [`63ebeeb`](https://github.com/specdown/specdown/commit/63ebeeb669ea0499b38b8ba496f78434f86a00b8) fix: bump indoc from 1.0.6 to 1.0.7

## [v1.2.26](https://github.com/specdown/specdown/releases/tag/v1.2.26) - 2022-08-01

- [`a83ef85`](https://github.com/specdown/specdown/commit/a83ef8587dcd211e95d88fbb5cf26a58efd5ee91) Merge pull request #177 from specdown/dependabot/cargo/clap-3.2.16
- [`94dc1e0`](https://github.com/specdown/specdown/commit/94dc1e050bd4fd88193768d1db1395d17b24de53) fix: bump clap from 3.2.15 to 3.2.16

## [v1.2.25](https://github.com/specdown/specdown/releases/tag/v1.2.25) - 2022-07-26

- [`4da61f7`](https://github.com/specdown/specdown/commit/4da61f7ef84867e4da7d169317268182b785f6ff) Merge pull request #176 from specdown/dependabot/cargo/clap-3.2.15
- [`ced4dde`](https://github.com/specdown/specdown/commit/ced4dde6028be935d6d519f761aa3bf08df87232) fix: bump clap from 3.2.14 to 3.2.15

## [v1.2.24](https://github.com/specdown/specdown/releases/tag/v1.2.24) - 2022-07-21

- [`d3855e1`](https://github.com/specdown/specdown/commit/d3855e18aa829592302fa02141a6f7e1b0f08219) Merge pull request #175 from specdown/dependabot/cargo/clap-3.2.14
- [`d88da05`](https://github.com/specdown/specdown/commit/d88da05c9363aea4144833672d03bb278540fc7f) fix: bump clap from 3.2.13 to 3.2.14

## [v1.2.23](https://github.com/specdown/specdown/releases/tag/v1.2.23) - 2022-07-20

- [`0b6cc3d`](https://github.com/specdown/specdown/commit/0b6cc3de599a586aff5c135018b227a8a3ec76a4) Merge pull request #174 from specdown/dependabot/cargo/clap-3.2.13
- [`6dde8d3`](https://github.com/specdown/specdown/commit/6dde8d3be6300063b0273111336cf4d28ca50d94) fix: bump clap from 3.2.12 to 3.2.13

## [v1.2.22](https://github.com/specdown/specdown/releases/tag/v1.2.22) - 2022-07-15

- [`9ff3f48`](https://github.com/specdown/specdown/commit/9ff3f48002de987b76e455e44f1089fc245e6ef5) Merge pull request #173 from specdown/dependabot/cargo/clap-3.2.12
- [`227cb94`](https://github.com/specdown/specdown/commit/227cb949f0e9b7534ede41c7ec639334233946b5) fix: bump clap from 3.2.11 to 3.2.12

## [v1.2.21](https://github.com/specdown/specdown/releases/tag/v1.2.21) - 2022-07-14

- [`fe96d71`](https://github.com/specdown/specdown/commit/fe96d710667d85b22afbfda425b9779e7889c085) Merge pull request #172 from specdown/dependabot/cargo/clap-3.2.11
- [`2e10086`](https://github.com/specdown/specdown/commit/2e100869e6115a3ab747b98336f0ee71c612c72c) fix: bump clap from 3.2.10 to 3.2.11

## [v1.2.20](https://github.com/specdown/specdown/releases/tag/v1.2.20) - 2022-07-13

- [`8e31e31`](https://github.com/specdown/specdown/commit/8e31e31f392058afa34cc4040f9d4162c2e565b7) Merge pull request #171 from specdown/dependabot/cargo/comrak-0.14.0
- [`dde2ac5`](https://github.com/specdown/specdown/commit/dde2ac553cbf8f999ea0f1ab184ea433a765620c) fix: bump comrak from 0.13.2 to 0.14.0

## [v1.2.19](https://github.com/specdown/specdown/releases/tag/v1.2.19) - 2022-07-12

- [`83888f7`](https://github.com/specdown/specdown/commit/83888f756d5cb5ccb2fd713e629902a4873beae3) Merge pull request #169 from specdown/dependabot/cargo/clap-3.2.10
- [`bb52045`](https://github.com/specdown/specdown/commit/bb52045498aeb636604b41dd6a3c6ce69ab386a8) fix: bump clap from 3.2.8 to 3.2.10

## [v1.2.18](https://github.com/specdown/specdown/releases/tag/v1.2.18) - 2022-07-04

- [`9030847`](https://github.com/specdown/specdown/commit/90308472c4c2802e9773327192d780db55335819) Merge pull request #167 from specdown/dependabot/cargo/comrak-0.13.2
- [`5972267`](https://github.com/specdown/specdown/commit/59722675a30249df190158b8c969a61424066fc2) Merge branch master into dependabot/cargo/comrak-0.13.2
- [`bf60da0`](https://github.com/specdown/specdown/commit/bf60da0067d1a9d03449387216c7adb8207467a5) fix: bump comrak from 0.13.0 to 0.13.2

## [v1.2.17](https://github.com/specdown/specdown/releases/tag/v1.2.17) - 2022-07-04

- [`a995e69`](https://github.com/specdown/specdown/commit/a995e6976b7750242826c9391c8d3202a6c224b0) Merge pull request #168 from specdown/dependabot/cargo/crossterm-0.24.0
- [`ee7007e`](https://github.com/specdown/specdown/commit/ee7007e973a995c519092a1cfd1ef13d86f70ba1) fix: bump crossterm from 0.23.2 to 0.24.0

## [v1.2.16](https://github.com/specdown/specdown/releases/tag/v1.2.16) - 2022-07-01

- [`c7a6e06`](https://github.com/specdown/specdown/commit/c7a6e06c361ab8c01f4aadc9fe226c38cadee824) Merge pull request #166 from specdown/dependabot/cargo/clap-3.2.8
- [`ac310bd`](https://github.com/specdown/specdown/commit/ac310bdc519d3f531541b3d292184153e8a576a6) fix: bump clap from 3.2.7 to 3.2.8

## [v1.2.15](https://github.com/specdown/specdown/releases/tag/v1.2.15) - 2022-06-29

- [`038deee`](https://github.com/specdown/specdown/commit/038deee1b3e00c47b4c70a487c1fc0401ce7ff9e) Merge pull request #165 from specdown/dependabot/cargo/clap-3.2.7
- [`4b2ec57`](https://github.com/specdown/specdown/commit/4b2ec57551e37d75e7af3769e19f05d42f7017c9) fix: bump clap from 3.2.6 to 3.2.7
- [`1de3c7c`](https://github.com/specdown/specdown/commit/1de3c7c3885cf60a370444f3703ff9c42af47f58) Merge pull request #159 from specdown/dependabot/github_actions/actions/cache-3.0.4
- [`a5240e4`](https://github.com/specdown/specdown/commit/a5240e41908f9d800d6c9d0cb3b4bdb62ac9fde7) ci: bump actions/cache from 3.0.3 to 3.0.4

## [v1.2.14](https://github.com/specdown/specdown/releases/tag/v1.2.14) - 2022-06-22

- [`1c755f5`](https://github.com/specdown/specdown/commit/1c755f5660adb6d5525cefe3dc0e7298a8bd6f26) Merge pull request #163 from specdown/dependabot/cargo/clap-3.2.6
- [`136700f`](https://github.com/specdown/specdown/commit/136700f3e8975b4d8df6b175549619d9282ba0c1) fix: bump clap from 3.2.5 to 3.2.6

## [v1.2.13](https://github.com/specdown/specdown/releases/tag/v1.2.13) - 2022-06-16

- [`29303bd`](https://github.com/specdown/specdown/commit/29303bdff8421dfc74850b06349e689bc22f9b02) Merge pull request #162 from specdown/dependabot/cargo/clap-3.2.5
- [`b04af59`](https://github.com/specdown/specdown/commit/b04af5933259b0d154235b561e6ba20e5a7c812e) fix: bump clap from 3.2.4 to 3.2.5

## [v1.2.12](https://github.com/specdown/specdown/releases/tag/v1.2.12) - 2022-06-14

- [`fb5d187`](https://github.com/specdown/specdown/commit/fb5d187e3bb722d3857b60cfe37fc6ad8a2a6c61) Merge pull request #161 from specdown/dependabot/cargo/clap-3.2.4
- [`4d2cb4f`](https://github.com/specdown/specdown/commit/4d2cb4f39eaa9def0910e09ba1e19c6349e1c073) Merge branch master into dependabot/cargo/clap-3.2.4
- [`9060611`](https://github.com/specdown/specdown/commit/90606116ad2cc0fedf100e94b720d4e7473aa338) fix: bump clap from 3.1.18 to 3.2.4

## [v1.2.11](https://github.com/specdown/specdown/releases/tag/v1.2.11) - 2022-06-14

- [`31302ec`](https://github.com/specdown/specdown/commit/31302ec143291aefa573c43ea3f99f76f3712bbc) Merge pull request #158 from specdown/dependabot/cargo/comrak-0.13.0
- [`2728504`](https://github.com/specdown/specdown/commit/2728504d21a7dd88e956ad07a1ecfc1d7280d75e) Merge branch master into dependabot/cargo/comrak-0.13.0
- [`2753a10`](https://github.com/specdown/specdown/commit/2753a10b9d3707e0fbd7f19e9bbc484015acbbb8) ci: Remove uplift-dry-run from mergify
- [`624d5d3`](https://github.com/specdown/specdown/commit/624d5d39e734e3de399f65e2743927885ccf17e1) Revert "Run uplift-dry-run on branches"
- [`584e2ff`](https://github.com/specdown/specdown/commit/584e2ff9eb93ebb361c8101b1d61cdbda70d071a) fix: bump comrak from 0.12.1 to 0.13.0
- [`8d39bfd`](https://github.com/specdown/specdown/commit/8d39bfd7053701afb4f9a6a0d644fb2e8cb852ea) Run uplift-dry-run on branches
- [`1ef1801`](https://github.com/specdown/specdown/commit/1ef1801b988bdf069898c88767559760624810c2) Merge pull request #157 from tomphp/workspaces
- [`8c3f906`](https://github.com/specdown/specdown/commit/8c3f9065470db173835212d7ff5d046b2ff6d1c6) refactor: Extract workspace logic
- [`25ba72c`](https://github.com/specdown/specdown/commit/25ba72cabc5bf94791f6aebff7e20b43a6a84a4f) refactor: Extract Arguments for run command

## [v1.2.10](https://github.com/specdown/specdown/releases/tag/v1.2.10) - 2022-06-03

- [`072e30f`](https://github.com/specdown/specdown/commit/072e30feb68b49cae05fccce347bf4187ee00555) Merge pull request #155 from specdown/dependabot/github_actions/dlavrenuek/conventional-changelog-action-1.2.1
- [`7dccfe4`](https://github.com/specdown/specdown/commit/7dccfe4f758e39db254e71a38ccb8b0c193c599c) Merge pull request #154 from specdown/dependabot/github_actions/actions/cache-3.0.3
- [`a3d1826`](https://github.com/specdown/specdown/commit/a3d1826d430dc150c35eda587ae1215eaf02ec60) ci: Update mergify config for uplift
- [`eafe586`](https://github.com/specdown/specdown/commit/eafe58607189aa80b2655a90aa432d2e45625f81) Merge pull request #156 from tomphp/clap3
- [`b5c0c0c`](https://github.com/specdown/specdown/commit/b5c0c0c19b316a61157647b9ee10739de7b2d2b8) refactor: Use PathBuf as argument type
- [`9e8cc9f`](https://github.com/specdown/specdown/commit/9e8cc9f23cd794c42ffd9849bd770834f096ddaa) fix: Upgrade to clap 3
- [`7b32c8e`](https://github.com/specdown/specdown/commit/7b32c8e913ee91b099402f7363d1805fd872accd) ci: bump dlavrenuek/conventional-changelog-action from 1.2.0 to 1.2.1
- [`b806372`](https://github.com/specdown/specdown/commit/b8063725127d147c1f410668b9634162e0746ed7) ci: bump actions/cache from 3.0.2 to 3.0.3

## [v1.2.9](https://github.com/specdown/specdown/releases/tag/v1.2.9) - 2022-05-13

- [`36e352e`](https://github.com/specdown/specdown/commit/36e352ed0507d1120e8b5599e232635717702788) Merge pull request #147 from specdown/dependabot/cargo/indoc-1.0.6
- [`5fe3bec`](https://github.com/specdown/specdown/commit/5fe3becef72ab4fadd341ed847c38e742e41c376) Merge pull request #138 from specdown/dependabot/github_actions/actions/cache-3.0.2
- [`2f37051`](https://github.com/specdown/specdown/commit/2f37051b557b83ef332829a58ae9e71fa2881849) Merge pull request #137 from specdown/dependabot/github_actions/actions/download-artifact-3
- [`c0bd36f`](https://github.com/specdown/specdown/commit/c0bd36f862f669aa3a596d25d79835c2cbdf3ae7) Merge pull request #136 from specdown/dependabot/github_actions/actions/upload-artifact-3
- [`d1d8973`](https://github.com/specdown/specdown/commit/d1d89739f877af472ce2fa03c806759bcef6d7f3) fix: bump indoc from 1.0.4 to 1.0.6
- [`0b77b38`](https://github.com/specdown/specdown/commit/0b77b382171b9290c5d9e2d1a90eb7eb15d950e8) ci: bump actions/cache from 2.1.7 to 3.0.2
- [`aa5effa`](https://github.com/specdown/specdown/commit/aa5effa48dc4036deeca144856922994a395b63e) ci: bump actions/download-artifact from 2 to 3
- [`c322805`](https://github.com/specdown/specdown/commit/c3228059ce4f0051f0c866e5b6a33257bd35936f) ci: bump actions/upload-artifact from 2 to 3
- [`7c61fec`](https://github.com/specdown/specdown/commit/7c61fec7d48397c4b92c45cc42d23c41822c9f5a) Merge pull request #135 from specdown/dependabot/cargo/crossterm-0.23.2
- [`a32cc2d`](https://github.com/specdown/specdown/commit/a32cc2dc6da304cc1cacd9632cb71196a9d1e181) fix: bump crossterm from 0.23.1 to 0.23.2
- [`0b70f20`](https://github.com/specdown/specdown/commit/0b70f20b83d26fc62354e4ac3e042c1fbea4e873) ci: Disable uplift --dry-run on PRs (for now)
- [`d9f5cd4`](https://github.com/specdown/specdown/commit/d9f5cd47ed77962701c0f3f1d44364ee517e2307) ci: Add --fetch-all to uplift release

## [v1.2.8](https://github.com/specdown/specdown/releases/tag/v1.2.8) - 2022-05-13

- [`8c7d240`](https://github.com/specdown/specdown/commit/8c7d240dcdbf2542d4f4bf9339777462b1ec895e) fix: Re-tag commit
- [`8daabec`](https://github.com/specdown/specdown/commit/8daabec159c6c68ce64a3c76683d09e4dd11c894) ci(uplift): uplifted for version v1.2.7
- [`651b9ac`](https://github.com/specdown/specdown/commit/651b9ac19deebb993a5f16240b299efec7b14b1d) ci(uplift): uplifted for version v1.2.7

## [v1.2.7](https://github.com/specdown/specdown/releases/tag/v1.2.7) - 2022-05-13

- [`25d0e7e`](https://github.com/specdown/specdown/commit/25d0e7ea064c03e2eb9addb03b1ef84392030b9a) fix: Removed unnecessary v prefixes
- [`3f6a176`](https://github.com/specdown/specdown/commit/3f6a176c94fd5e88bc3202374bb351c91a4de6ae) ci(uplift): uplifted for version v1.2.6
- [`44b130b`](https://github.com/specdown/specdown/commit/44b130b7e276484539430ba980a3560baac23945) ci(uplift): uplifted for version v1.2.6

## [v1.2.6](https://github.com/specdown/specdown/releases/tag/v1.2.6) - 2022-05-13

- [`33a9945`](https://github.com/specdown/specdown/commit/33a9945288cb19d26defa8d85c6a04fc05822cf1) fix: Revert version in Cargo to 1.2.5
- [`483680a`](https://github.com/specdown/specdown/commit/483680a87e62bda452a3a3bda5df41b163c86f1a) fix: Revert version to 1.2.5
- [`22e76d8`](https://github.com/specdown/specdown/commit/22e76d8e2331799e40c333a7f159f97d8033fbc3) fix: Push the tags
- [`9f3540d`](https://github.com/specdown/specdown/commit/9f3540d4ed53d170d9a1d5827de0c470f98c15a8) ci(uplift): uplifted for version v1.2.6
- [`a12de67`](https://github.com/specdown/specdown/commit/a12de67fb05f667cc940b516b367bc94f76924b9) fix: Setup git config
- [`033d947`](https://github.com/specdown/specdown/commit/033d947acf0c738f936b92255751a61b73a69162) fix: Update the lock file after release
- [`64f2d84`](https://github.com/specdown/specdown/commit/64f2d84198a3fb1ddfa5c982bba408f34b855b6c) ci: Add debug code
- [`2d4f5ba`](https://github.com/specdown/specdown/commit/2d4f5ba16a61a8e8970d019020118dfe476aa232) fix: Debug pipeline
- [`4b0c516`](https://github.com/specdown/specdown/commit/4b0c5161faed8fe4594ffd1086f8f3bd92b3d5b4) fix: Display output from tag

## [v1.2.5](https://github.com/specdown/specdown/releases/tag/v1.2.5) - 2022-05-13

- [`ffc2ff1`](https://github.com/specdown/specdown/commit/ffc2ff1dc7c1b128fdfffb67f2ee28aed76f15e2) fix: Fix fetching of current version
- [`b465e88`](https://github.com/specdown/specdown/commit/b465e881ebdbdaade81289eb017713d59916c24f) fix: Fix casing in Cargo file path
- [`9db4d81`](https://github.com/specdown/specdown/commit/9db4d8178dde3ef82f344b91da8a19f0070579ef) fix: trigger release
- [`5989e53`](https://github.com/specdown/specdown/commit/5989e53f2aed342493f8e3e914cc79b2e9718b43) docs: Fix version number
- [`58fd558`](https://github.com/specdown/specdown/commit/58fd5581d3056c249ad9639824c988ff5775bf55) ci: Add uplift to workflow
- [`0e1d841`](https://github.com/specdown/specdown/commit/0e1d8416b8eb92108bc63c8a09b640b470fab5a0) ci: Add uplift config
