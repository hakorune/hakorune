# route_entry

`registry` は現行の source route の診断が使う述語を持つ。

旧 JoinIR router と winner から MIR までの専用経路は退役した。
checked source route を選べない raw Loop は
`raw-loop-child-entry/checked-source-route-unavailable` で拒否する。
古い phase 文書の `joinir/patterns/` は当時の配置を表す。
