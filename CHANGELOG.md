# Changelog

## [0.2.0](https://github.com/schemabound/enterprise/compare/schemabound-proto-v0.1.0...schemabound-proto-v0.2.0) (2026-04-25)


### ⚠ BREAKING CHANGES

* rebrand oamrs/roam to schemabound

### Features

* rebrand oamrs/roam to schemabound ([8c1c46b](https://github.com/schemabound/enterprise/commit/8c1c46b6b3624458be1385a142f604d3a64a30b4))


### Bug Fixes

* rename roam_dev→schemabound_dev in db_test/api_test macros and reset package versions to 0.1.0 ([baf92a9](https://github.com/schemabound/enterprise/commit/baf92a994055a2573582b7b788e7bd4833277fb1))

## [0.5.0](https://github.com/schemabound/schemabound/compare/schemabound-proto-v0.4.0...schemabound-proto-v0.5.0) (2026-04-21)


### Features

* **control-plane:** add multi-step workflow orchestration ([b4aacb9](https://github.com/schemabound/schemabound/commit/b4aacb94b284462eb75c12b387293eed9a2d9f91))
* **control-plane:** add multi-step workflow orchestration ([38548d5](https://github.com/schemabound/schemabound/commit/38548d55326963c4f57d3568e89477258f408011))

## [0.4.0](https://github.com/schemabound/schemabound/compare/schemabound-proto-v0.3.0...schemabound-proto-v0.4.0) (2026-04-20)


### Features

* ORM metadata introspection + E2E Dolt branch isolation ([07809ba](https://github.com/schemabound/schemabound/commit/07809ba9f975f8d69b1443e650c07b253be54de9))
* ORM metadata introspection + E2E Dolt branch isolation ([bc33ca6](https://github.com/schemabound/schemabound/commit/bc33ca636809beb43bebc36145dddcc9dbbad8ec))
* RLS/CLS authorization middleware with DoltPolicyProvider ([4796b1d](https://github.com/schemabound/schemabound/commit/4796b1dc649386b07880c2e96e6764d253fa67e9))


### Bug Fixes

* address PR review comments ([4e4e792](https://github.com/schemabound/schemabound/commit/4e4e792fc98df2d6bb21ef470936a342d872443c))

## [0.3.0](https://github.com/schemabound/schemabound/compare/schemabound-proto-v0.2.0...schemabound-proto-v0.3.0) (2026-04-18)


### Features

* **cd:** publish schemabound-proto and schemabound to crates.io on public-v* tag ([d815629](https://github.com/schemabound/schemabound/commit/d815629a43c2a8a703ef1ae69b0872788442c2f1))
* **cd:** publish schemabound-schema, schemabound-proto, schemabound to crates.io on public… ([9c50303](https://github.com/schemabound/schemabound/commit/9c503038460a8dcec5f048c0af5a00d9b21e861c))
* LLM-ready schemas from SeaORM models and gRPC request execution with tonic ([b8ee351](https://github.com/schemabound/schemabound/commit/b8ee35172bd333a66f39a795cbd2c517e6cd5a75))
* **policy:** add semantic P2SQL engine with neutral OSS policy context ([c672aa0](https://github.com/schemabound/schemabound/commit/c672aa0f8459244c00001886e2e853205f27f7bb))
* **python:** replace rust bindings with idiomatic python sdk ([47c1982](https://github.com/schemabound/schemabound/commit/47c19821db68b2b0de825537088f9b0f8cef2743))
* Scaffold SDKs, Hardware lib, and update Docs infrastructure ([3c4f0d5](https://github.com/schemabound/schemabound/commit/3c4f0d502a63c9d9a2fa3f47c5b434b4f47873e5))
* **sdk+proto:** rename SchemaMode enum (DATA_ONLY-&gt;DATA_FIRST, CODE_STRICT-&gt;CODE_FIRST) ([4d4fb32](https://github.com/schemabound/schemabound/commit/4d4fb3228ca5fb9a7a73c27c502a44f03cc3475a))
* **sdk:** add SchemaMode support and SQLAlchemy mixin ([aeccf66](https://github.com/schemabound/schemabound/commit/aeccf66154a401e1c71b06657b1599d3fd3a1673))


### Bug Fixes

* Address code review feedback ([19ac575](https://github.com/schemabound/schemabound/commit/19ac5754511a9190a6bad61d67596a1560300d1b))
* **quality:** Address lints in FFI and proto definition ([08054be](https://github.com/schemabound/schemabound/commit/08054be7bffb8cc4df5922fa2a1d29d47c44d04d))


### Performance Improvements

* **ci:** run quality-checks before tests, collapse test+coverage builds ([ece8887](https://github.com/schemabound/schemabound/commit/ece8887487aabde9935db548da73c0048d36b777))

## [0.2.0](https://github.com/schemabound/schemabound/compare/schemabound-proto-vv0.1.1...schemabound-proto-vv0.2.0) (2026-04-18)


### Features

* **cd:** publish schemabound-proto and schemabound to crates.io on public-v* tag ([d815629](https://github.com/schemabound/schemabound/commit/d815629a43c2a8a703ef1ae69b0872788442c2f1))
* **cd:** publish schemabound-schema, schemabound-proto, schemabound to crates.io on public… ([9c50303](https://github.com/schemabound/schemabound/commit/9c503038460a8dcec5f048c0af5a00d9b21e861c))
* LLM-ready schemas from SeaORM models and gRPC request execution with tonic ([b8ee351](https://github.com/schemabound/schemabound/commit/b8ee35172bd333a66f39a795cbd2c517e6cd5a75))
* **policy:** add semantic P2SQL engine with neutral OSS policy context ([c672aa0](https://github.com/schemabound/schemabound/commit/c672aa0f8459244c00001886e2e853205f27f7bb))
* **python:** replace rust bindings with idiomatic python sdk ([47c1982](https://github.com/schemabound/schemabound/commit/47c19821db68b2b0de825537088f9b0f8cef2743))
* Scaffold SDKs, Hardware lib, and update Docs infrastructure ([3c4f0d5](https://github.com/schemabound/schemabound/commit/3c4f0d502a63c9d9a2fa3f47c5b434b4f47873e5))
* **sdk+proto:** rename SchemaMode enum (DATA_ONLY-&gt;DATA_FIRST, CODE_STRICT-&gt;CODE_FIRST) ([4d4fb32](https://github.com/schemabound/schemabound/commit/4d4fb3228ca5fb9a7a73c27c502a44f03cc3475a))
* **sdk:** add SchemaMode support and SQLAlchemy mixin ([aeccf66](https://github.com/schemabound/schemabound/commit/aeccf66154a401e1c71b06657b1599d3fd3a1673))


### Bug Fixes

* Address code review feedback ([19ac575](https://github.com/schemabound/schemabound/commit/19ac5754511a9190a6bad61d67596a1560300d1b))
* **quality:** Address lints in FFI and proto definition ([08054be](https://github.com/schemabound/schemabound/commit/08054be7bffb8cc4df5922fa2a1d29d47c44d04d))


### Performance Improvements

* **ci:** run quality-checks before tests, collapse test+coverage builds ([ece8887](https://github.com/schemabound/schemabound/commit/ece8887487aabde9935db548da73c0048d36b777))
