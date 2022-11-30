# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.6.0](https://github.com/Loewetiger/OvenMitts/compare/v0.5.0..v0.6.0) - 2022-11-30
#### Refactoring
- port the codebase to axum - ([b9a8512](https://github.com/Loewetiger/OvenMitts/commit/b9a8512a34bc0f2d6f3911aab08e5065e2fd832c)) - [@Loewetiger](https://github.com/Loewetiger)

- - -

## [v0.5.0](https://github.com/Loewetiger/OvenMitts/compare/v0.4.0..v0.5.0) - 2022-11-22
#### Features
- make better use of new error type in "/update" route - ([149094e](https://github.com/Loewetiger/OvenMitts/commit/149094e20867d80e3497286f163f5b722cfe564e)) - [@Loewetiger](https://github.com/Loewetiger)
- derive clone for `User` - ([c67c102](https://github.com/Loewetiger/OvenMitts/commit/c67c1024e1042eae09cac479ed9e0c1f418a3123)) - [@Loewetiger](https://github.com/Loewetiger)
#### Refactoring
- improved error handling - ([58c1e09](https://github.com/Loewetiger/OvenMitts/commit/58c1e0959fe99d9802e61f954a4146d26656c276)) - [@Loewetiger](https://github.com/Loewetiger)

- - -

## [v0.4.0](https://github.com/Loewetiger/OvenMitts/compare/v0.3.0..v0.4.0) - 2022-11-21
#### Bug Fixes
- disable the default features of the chrono crate, thus avoiding using the `time 0.1.44` crate - ([288316d](https://github.com/Loewetiger/OvenMitts/commit/288316dc35353ed202954e53228f14f93e104952)) - [@Loewetiger](https://github.com/Loewetiger)
#### Continuous Integration
- run action on new tags and tag docker image accordingly - ([c080760](https://github.com/Loewetiger/OvenMitts/commit/c080760fec64f2c1061ee59a768182122af85b5e)) - [@Loewetiger](https://github.com/Loewetiger)
#### Features
- allow admin to always overwrite password - ([80ceab0](https://github.com/Loewetiger/OvenMitts/commit/80ceab0048c72ef600d81625cddc3958480f9e3d)) - [@Loewetiger](https://github.com/Loewetiger)
- add route to list all users - ([c0af2f6](https://github.com/Loewetiger/OvenMitts/commit/c0af2f689777e8a58f1d36a7d59bad79545c423d)) - [@Loewetiger](https://github.com/Loewetiger)
- add route to update user attributes - ([3d90a1a](https://github.com/Loewetiger/OvenMitts/commit/3d90a1a1527fd893b22b23da28c48d36ab587b51)) - [@Loewetiger](https://github.com/Loewetiger)
#### Miscellaneous Chores
- update sqlx-data - ([53f6ea3](https://github.com/Loewetiger/OvenMitts/commit/53f6ea3d493adbf6d2a2234a4122c8e39c4358d9)) - [@Loewetiger](https://github.com/Loewetiger)
#### Refactoring
- avoid unnecessary clone - ([85b61b4](https://github.com/Loewetiger/OvenMitts/commit/85b61b4236743f1e489b04d8bf7d7bbca0511e92)) - [@Loewetiger](https://github.com/Loewetiger)

- - -

## [v0.3.0](https://github.com/Loewetiger/OvenMitts/compare/v0.2.0..v0.3.0) - 2022-11-16
#### Bug Fixes
- **(api)** add `stream_title` to SendableUser - ([0ed5aae](https://github.com/Loewetiger/OvenMitts/commit/0ed5aae14636157aa4026d0c6cc089f3a61717b3)) - [@Loewetiger](https://github.com/Loewetiger)
- allow "/streams" endpoint to return the proper values - ([8e27458](https://github.com/Loewetiger/OvenMitts/commit/8e27458c1ee8effcaa1f6d150f98147efaa76d05)) - [@Loewetiger](https://github.com/Loewetiger)
- mount user route - ([444e757](https://github.com/Loewetiger/OvenMitts/commit/444e7571aca56fce5ef0488bb55fa5fbc36f2edb)) - [@Loewetiger](https://github.com/Loewetiger)
#### Build system
- **(cargo)** add libraries for making request and embedding files - ([36f0a97](https://github.com/Loewetiger/OvenMitts/commit/36f0a97fb4884ed07800da3e1312175beac82c33)) - [@Loewetiger](https://github.com/Loewetiger)
- **(cargo)** update SQLx to 0.6, update Rocket to master - ([8431e27](https://github.com/Loewetiger/OvenMitts/commit/8431e2755135e229d678f25a93e62d5dff682f30)) - [@Loewetiger](https://github.com/Loewetiger)
- **(docker)** change default config path - ([7c6b040](https://github.com/Loewetiger/OvenMitts/commit/7c6b0405371ed59a96f1107aeca0c43e16ca20d7)) - [@Loewetiger](https://github.com/Loewetiger)
- **(docker)** add Dockerfile - ([40dadc3](https://github.com/Loewetiger/OvenMitts/commit/40dadc3ec9504e9c75aa0db67663fe3e9af5fe6c)) - [@Loewetiger](https://github.com/Loewetiger)
- add webapp as submodule - ([f0076fd](https://github.com/Loewetiger/OvenMitts/commit/f0076fdd6155af9c865fa34a60803ca1aa6ffb2a)) - [@Loewetiger](https://github.com/Loewetiger)
- build webapp in Dockerfile - ([65ce81b](https://github.com/Loewetiger/OvenMitts/commit/65ce81b59d7f2d7f686e3606080719b293686a1d)) - [@Loewetiger](https://github.com/Loewetiger)
- update sqlx-data - ([d6ae8b3](https://github.com/Loewetiger/OvenMitts/commit/d6ae8b3d6f5219502f558f4161ef520be531bd83)) - [@Loewetiger](https://github.com/Loewetiger)
- cache the cargo registry for faster compilation - ([9c5620a](https://github.com/Loewetiger/OvenMitts/commit/9c5620a645f525d9e90ab6003ea94298fadb6617)) - [@Loewetiger](https://github.com/Loewetiger)
- update database - ([d4cd1b1](https://github.com/Loewetiger/OvenMitts/commit/d4cd1b199a342532aa7d65aef14e5707ae0175ec)) - [@Loewetiger](https://github.com/Loewetiger)
- update SQLx offline build - ([59a03fe](https://github.com/Loewetiger/OvenMitts/commit/59a03fe6aff2888494653553b7d06e77d72f4cd7)) - [@Loewetiger](https://github.com/Loewetiger)
- enable offline builds for SQLx - ([8d14a80](https://github.com/Loewetiger/OvenMitts/commit/8d14a80f9b9960e5c22b92f7fb2cffda7737e025)) - [@Loewetiger](https://github.com/Loewetiger)
#### Continuous Integration
- create main.yml - ([e0085cf](https://github.com/Loewetiger/OvenMitts/commit/e0085cf3a0f986dbfd4ff062c4c529694d0572dd)) - Leon Hajdari
#### Documentation
- add missing docs - ([5ba7835](https://github.com/Loewetiger/OvenMitts/commit/5ba7835e185e78aad5227ceb5d5f34ee416082ff)) - [@Loewetiger](https://github.com/Loewetiger)
- add documentation - ([c492b52](https://github.com/Loewetiger/OvenMitts/commit/c492b52ba97e7ca95f43cb6b8bb8dec18faa68e4)) - [@Loewetiger](https://github.com/Loewetiger)
- add missing documentation - ([52fa55f](https://github.com/Loewetiger/OvenMitts/commit/52fa55f568d94908c64e9f7a5f425e8bf48ec5ba)) - [@Loewetiger](https://github.com/Loewetiger)
- add missing documentation and formatting - ([ea4c16c](https://github.com/Loewetiger/OvenMitts/commit/ea4c16cf8a1a2e6716abf51ce081c4c12ebcc00a)) - [@Loewetiger](https://github.com/Loewetiger)
#### Features
- **(api)** add route to see current streams - ([cbe0fe0](https://github.com/Loewetiger/OvenMitts/commit/cbe0fe04999a8fc1e5a95baeafaf3bd62fcac79d)) - [@Loewetiger](https://github.com/Loewetiger)
- **(interface)** add ability to serve static files - ([bc5dc53](https://github.com/Loewetiger/OvenMitts/commit/bc5dc5355ed0e13a8e7bed167e78dd2ea6ff4024)) - [@Loewetiger](https://github.com/Loewetiger)
- add config option for base and websocket url - ([704faba](https://github.com/Loewetiger/OvenMitts/commit/704fabad387a2446e3ca1f7d88d2f6076fdb847a)) - [@Loewetiger](https://github.com/Loewetiger)
- verify if the chosen username contains invalid characters - ([9c3a889](https://github.com/Loewetiger/OvenMitts/commit/9c3a889961c7a395520e89184e61699516ad74ea)) - [@Loewetiger](https://github.com/Loewetiger)
- add config fairing and new routes - ([7bca97c](https://github.com/Loewetiger/OvenMitts/commit/7bca97c757ef48460cd82a06113bd548277ff6a9)) - [@Loewetiger](https://github.com/Loewetiger)
- add config struct and wrapper for reqwest errors - ([ee4112f](https://github.com/Loewetiger/OvenMitts/commit/ee4112f14c6cb59c64ad4adc6053710a2670d7f2)) - [@Loewetiger](https://github.com/Loewetiger)
- add error message when registering if user exists already - ([02d113e](https://github.com/Loewetiger/OvenMitts/commit/02d113ef21aca9ffdf4af28f0c0fb8a28cac3d1a)) - [@Loewetiger](https://github.com/Loewetiger)
- add function to check if a username is already in use - ([a5ab4e6](https://github.com/Loewetiger/OvenMitts/commit/a5ab4e63b37984f91d644d408f1d28d54d4e2002)) - [@Loewetiger](https://github.com/Loewetiger)
- create `Db` type for functions that borrow a database connection - ([25ae45f](https://github.com/Loewetiger/OvenMitts/commit/25ae45f9bcc82a0ec792fc73d4a19288f6ed9091)) - [@Loewetiger](https://github.com/Loewetiger)
- add registration route - ([fac7674](https://github.com/Loewetiger/OvenMitts/commit/fac7674b39e13b0eb3bfcbef4ebc797388de3137)) - [@Loewetiger](https://github.com/Loewetiger)
- check for stream permission - ([43ea568](https://github.com/Loewetiger/OvenMitts/commit/43ea5684994e40466cafe0077c6e20543a191328)) - [@Loewetiger](https://github.com/Loewetiger)
- add default database path - ([58d0b11](https://github.com/Loewetiger/OvenMitts/commit/58d0b11e6da54d618d92e369ffcaa697c8d1a4c5)) - [@Loewetiger](https://github.com/Loewetiger)
- add struct for receiving login credentials - ([7d74d4c](https://github.com/Loewetiger/OvenMitts/commit/7d74d4c5301b3990659427cfcbfb6253ebb2e72d)) - [@Loewetiger](https://github.com/Loewetiger)
- add password hashing - ([5615056](https://github.com/Loewetiger/OvenMitts/commit/5615056589a8abd2de6bc653a3cdc2612ae33d05)) - [@Loewetiger](https://github.com/Loewetiger)
- add query to get user by username, fix db type - ([27b8de3](https://github.com/Loewetiger/OvenMitts/commit/27b8de3340c4206f266bd581ffb2fc7f9b52ad6e)) - [@Loewetiger](https://github.com/Loewetiger)
- add login and logout routes - ([8cabda6](https://github.com/Loewetiger/OvenMitts/commit/8cabda633bbd5a57f772a43d0d0f719f42c5ce57)) - [@Loewetiger](https://github.com/Loewetiger)
- implement permissions based on rocket-grants - ([2ffcc39](https://github.com/Loewetiger/OvenMitts/commit/2ffcc39c87fe92dd4babd412a7b7292db68a0f47)) - [@Loewetiger](https://github.com/Loewetiger)
- add 'SendableUser' meant for use in frontend - ([4755899](https://github.com/Loewetiger/OvenMitts/commit/47558995712f412c99a2ac809a7f3b0fb38a4db1)) - [@Loewetiger](https://github.com/Loewetiger)
- add new modules, warn on missing documentation - ([c56a2b0](https://github.com/Loewetiger/OvenMitts/commit/c56a2b00c189a17faae2a8b70de8760ae11f27d6)) - [@Loewetiger](https://github.com/Loewetiger)
- add SQL query for getting users based on sessions - ([be935d1](https://github.com/Loewetiger/OvenMitts/commit/be935d1ddae4db426f91ee76ea0d9185ce312ed4)) - [@Loewetiger](https://github.com/Loewetiger)
- add route to get user information - ([2b9b56b](https://github.com/Loewetiger/OvenMitts/commit/2b9b56b00769b7b96e14630908e4543ff2b1e73c)) - [@Loewetiger](https://github.com/Loewetiger)
- add auth guard - ([dab17ee](https://github.com/Loewetiger/OvenMitts/commit/dab17ee13c43378c8563e7e557c1a825c6560e49)) - [@Loewetiger](https://github.com/Loewetiger)
- add user struct - ([b17f543](https://github.com/Loewetiger/OvenMitts/commit/b17f543b1e933215bf348174cfffe66427486e3e)) - [@Loewetiger](https://github.com/Loewetiger)
- finish admission logic - ([49c2abd](https://github.com/Loewetiger/OvenMitts/commit/49c2abd49b816a14f2fc8aac351af65917a4a24b)) - [@Loewetiger](https://github.com/Loewetiger)
- import new modules - ([e303a45](https://github.com/Loewetiger/OvenMitts/commit/e303a45f5a245967a4133a8814762fda8e5e759d)) - [@Loewetiger](https://github.com/Loewetiger)
- add logic to verify whether admission is allowed or denied - ([853d611](https://github.com/Loewetiger/OvenMitts/commit/853d611319df150188d8e35ef67e107e155e8b5d)) - [@Loewetiger](https://github.com/Loewetiger)
- add structs for JSON compat - ([d825c55](https://github.com/Loewetiger/OvenMitts/commit/d825c553bddc29993c8c43b3d4f268e7ba230c57)) - [@Loewetiger](https://github.com/Loewetiger)
- add url crate - ([bbd8b2d](https://github.com/Loewetiger/OvenMitts/commit/bbd8b2d22cf597677141cd14125b530dbb1ecb1c)) - [@Loewetiger](https://github.com/Loewetiger)
- add database functionality - ([468b681](https://github.com/Loewetiger/OvenMitts/commit/468b6819518c956983bdef70b1bd5904cc2378ed)) - [@Loewetiger](https://github.com/Loewetiger)
- initial hello world webserver - ([25e31c3](https://github.com/Loewetiger/OvenMitts/commit/25e31c33e10b9ae1829e6faa460d5a8da015b5f2)) - [@Loewetiger](https://github.com/Loewetiger)
#### Miscellaneous Chores
- add changelog generation based on cocogitto - ([601c514](https://github.com/Loewetiger/OvenMitts/commit/601c514efa4dc30e52a587926a94fef68d70db71)) - [@Loewetiger](https://github.com/Loewetiger)
- fix gitignore - ([a35e23f](https://github.com/Loewetiger/OvenMitts/commit/a35e23fe270593f1f762d767eebbab287ff108d2)) - [@Loewetiger](https://github.com/Loewetiger)
- exclude interface and config files - ([e52c8f6](https://github.com/Loewetiger/OvenMitts/commit/e52c8f655b5628a3f00fc090c3ff8474dc13e48d)) - [@Loewetiger](https://github.com/Loewetiger)
- update CHANGELOG.md version - ([c485764](https://github.com/Loewetiger/OvenMitts/commit/c4857649c6ff9a83803ab105a5cafa1db651ccc5)) - [@Loewetiger](https://github.com/Loewetiger)
- create CHANGELOG.md - ([c8a2403](https://github.com/Loewetiger/OvenMitts/commit/c8a2403ea15f606f0d973d51019295d88494d7b0)) - [@Loewetiger](https://github.com/Loewetiger)
- update ovenmitts version - ([d533e58](https://github.com/Loewetiger/OvenMitts/commit/d533e58e0039bbf77d8329211b225a51ab9c3096)) - [@Loewetiger](https://github.com/Loewetiger)
- remove flake.nix, as it currently is not working - ([1a78f0a](https://github.com/Loewetiger/OvenMitts/commit/1a78f0a5017b2a7b54b47965081d83406afc3b32)) - [@Loewetiger](https://github.com/Loewetiger)
- remove Cargo.lock from gitignore and check it into Git - ([f5c94de](https://github.com/Loewetiger/OvenMitts/commit/f5c94deff3f47eb4aceaf697346c64adf2db9be9)) - [@Loewetiger](https://github.com/Loewetiger)
- add README - ([7190fcd](https://github.com/Loewetiger/OvenMitts/commit/7190fcdca075d18cc322628646d4f150f3dc8b86)) - [@Loewetiger](https://github.com/Loewetiger)
- update gitignore - ([0ebf607](https://github.com/Loewetiger/OvenMitts/commit/0ebf6073fbc33a833026a2c28bbed046b1d1145c)) - [@Loewetiger](https://github.com/Loewetiger)
#### Refactoring
- **(auth)** implement FromRequest for the `User` struct - ([f2e8fb9](https://github.com/Loewetiger/OvenMitts/commit/f2e8fb961103899a9457aca7616ed346a9b07c25)) - [@Loewetiger](https://github.com/Loewetiger)
- **(config)** change Mitts.toml to lowercase and allow use of MITTS_CONFIG - ([6a00d63](https://github.com/Loewetiger/OvenMitts/commit/6a00d630d9272030ccf080a9907dcf0471ae9cdf)) - [@Loewetiger](https://github.com/Loewetiger)
- **(database)** add additional "display_name" column - ([e0da9e7](https://github.com/Loewetiger/OvenMitts/commit/e0da9e7e0cb4f628315bd4f57a3f551aff7dcc58)) - [@Loewetiger](https://github.com/Loewetiger)
- switch to private cookies for session management - ([8f0af20](https://github.com/Loewetiger/OvenMitts/commit/8f0af2042d7f0e3a21053eb2c023bf73daa8d0fb)) - [@Loewetiger](https://github.com/Loewetiger)
- remove `rocket_grants` - ([035accc](https://github.com/Loewetiger/OvenMitts/commit/035accc8984c248b7a6821195f7b7e4cf4030e86)) - [@Loewetiger](https://github.com/Loewetiger)
- update structs to make use of display_name - ([8b66abc](https://github.com/Loewetiger/OvenMitts/commit/8b66abcbbeb3dc47bcd8ad03596615b2d061afdf)) - [@Loewetiger](https://github.com/Loewetiger)
- disable rocket-grants, as it isn't compatible with rocket master - ([1551f91](https://github.com/Loewetiger/OvenMitts/commit/1551f9129eeb1a4bfcedfd8cd757f8705b2506da)) - [@Loewetiger](https://github.com/Loewetiger)
- remove `AuthGuard` as it was unused - ([c4d22fd](https://github.com/Loewetiger/OvenMitts/commit/c4d22fd87cabbf5a40057eacc55644bd1b740d9c)) - [@Loewetiger](https://github.com/Loewetiger)
- remove login cookies even if database deletion fails - ([4c2dd14](https://github.com/Loewetiger/OvenMitts/commit/4c2dd146ebf1d19048aa7ea7240569d40ae83b3a)) - [@Loewetiger](https://github.com/Loewetiger)
- move the queries that return `User` into an associated function - ([f207ee0](https://github.com/Loewetiger/OvenMitts/commit/f207ee0eda4f4b88b60e1e705824137f799f9cbf)) - [@Loewetiger](https://github.com/Loewetiger)
- change salt generation, add stream key generator - ([55ddbc5](https://github.com/Loewetiger/OvenMitts/commit/55ddbc5e523a4b4049de4cb5405ba570cd139504)) - [@Loewetiger](https://github.com/Loewetiger)
- move permission logic into User struct - ([62dfcda](https://github.com/Loewetiger/OvenMitts/commit/62dfcdad1e2cf428c5d975723b1631b2e31fe906)) - [@Loewetiger](https://github.com/Loewetiger)
- use alternative crate for Argon2 - ([e408686](https://github.com/Loewetiger/OvenMitts/commit/e4086869fee4dc90b2f6e9b3477342140dcf9204)) - [@Loewetiger](https://github.com/Loewetiger)
- switch to grants as permissions - ([381d020](https://github.com/Loewetiger/OvenMitts/commit/381d020bd0c40d561f1d2c9243a5166e1ea40cfc)) - [@Loewetiger](https://github.com/Loewetiger)
- make admission route async - ([1b4017d](https://github.com/Loewetiger/OvenMitts/commit/1b4017dcdcd7faffc82f67f4faab5594d9e55afd)) - [@Loewetiger](https://github.com/Loewetiger)
- replace hello world route with admission - ([555f057](https://github.com/Loewetiger/OvenMitts/commit/555f057be89965a66dcd08b76daf94715a1be816)) - [@Loewetiger](https://github.com/Loewetiger)
#### Style
- apply rust-analyzer formatting - ([78b2bab](https://github.com/Loewetiger/OvenMitts/commit/78b2bab26af5b0e5efc2f998ca3a044f64a93958)) - [@Loewetiger](https://github.com/Loewetiger)
- apply suggestions from clippy - ([d20b666](https://github.com/Loewetiger/OvenMitts/commit/d20b6667363d5b8f46944fb55faabd2df22e9936)) - [@Loewetiger](https://github.com/Loewetiger)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).