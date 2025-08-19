# askit Release Guide

Passo a passo para atualizar e publicar a biblioteca no crates.io.

------------------------------------------------------------------------

## 1. Criar branch de release

``` bash
git checkout -b release/vX.Y.Z
```

## 2. Atualizar código e docs

-   Ajuste código, exemplos em `examples/`, e comentários com
    *doc-tests*.
-   Para exemplos interativos, use `no_run` ou `ignore`.

## 3. Definir versão (SemVer)

-   **PATCH** (`0.1.3`): correções/README/metadados.
-   **MINOR** (`0.2.0`): features compatíveis (sem breaking).
-   **MAJOR** (`1.0.0`): breaking changes.

``` bash
cargo install cargo-edit # uma vez
cargo set-version X.Y.Z
```

## 4. Atualizar CHANGELOG

Liste mudanças por categoria (Added/Changed/Fixed/Docs).

## 5. Checagens locais

``` bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
cargo package
cargo publish --dry-run
```

## 6. Commit + Tag

``` bash
git add -A
git commit -m "release: vX.Y.Z"
git tag -a vX.Y.Z -m "askit vX.Y.Z"
```

## 7. Publicar

``` bash
cargo publish
git push && git push --tags
```

## 8. Pós-publicação

-   Verifique a página do crate e o build no **docs.rs**.

Se precisar, configure no `Cargo.toml`:

``` toml
[package.metadata.docs.rs]
all-features = true
```

Problema sério? Faça:

``` bash
cargo yank --vers X.Y.Z
# corrige e sobe versão PATCH nova (X.Y.Z+1)
```

------------------------------------------------------------------------

## Dicas

-   Mesmo mudanças de README exigem bump de versão (PATCH).
-   Para testar antes de publicar:

``` toml
[patch.crates-io]
askit = { path = "../askit" }
```

-   Para automatizar releases:
    [`cargo-release`](https://github.com/crate-ci/cargo-release).

------------------------------------------------------------------------

## License

MIT
