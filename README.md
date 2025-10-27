# japanese_law_id
[![crates.io][crates-badge]][crates]
[![Build Status][ci-badge]][ci]

[crates]: https://crates.io/crates/japanese_law_id
[crates-badge]: https://img.shields.io/crates/v/japanese_law_id
[ci]: https://github.com/japanese-law-analysis/japanese_law_id/actions/workflows/ci.yaml
[ci-badge]: https://github.com/japanese-law-analysis/japanese_law_id/actions/workflows/ci.yaml/badge.svg

## 概要

日本政府が策定し，e-gov法令検索などで使用している法令IDの解析などを行うライブラリです．

仕様はここで定められており，本ライブラリもこれに従います．: <https://laws.e-gov.go.jp/file/LawIdNamingConvention.pdf>

## 使用例

```
use japanese_law_id::*;

let s = "505M60001024060";
let law_id = LawId::from_id_str(s).unwrap();
assert_eq!(
    law_id,
    LawId {
        wareki: Wareki::new(Era::Reiwa, 5),
        law_type: LawType::MinistryOrder {
            ministry: Ministry::M6(vec![
                M6Ministry::MinistryOfTheEnvironmentOrdinance,
                M6Ministry::MinistryOfForeignAffairsOrdinance,
                M6Ministry::ReconstructionAgencyOrdinance,
            ]),
            num: 60
        }
    }
);
assert_eq!(law_id.to_id_str(), s);
```

---

[The MIT License](https://github.com/japanese-law-analysis/japanese_law_id/blob/master/LICENSE)

(c) 2025 Naoki Kitano
