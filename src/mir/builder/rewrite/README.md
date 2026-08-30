# rewrite — explicit special-method handling

この module は、source の意味を再解決する場所ではなく、既存の typed
`Method(Some(receiver))` を補助する明示的な特殊規則だけを扱う。

## 現在の責務

- `special.rs` の `toString` / `stringify` / `str` early route。
- early route は既存の `RuntimeDataBox` method terminal を一度だけ発行する。
- それ以外の method は unified emitter の既存 resolver/materializer/terminal に渡す。

## 退役済み

`MIR-CALL-SAME-MODULE-REWRITE-KNOWN-POLICY-RETIRE-I0` により、任意の
Known/Unique instance-to-Global rewrite、`equals/1` の特別 rewrite、header/suffix
lookup、次の selector は削除された。

- `try_known_*` / `try_unique_suffix_*` / `try_known_or_unique_*`
- `try_special_equals_*`
- `NYASH_REWRITE_KNOWN_DEFAULT`
- `NYASH_BUILDER_REWRITE_INSTANCE`
- `NYASH_DEV_REWRITE_USERBOX`
- `NYASH_DEV_REWRITE_NEW_ORIGIN`

caller-zero の no-destination facade と合わせて、これらは canonical target authority
ではない。generated name、name/arity、header、
suffix、`StaticMethodId`、env、trace、fixture から target を再構築してはならない。

## 保持する不変条件

- exact receiver を持つ typed `Method(Some(receiver))` が method の唯一の通常経路。
- `toString` 系の early route と ArrayWrite / BoxCall の既存 alternate は維持する。
- rewrite 退役で新しい resolver、fallback、retry、Global issuer、MIR schema は作らない。
