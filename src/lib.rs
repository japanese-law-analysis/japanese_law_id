#![doc = include_str!("../README.md")]
//!

use kansuji::Kansuji;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 元号
/// 現在の法体系が始まった明治以降を扱う
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Era {
    /// 明治
    Meiji,
    /// 大正
    Taisho,
    /// 昭和
    Showa,
    /// 平成
    Heisei,
    /// 令和
    Reiwa,
}

impl Era {
    /// 開始した年月日を整数で出す
    fn start(self) -> usize {
        match self {
            Self::Meiji => 18681023,
            Self::Taisho => 19120729,
            Self::Showa => 19261225,
            Self::Heisei => 19890108,
            Self::Reiwa => 20190501,
        }
    }

    /// 計算の基点となる開始年 - 1
    /// 和暦は1-indexなので
    fn start_year(self) -> usize {
        match self {
            Self::Meiji => 1867,
            Self::Taisho => 1911,
            Self::Showa => 1925,
            Self::Heisei => 1988,
            Self::Reiwa => 2018,
        }
    }

    /// 終了した年月日を整数で出す
    /// 令和は終了していないため，usize::MAX
    fn end(self) -> usize {
        match self {
            Self::Meiji => 19120728,
            Self::Taisho => 19261224,
            Self::Showa => 19890107,
            Self::Heisei => 20190431,
            Self::Reiwa => usize::MAX,
        }
    }

    /// 文字列から生成
    pub fn from_text(text: &str) -> Option<Self> {
        match text {
            "明治" => Some(Self::Meiji),
            "大正" => Some(Self::Taisho),
            "昭和" => Some(Self::Showa),
            "平成" => Some(Self::Heisei),
            "令和" => Some(Self::Reiwa),
            _ => None,
        }
    }

    /// 文字列を生成
    pub fn to_text(self) -> String {
        match self {
            Self::Meiji => String::from("明治"),
            Self::Taisho => String::from("大正"),
            Self::Showa => String::from("昭和"),
            Self::Heisei => String::from("平成"),
            Self::Reiwa => String::from("令和"),
        }
    }

    /// 明治を1，大正を2としていくナンバリングから生成
    pub fn from_number(n: usize) -> Option<Self> {
        match n {
            1 => Some(Self::Meiji),
            2 => Some(Self::Taisho),
            3 => Some(Self::Showa),
            4 => Some(Self::Heisei),
            5 => Some(Self::Reiwa),
            _ => None,
        }
    }

    /// 明治を1，大正を2としていくナンバリングを生成
    pub fn to_number(self) -> usize {
        match self {
            Self::Meiji => 1,
            Self::Taisho => 2,
            Self::Showa => 3,
            Self::Heisei => 4,
            Self::Reiwa => 5,
        }
    }
}

/// 和暦（平成5年，令和元年など）
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wareki {
    era: Era,
    year: usize,
}

impl Wareki {
    pub fn new(era: Era, year: usize) -> Self {
        Self { era, year }
    }

    /// 西暦からの作成
    pub fn from_ad(year: usize, month: usize, day: usize) -> Self {
        use Era::*;
        let t = year * 10000 + month * 100 + day;
        let (era, year) = if (Meiji.start()..=Meiji.end()).contains(&t) {
            (Meiji, year - Meiji.start_year())
        } else if (Taisho.start()..=Taisho.end()).contains(&t) {
            (Taisho, year - Taisho.start_year())
        } else if (Showa.start()..=Showa.end()).contains(&t) {
            (Showa, year - Showa.start_year())
        } else if (Heisei.start()..=Heisei.end()).contains(&t) {
            (Heisei, year - Heisei.start_year())
        } else if Reiwa.start() <= t {
            (Reiwa, year - Reiwa.start_year())
        } else {
            unreachable!()
        };
        Self { era, year }
    }

    /// 西暦での年を生成
    pub fn to_ad(self) -> usize {
        self.era.start_year() + self.year
    }

    /// 「大正元年」，「平成五年」，「平成5年」，「平成５年」などのテキストから生成
    pub fn from_text(text: &str) -> Option<Self> {
        let re = Regex::new("(?<era>明治|大正|昭和|平成|令和)((?<year_gan>元)|(?<year_kansuji>[一|二|三|四|五|六|七|八|九|十|百]+)|(?<year_num>[1|2|3|4|5|6|7|8|9|0]+)|(?<year_num_zen>[１|２|３|４|５|６|７|８|９|０]+))年").unwrap();
        re.captures(text).and_then(|caps| {
            let era = Era::from_text(&caps["era"]).unwrap();
            if caps.name("year_gan").is_some() {
                Some(Self { era, year: 1 })
            } else if let Some(s) = &caps.name("year_kansuji") {
                let year_k = Kansuji::try_from(s.as_str()).ok();
                let year_opt: Option<u128> = year_k.map(|k| k.into());
                year_opt.map(|year| Self {
                    era,
                    year: year as usize,
                })
            } else if let Some(s) = &caps.name("year_num") {
                let year_opt = s.as_str().parse::<usize>().ok();
                year_opt.map(|year| Self { era, year })
            } else if let Some(s) = &caps.name("year_num_zen") {
                let s = s
                    .as_str()
                    .replace("０", "0")
                    .replace("１", "1")
                    .replace("２", "2")
                    .replace("３", "3")
                    .replace("４", "4")
                    .replace("５", "5")
                    .replace("６", "6")
                    .replace("７", "7")
                    .replace("８", "8")
                    .replace("９", "9");
                let year_opt = s.as_str().parse::<usize>().ok();
                year_opt.map(|year| Self { era, year })
            } else {
                None
            }
        })
    }
}

#[test]
fn check_wareki_parse() {
    assert_eq!(
        Wareki::from_text("大正元年"),
        Some(Wareki {
            era: Era::Taisho,
            year: 1
        })
    );
    assert_eq!(
        Wareki::from_text("大正五年"),
        Some(Wareki {
            era: Era::Taisho,
            year: 5
        })
    );
    assert_eq!(
        Wareki::from_text("大正5年"),
        Some(Wareki {
            era: Era::Taisho,
            year: 5
        })
    );
    assert_eq!(
        Wareki::from_text("大正５年"),
        Some(Wareki {
            era: Era::Taisho,
            year: 5
        })
    );
    assert_eq!(
        Wareki::from_text("昭和十五年"),
        Some(Wareki {
            era: Era::Showa,
            year: 15
        })
    );
    assert_eq!(
        Wareki::from_text("昭和15年"),
        Some(Wareki {
            era: Era::Showa,
            year: 15
        })
    );
    assert_eq!(
        Wareki::from_text("昭和１５年"),
        Some(Wareki {
            era: Era::Showa,
            year: 15
        })
    );
}

/// 日付
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Date {
    // 内部は西暦で管理
    year: usize,
    month: usize,
    day: usize,
}

impl Date {
    /// 和暦からの作成
    pub fn new_wareki(era: Era, year: usize, month: usize, day: usize) -> Self {
        Self {
            year: Wareki::new(era, year).to_ad(),
            month,
            day,
        }
    }

    /// 西暦からの作成
    pub fn new_ad(year: usize, month: usize, day: usize) -> Self {
        Self { year, month, day }
    }

    /// 西暦年の取得
    pub fn get_ad_year(self) -> usize {
        self.year
    }

    /// 和暦年の取得
    pub fn gen_wareki_year(self) -> Wareki {
        Wareki::from_ad(self.year, self.month, self.day)
    }
}

#[test]
fn check_date_gen() {
    let d = Date::new_ad(1923, 06, 20).gen_wareki_year();
    assert_eq!(
        d,
        Wareki {
            era: Era::Taisho,
            year: 12
        }
    )
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s = self.year * 10000 + self.month * 100 + self.day;
        let o = other.year * 10000 + other.month * 100 + other.day;
        s.cmp(&o)
    }
}

/// 法律の立法の種類
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RippouType {
    /// 閣法
    Kakuhou,
    /// 衆議院議員立法
    Syuin,
    /// 参議院議員立法
    Sanin,
}

/// 法律の効力の種類
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LawEfficacy {
    /// 政令
    CabinetOrder,
    /// 法律
    Law,
}

/// 府・省に共通化させる
pub trait MinistryContents: Sized {
    /// 事前に用意されている府・省令のビットに変換する．
    /// <https://laws.e-gov.go.jp/file/LawIdNamingConvention.pdf>の9ページ参照．
    fn to_int(&self) -> usize;
    /// 事前に用意されている府・省令のビットから戻す
    /// <https://laws.e-gov.go.jp/file/LawIdNamingConvention.pdf>の9ページ参照．
    fn from_int(n: usize) -> Option<Self>;
    /// 区分の開始年月日
    fn start() -> Date;
    /// 区分の終了年月日
    fn end() -> Date;
    /// 該当する年代かどうかの判定
    fn applicable(date: Date) -> bool {
        Self::start() <= date && date <= Self::end()
    }
    /// 和暦から該当する年代かどうかの判定
    fn applicable_wareki(wareki: Wareki) -> bool {
        Self::start().year <= wareki.to_ad() && wareki.to_ad() <= Self::end().year
    }
    /// 複数省庁管轄の法令の法令IDを計算する
    fn to_id_str(l: &[Self]) -> String {
        let mut n = 0;
        for u in l.iter().map(|v| v.to_int() as u32) {
            n |= 2_u32.pow(u - 1);
        }
        format!("{:07X}", n)
    }
    /// 複数省庁管轄の法令の法令ID文字列のうち，省庁を表す箇所から担当省庁を計算する
    fn from_id_str(byte_s: &str) -> Result<Vec<Self>, String> {
        let chars = byte_s.chars();
        let mut v = Vec::new();
        for (i, c) in chars.enumerate() {
            if c == '1' {
                let n = 28 - i;
                if let Some(t) = Self::from_int(n) {
                    v.push(t);
                } else {
                    return Err(format!("unexpected flag: {n}"));
                }
            } else if c == '0' {
            } else {
                return Err(format!("unexpected char: {c}"));
            }
        }
        Ok(v)
    }

    /// 「厚生労働省令」や「厚生労働省・農林水産省令」などから導き出す
    fn from_name(name: &str) -> Vec<Self>;
}

/// M1時（1869年7月8日〜1943年10月31日）での府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum M1Ministry {
    /// 閣令
    CabinetOrder,
    /// 宮内省令
    ImperialHouseholdOrdinance,
    /// 大東亜省令
    GreaterEastAsiaMinisterialOrdinance,
    /// 内務省令
    MinistryOfTheInteriorOrdinance,
    /// 司法省令
    MinistryOfJusticeOrdinance,
    /// 外務省令
    MinistryOfForeignAffairsOrdinance,
    /// 大蔵省令
    MinistryOfFinanceOrdinance,
    /// 文部省令
    MinistryOfEducationOrdinance,
    /// 厚生省令
    MinistryOfHealthAndWelfareOrdinance,
    /// 農商務省令
    MinistryOfAgricultureAndCommerceOrdinance,
    /// 商工省令
    MinistryOfCommerceAndIndustryOrdinance,
    /// 鉄道省令
    RailwayMinisterialOrdinance,
    /// 逓信省令
    MinistryOfCommunicationsOrdinance,
    /// 陸軍省令（甲）
    MinistryOfTheArmyOrdinanceA,
    ///海軍省令
    NavyMinisterialOrdinance,
    /// 陸軍省令（乙）
    MinistryOfTheArmyOrdinanceB,
    /// 農林省令
    MinistryOfAgricultureAndForestryOrdinance,
    /// 拓殖務省令
    MinistryOfLandDevelopmentOrdinanceA,
    /// 拓務省令
    MinistryOfLandDevelopmentOrdinanceB,
    /// 農商務省令臨
    MinistryOfAgricultureAndCommerceOrdinanceTemporary,
    /// 司法省令（丙）
    /// 例：明治十九年度以降科料罰金徴収及民事刑事其他一時預リ金取扱方（明治19年4月7日司法省令丙第1号）<https://hourei.ndl.go.jp/#/detail?lawId=0000000249&searchDiv=1&current=1>
    MinistryOfJusticeOrdinanceHei,
}

impl MinistryContents for M1Ministry {
    fn to_int(&self) -> usize {
        use M1Ministry::*;
        match self {
            CabinetOrder => 1,
            ImperialHouseholdOrdinance => 2,
            GreaterEastAsiaMinisterialOrdinance => 3,
            MinistryOfTheInteriorOrdinance => 4,
            MinistryOfJusticeOrdinance => 5,
            MinistryOfForeignAffairsOrdinance => 6,
            MinistryOfFinanceOrdinance => 7,
            MinistryOfEducationOrdinance => 8,
            MinistryOfHealthAndWelfareOrdinance => 9,
            MinistryOfAgricultureAndCommerceOrdinance => 10,
            MinistryOfCommerceAndIndustryOrdinance => 11,
            RailwayMinisterialOrdinance => 12,
            MinistryOfCommunicationsOrdinance => 13,
            MinistryOfTheArmyOrdinanceA => 14,
            NavyMinisterialOrdinance => 15,
            MinistryOfTheArmyOrdinanceB => 16,
            MinistryOfAgricultureAndForestryOrdinance => 17,
            MinistryOfLandDevelopmentOrdinanceA => 18,
            MinistryOfLandDevelopmentOrdinanceB => 19,
            MinistryOfAgricultureAndCommerceOrdinanceTemporary => 20,
            MinistryOfJusticeOrdinanceHei => 21,
        }
    }

    fn from_int(n: usize) -> Option<Self> {
        use M1Ministry::*;
        match n {
            1 => Some(CabinetOrder),
            2 => Some(ImperialHouseholdOrdinance),
            3 => Some(GreaterEastAsiaMinisterialOrdinance),
            4 => Some(MinistryOfTheInteriorOrdinance),
            5 => Some(MinistryOfJusticeOrdinance),
            6 => Some(MinistryOfForeignAffairsOrdinance),
            7 => Some(MinistryOfFinanceOrdinance),
            8 => Some(MinistryOfEducationOrdinance),
            9 => Some(MinistryOfHealthAndWelfareOrdinance),
            10 => Some(MinistryOfAgricultureAndCommerceOrdinance),
            11 => Some(MinistryOfCommerceAndIndustryOrdinance),
            12 => Some(RailwayMinisterialOrdinance),
            13 => Some(MinistryOfCommunicationsOrdinance),
            14 => Some(MinistryOfTheArmyOrdinanceA),
            15 => Some(NavyMinisterialOrdinance),
            16 => Some(MinistryOfTheArmyOrdinanceB),
            17 => Some(MinistryOfAgricultureAndForestryOrdinance),
            18 => Some(MinistryOfLandDevelopmentOrdinanceA),
            19 => Some(MinistryOfLandDevelopmentOrdinanceB),
            20 => Some(MinistryOfAgricultureAndCommerceOrdinanceTemporary),
            21 => Some(MinistryOfJusticeOrdinanceHei),
            _ => None,
        }
    }

    fn start() -> Date {
        Date::new_ad(1869, 7, 8)
    }

    fn end() -> Date {
        Date::new_ad(1943, 10, 31)
    }

    fn from_name(name: &str) -> Vec<Self> {
        let mut v = Vec::new();
        if name.contains("閣") {
            v.push(Self::CabinetOrder)
        }
        if name.contains("宮内省") {
            v.push(Self::ImperialHouseholdOrdinance)
        }
        if name.contains("大東亜省") {
            v.push(Self::GreaterEastAsiaMinisterialOrdinance)
        }
        if name.contains("内務省") {
            v.push(Self::MinistryOfTheInteriorOrdinance)
        }
        if name.contains("司法省") {
            v.push(Self::MinistryOfJusticeOrdinance)
        }
        if name.contains("外務省") {
            v.push(Self::MinistryOfForeignAffairsOrdinance)
        }
        if name.contains("大蔵省") {
            v.push(Self::MinistryOfFinanceOrdinance)
        }
        if name.contains("文部省") {
            v.push(Self::MinistryOfEducationOrdinance)
        }
        if name.contains("厚生省") {
            v.push(Self::MinistryOfHealthAndWelfareOrdinance)
        }
        if name.contains("農商務省") {
            v.push(Self::MinistryOfAgricultureAndCommerceOrdinance)
        }
        if name.contains("商工省") {
            v.push(Self::MinistryOfCommerceAndIndustryOrdinance)
        }
        if name.contains("鉄道省") {
            v.push(Self::RailwayMinisterialOrdinance)
        }
        if name.contains("逓信省") {
            v.push(Self::MinistryOfCommunicationsOrdinance)
        }
        if name.contains("陸軍省") && name.contains("甲") {
            v.push(Self::MinistryOfTheArmyOrdinanceA)
        }
        if name.contains("海軍省") {
            v.push(Self::NavyMinisterialOrdinance)
        }
        if name.contains("陸軍省") && name.contains("乙") {
            v.push(Self::MinistryOfTheArmyOrdinanceB)
        }
        if name.contains("農林省") {
            v.push(Self::MinistryOfAgricultureAndForestryOrdinance)
        }
        if name.contains("拓殖務省") {
            v.push(Self::MinistryOfLandDevelopmentOrdinanceA)
        }
        if name.contains("拓務省") {
            v.push(Self::MinistryOfLandDevelopmentOrdinanceB)
        }
        if name.contains("農商務省") && name.contains("臨") {
            v.push(Self::MinistryOfAgricultureAndCommerceOrdinanceTemporary)
        }
        if name.contains("司法省") && name.contains("丙") {
            v.push(Self::MinistryOfJusticeOrdinanceHei)
        }
        v
    }
}

/// M2時（1943年11月1日〜1945年11月31日）での府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum M2Ministry {
    /// 閣令
    CabinetOrder,
    /// 宮内省令
    ImperialHouseholdOrdinance,
    /// 大東亜省令
    GreaterEastAsiaMinisterialOrdinance,
    /// 内務省令
    MinistryOfTheInteriorOrdinance,
    /// 司法省令
    MinistryOfJusticeOrdinance,
    /// 外務省令
    MinistryOfForeignAffairsOrdinance,
    /// 大蔵省令
    MinistryOfFinanceOrdinance,
    /// 文部省令
    MinistryOfEducationOrdinance,
    /// 厚生省令
    MinistryOfHealthAndWelfareOrdinance,
    /// 農商務省令
    MinistryOfAgricultureAndCommerceOrdinance,
    /// 商工省令
    MinistryOfCommerceAndIndustryOrdinance,
    /// 運輸省令
    MinistryOfTransportOrdinance,
    /// 運輸通信省令
    MinistryOfTransportAndCommunicationsOrdinance,
    /// 陸軍省令（甲）
    MinistryOfTheArmyOrdinanceA,
    ///海軍省令
    NavyMinisterialOrdinance,
    /// 軍需省令
    OrdinanceOfTheMinistryOfMunitions,
    /// 農林省令
    MinistryOfAgricultureAndForestryOrdinance,
}

impl MinistryContents for M2Ministry {
    fn to_int(&self) -> usize {
        use M2Ministry::*;
        match self {
            CabinetOrder => 1,
            ImperialHouseholdOrdinance => 2,
            GreaterEastAsiaMinisterialOrdinance => 3,
            MinistryOfTheInteriorOrdinance => 4,
            MinistryOfJusticeOrdinance => 5,
            MinistryOfForeignAffairsOrdinance => 6,
            MinistryOfFinanceOrdinance => 7,
            MinistryOfEducationOrdinance => 8,
            MinistryOfHealthAndWelfareOrdinance => 9,
            MinistryOfAgricultureAndCommerceOrdinance => 10,
            MinistryOfCommerceAndIndustryOrdinance => 11,
            MinistryOfTransportOrdinance => 12,
            MinistryOfTransportAndCommunicationsOrdinance => 13,
            MinistryOfTheArmyOrdinanceA => 14,
            NavyMinisterialOrdinance => 15,
            OrdinanceOfTheMinistryOfMunitions => 16,
            MinistryOfAgricultureAndForestryOrdinance => 17,
        }
    }

    fn from_int(n: usize) -> Option<Self> {
        use M2Ministry::*;
        match n {
            1 => Some(CabinetOrder),
            2 => Some(ImperialHouseholdOrdinance),
            3 => Some(GreaterEastAsiaMinisterialOrdinance),
            4 => Some(MinistryOfTheInteriorOrdinance),
            5 => Some(MinistryOfJusticeOrdinance),
            6 => Some(MinistryOfForeignAffairsOrdinance),
            7 => Some(MinistryOfFinanceOrdinance),
            8 => Some(MinistryOfEducationOrdinance),
            9 => Some(MinistryOfHealthAndWelfareOrdinance),
            10 => Some(MinistryOfAgricultureAndCommerceOrdinance),
            11 => Some(MinistryOfCommerceAndIndustryOrdinance),
            12 => Some(MinistryOfTransportOrdinance),
            13 => Some(MinistryOfTransportAndCommunicationsOrdinance),
            14 => Some(MinistryOfTheArmyOrdinanceA),
            15 => Some(NavyMinisterialOrdinance),
            16 => Some(OrdinanceOfTheMinistryOfMunitions),
            17 => Some(MinistryOfAgricultureAndForestryOrdinance),
            _ => None,
        }
    }

    fn start() -> Date {
        Date::new_ad(1943, 11, 1)
    }

    fn end() -> Date {
        Date::new_ad(1945, 11, 30)
    }

    fn from_name(name: &str) -> Vec<Self> {
        let mut v = Vec::new();
        if name.contains("閣") {
            v.push(Self::CabinetOrder)
        }
        if name.contains("宮内省") {
            v.push(Self::ImperialHouseholdOrdinance)
        }
        if name.contains("大東亜省") {
            v.push(Self::GreaterEastAsiaMinisterialOrdinance)
        }
        if name.contains("内務省") {
            v.push(Self::MinistryOfTheInteriorOrdinance)
        }
        if name.contains("司法省") {
            v.push(Self::MinistryOfJusticeOrdinance)
        }
        if name.contains("外務省") {
            v.push(Self::MinistryOfForeignAffairsOrdinance)
        }
        if name.contains("大蔵省") {
            v.push(Self::MinistryOfFinanceOrdinance)
        }
        if name.contains("文部省") {
            v.push(Self::MinistryOfEducationOrdinance)
        }
        if name.contains("厚生省") {
            v.push(Self::MinistryOfHealthAndWelfareOrdinance)
        }
        if name.contains("農商務省") {
            v.push(Self::MinistryOfAgricultureAndCommerceOrdinance)
        }
        if name.contains("商工省") {
            v.push(Self::MinistryOfCommerceAndIndustryOrdinance)
        }
        if name.contains("運輸省") {
            v.push(Self::MinistryOfTransportOrdinance)
        }
        if name.contains("運輸通信省") {
            v.push(Self::MinistryOfTransportAndCommunicationsOrdinance)
        }
        if name.contains("陸軍省") && name.contains("甲") {
            v.push(Self::MinistryOfTheArmyOrdinanceA)
        }
        if name.contains("海軍省") {
            v.push(Self::NavyMinisterialOrdinance)
        }
        if name.contains("軍需省") {
            v.push(Self::OrdinanceOfTheMinistryOfMunitions)
        }
        if name.contains("農林省") {
            v.push(Self::MinistryOfAgricultureAndForestryOrdinance)
        }
        v
    }
}

/// M3時（1945年12月1日〜1947年5月2日）での府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum M3Ministry {
    /// 閣令
    CabinetOrder,
    /// 宮内省令
    ImperialHouseholdOrdinance,
    /// 経済安定本部令
    EconomicStabilityHeadquartersOrdinance,
    /// 内務省令
    MinistryOfTheInteriorOrdinance,
    /// 司法省令
    MinistryOfJusticeOrdinance,
    /// 外務省令
    MinistryOfForeignAffairsOrdinance,
    /// 大蔵省令
    MinistryOfFinanceOrdinance,
    /// 文部省令
    MinistryOfEducationOrdinance,
    /// 厚生省令
    MinistryOfHealthAndWelfareOrdinance,
    /// 農林省令
    MinistryOfAgricultureAndForestryOrdinance,
    /// 商工省令
    MinistryOfCommerceAndIndustryOrdinance,
    /// 運輸省令
    MinistryOfTransportOrdinance,
    /// 逓信省令
    MinistryOfCommunicationsOrdinance,
    /// 第一復員省令
    FirstMinisterialOrdinanceForDemobilization,
    /// 第二復員省令
    SecondMinisterialOrdinanceForDemobilization,
    /// 物価庁令
    PriceAgencyOrdinance,
    /// 中央労働委員会規則
    CentralLaborRelationsCommissionRules,
}

impl MinistryContents for M3Ministry {
    fn to_int(&self) -> usize {
        use M3Ministry::*;
        match self {
            CabinetOrder => 1,
            ImperialHouseholdOrdinance => 2,
            EconomicStabilityHeadquartersOrdinance => 3,
            MinistryOfTheInteriorOrdinance => 4,
            MinistryOfJusticeOrdinance => 5,
            MinistryOfForeignAffairsOrdinance => 6,
            MinistryOfFinanceOrdinance => 7,
            MinistryOfEducationOrdinance => 8,
            MinistryOfHealthAndWelfareOrdinance => 9,
            MinistryOfAgricultureAndForestryOrdinance => 10,
            MinistryOfCommerceAndIndustryOrdinance => 11,
            MinistryOfTransportOrdinance => 12,
            MinistryOfCommunicationsOrdinance => 13,
            FirstMinisterialOrdinanceForDemobilization => 14,
            SecondMinisterialOrdinanceForDemobilization => 15,
            PriceAgencyOrdinance => 16,
            CentralLaborRelationsCommissionRules => 21,
        }
    }

    fn from_int(n: usize) -> Option<Self> {
        use M3Ministry::*;
        match n {
            1 => Some(CabinetOrder),
            2 => Some(ImperialHouseholdOrdinance),
            3 => Some(EconomicStabilityHeadquartersOrdinance),
            4 => Some(MinistryOfTheInteriorOrdinance),
            5 => Some(MinistryOfJusticeOrdinance),
            6 => Some(MinistryOfForeignAffairsOrdinance),
            7 => Some(MinistryOfFinanceOrdinance),
            8 => Some(MinistryOfEducationOrdinance),
            9 => Some(MinistryOfHealthAndWelfareOrdinance),
            10 => Some(MinistryOfAgricultureAndForestryOrdinance),
            11 => Some(MinistryOfCommerceAndIndustryOrdinance),
            12 => Some(MinistryOfTransportOrdinance),
            13 => Some(MinistryOfCommunicationsOrdinance),
            14 => Some(FirstMinisterialOrdinanceForDemobilization),
            15 => Some(SecondMinisterialOrdinanceForDemobilization),
            16 => Some(PriceAgencyOrdinance),
            21 => Some(CentralLaborRelationsCommissionRules),
            _ => None,
        }
    }

    fn start() -> Date {
        Date::new_ad(1945, 12, 1)
    }

    fn end() -> Date {
        Date::new_ad(1947, 5, 2)
    }

    fn from_name(name: &str) -> Vec<Self> {
        let mut v = Vec::new();
        if name.contains("閣") {
            v.push(Self::CabinetOrder)
        }
        if name.contains("宮内省") {
            v.push(Self::ImperialHouseholdOrdinance)
        }
        if name.contains("経済安定本部") {
            v.push(Self::EconomicStabilityHeadquartersOrdinance)
        }
        if name.contains("内務省") {
            v.push(Self::MinistryOfTheInteriorOrdinance)
        }
        if name.contains("司法省") {
            v.push(Self::MinistryOfJusticeOrdinance)
        }
        if name.contains("外務省") {
            v.push(Self::MinistryOfForeignAffairsOrdinance)
        }
        if name.contains("大蔵省") {
            v.push(Self::MinistryOfFinanceOrdinance)
        }
        if name.contains("文部省") {
            v.push(Self::MinistryOfEducationOrdinance)
        }
        if name.contains("厚生省") {
            v.push(Self::MinistryOfHealthAndWelfareOrdinance)
        }
        if name.contains("農林省") {
            v.push(Self::MinistryOfAgricultureAndForestryOrdinance)
        }
        if name.contains("商工省") {
            v.push(Self::MinistryOfCommerceAndIndustryOrdinance)
        }
        if name.contains("運輸省") {
            v.push(Self::MinistryOfTransportOrdinance)
        }
        if name.contains("逓信省") {
            v.push(Self::MinistryOfCommunicationsOrdinance)
        }
        if name.contains("第一復員省") {
            v.push(Self::FirstMinisterialOrdinanceForDemobilization)
        }
        if name.contains("第二復員省") {
            v.push(Self::SecondMinisterialOrdinanceForDemobilization)
        }
        if name.contains("物価庁") {
            v.push(Self::PriceAgencyOrdinance)
        }
        if name.contains("中央労働委員会") {
            v.push(Self::CentralLaborRelationsCommissionRules)
        }
        v
    }
}

/// M4時（1947年5月3日〜1949年5月31日）での府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum M4Ministry {
    /// 法務庁令
    LegalAffairsAgencyOrdinance,
    /// 総理庁令
    PrimeMinistersOfficeOrdinance,
    /// 経済安定本部令
    EconomicStabilityHeadquartersOrdinance,
    /// 内務省令
    MinistryOfTheInteriorOrdinance,
    /// 司法省令
    MinistryOfJusticeOrdinance,
    /// 外務省令
    MinistryOfForeignAffairsOrdinance,
    /// 大蔵省令
    MinistryOfFinanceOrdinance,
    /// 文部省令
    MinistryOfEducationOrdinance,
    /// 厚生省令
    MinistryOfHealthAndWelfareOrdinance,
    /// 農林省令
    MinistryOfAgricultureAndForestryOrdinance,
    /// 通商産業省令
    MinistryOfInternationalTradeAndIndustryOrdinance,
    /// 運輸省令
    MinistryOfTransportOrdinance,
    /// 逓信省令
    MinistryOfCommunicationsOrdinance,
    /// 労働省令
    MinistryOfLaborOrdinance,
    /// 建設省令
    MinistryOfConstructionOrdinance,
    /// 物価庁令
    PriceAgencyOrdinance,
    /// 商工省令
    MinistryOfCommerceAndIndustryOrdinance,
    /// 中央労働委員会規則
    CentralLaborRelationsCommissionRules,
    /// 公正取引委員会規則
    FairTradeCommissionRules,
    /// 国家公安委員会規則
    NationalPublicSafetyCommissionRegulations,
}

impl MinistryContents for M4Ministry {
    fn to_int(&self) -> usize {
        use M4Ministry::*;
        match self {
            LegalAffairsAgencyOrdinance => 1,
            PrimeMinistersOfficeOrdinance => 2,
            EconomicStabilityHeadquartersOrdinance => 3,
            MinistryOfTheInteriorOrdinance => 4,
            MinistryOfJusticeOrdinance => 5,
            MinistryOfForeignAffairsOrdinance => 6,
            MinistryOfFinanceOrdinance => 7,
            MinistryOfEducationOrdinance => 8,
            MinistryOfHealthAndWelfareOrdinance => 9,
            MinistryOfAgricultureAndForestryOrdinance => 10,
            MinistryOfInternationalTradeAndIndustryOrdinance => 11,
            MinistryOfTransportOrdinance => 12,
            MinistryOfCommunicationsOrdinance => 13,
            MinistryOfLaborOrdinance => 14,
            MinistryOfConstructionOrdinance => 15,
            PriceAgencyOrdinance => 16,
            MinistryOfCommerceAndIndustryOrdinance => 17,
            CentralLaborRelationsCommissionRules => 21,
            FairTradeCommissionRules => 22,
            NationalPublicSafetyCommissionRegulations => 23,
        }
    }

    fn from_int(n: usize) -> Option<Self> {
        use M4Ministry::*;
        match n {
            1 => Some(LegalAffairsAgencyOrdinance),
            2 => Some(PrimeMinistersOfficeOrdinance),
            3 => Some(EconomicStabilityHeadquartersOrdinance),
            4 => Some(MinistryOfTheInteriorOrdinance),
            5 => Some(MinistryOfJusticeOrdinance),
            6 => Some(MinistryOfForeignAffairsOrdinance),
            7 => Some(MinistryOfFinanceOrdinance),
            8 => Some(MinistryOfEducationOrdinance),
            9 => Some(MinistryOfHealthAndWelfareOrdinance),
            10 => Some(MinistryOfAgricultureAndForestryOrdinance),
            11 => Some(MinistryOfInternationalTradeAndIndustryOrdinance),
            12 => Some(MinistryOfTransportOrdinance),
            13 => Some(MinistryOfCommunicationsOrdinance),
            14 => Some(MinistryOfLaborOrdinance),
            15 => Some(MinistryOfConstructionOrdinance),
            16 => Some(PriceAgencyOrdinance),
            17 => Some(MinistryOfCommerceAndIndustryOrdinance),
            21 => Some(CentralLaborRelationsCommissionRules),
            22 => Some(FairTradeCommissionRules),
            23 => Some(NationalPublicSafetyCommissionRegulations),
            _ => None,
        }
    }

    fn start() -> Date {
        Date::new_ad(1947, 5, 3)
    }

    fn end() -> Date {
        Date::new_ad(1949, 5, 31)
    }

    fn from_name(name: &str) -> Vec<Self> {
        let mut v = Vec::new();
        if name.contains("法務庁") {
            v.push(Self::LegalAffairsAgencyOrdinance)
        }
        if name.contains("総理庁") {
            v.push(Self::PrimeMinistersOfficeOrdinance)
        }
        if name.contains("経済安定本部") {
            v.push(Self::EconomicStabilityHeadquartersOrdinance)
        }
        if name.contains("内務省") {
            v.push(Self::MinistryOfTheInteriorOrdinance)
        }
        if name.contains("司法省") {
            v.push(Self::MinistryOfJusticeOrdinance)
        }
        if name.contains("外務省") {
            v.push(Self::MinistryOfForeignAffairsOrdinance)
        }
        if name.contains("大蔵省") {
            v.push(Self::MinistryOfFinanceOrdinance)
        }
        if name.contains("文部省") {
            v.push(Self::MinistryOfEducationOrdinance)
        }
        if name.contains("厚生省") {
            v.push(Self::MinistryOfHealthAndWelfareOrdinance)
        }
        if name.contains("農林省") {
            v.push(Self::MinistryOfAgricultureAndForestryOrdinance)
        }
        if name.contains("通商産業省") {
            v.push(Self::MinistryOfInternationalTradeAndIndustryOrdinance)
        }
        if name.contains("運輸省") {
            v.push(Self::MinistryOfTransportOrdinance)
        }
        if name.contains("逓信省") {
            v.push(Self::MinistryOfCommunicationsOrdinance)
        }
        if name.contains("労働省") {
            v.push(Self::MinistryOfLaborOrdinance)
        }
        if name.contains("建設省") {
            v.push(Self::MinistryOfConstructionOrdinance)
        }
        if name.contains("物価庁") {
            v.push(Self::PriceAgencyOrdinance)
        }
        if name.contains("商工省") {
            v.push(Self::MinistryOfCommerceAndIndustryOrdinance)
        }
        if name.contains("中央労働委員会") {
            v.push(Self::CentralLaborRelationsCommissionRules)
        }
        if name.contains("公正取引委員会") {
            v.push(Self::FairTradeCommissionRules)
        }
        if name.contains("国家公安委員会") {
            v.push(Self::NationalPublicSafetyCommissionRegulations)
        }
        v
    }
}

/// M5時（1949年6月1日〜2001年1月15日）での府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum M5Ministry {
    /// 法務庁令
    LegalAffairsAgencyOrdinance,
    /// 総理庁令
    PrimeMinistersOfficeOrdinance,
    /// 経済安定本部令
    EconomicStabilityHeadquartersOrdinance,
    /// 自治省令
    MinistryOfHomeAffairsOrdinance,
    /// 法務省令
    MinistryOfJusticeOrdinance,
    /// 外務省令
    MinistryOfForeignAffairsOrdinance,
    /// 大蔵省令
    MinistryOfFinanceOrdinance,
    /// 文部省令
    MinistryOfEducationOrdinance,
    /// 厚生省令
    MinistryOfHealthAndWelfareOrdinance,
    /// 農林水産省令
    MinistryOfAgricultureAndForestryAndFisheriesOrdinance,
    /// 通商産業省令
    MinistryOfInternationalTradeAndIndustryOrdinance,
    /// 運輸省令
    MinistryOfTransportOrdinance,
    /// 郵政省令
    MinistryOfPostsAndTelecommunicationsOrdinance,
    /// 労働省令
    MinistryOfLaborOrdinance,
    /// 建設省令
    MinistryOfConstructionOrdinance,
    /// 物価庁令
    PriceAgencyOrdinance,
    /// 農林省令
    MinistryOfAgricultureAndForestryOrdinance,
    /// 電気通信省令
    TelecommunicationsMinisterialOrdinance,
    /// 中央省庁等改革推進本部令
    CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance,
    /// 電波監理委員会規則
    RadioRegulatoryCommissionRules,
    /// 中央労働委員会規則
    CentralLaborRelationsCommissionRules,
    /// 公正取引委員会規則
    FairTradeCommissionRules,
    /// 国家公安委員会規則
    NationalPublicSafetyCommissionRegulations,
    /// 公害等調整委員会規則
    PollutionAdjustmentCommitteeRules,
    /// 公安審査委員会規則
    PublicSafetyReviewCommitteeRules,
}

impl MinistryContents for M5Ministry {
    fn to_int(&self) -> usize {
        use M5Ministry::*;
        match self {
            LegalAffairsAgencyOrdinance => 1,
            PrimeMinistersOfficeOrdinance => 2,
            EconomicStabilityHeadquartersOrdinance => 3,
            MinistryOfHomeAffairsOrdinance => 4,
            MinistryOfJusticeOrdinance => 5,
            MinistryOfForeignAffairsOrdinance => 6,
            MinistryOfFinanceOrdinance => 7,
            MinistryOfEducationOrdinance => 8,
            MinistryOfHealthAndWelfareOrdinance => 9,
            MinistryOfAgricultureAndForestryAndFisheriesOrdinance => 10,
            MinistryOfInternationalTradeAndIndustryOrdinance => 11,
            MinistryOfTransportOrdinance => 12,
            MinistryOfPostsAndTelecommunicationsOrdinance => 13,
            MinistryOfLaborOrdinance => 14,
            MinistryOfConstructionOrdinance => 15,
            PriceAgencyOrdinance => 16,
            MinistryOfAgricultureAndForestryOrdinance => 17,
            TelecommunicationsMinisterialOrdinance => 18,
            CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance => 19,
            RadioRegulatoryCommissionRules => 20,
            CentralLaborRelationsCommissionRules => 21,
            FairTradeCommissionRules => 22,
            NationalPublicSafetyCommissionRegulations => 23,
            PollutionAdjustmentCommitteeRules => 24,
            PublicSafetyReviewCommitteeRules => 25,
        }
    }

    fn from_int(n: usize) -> Option<Self> {
        use M5Ministry::*;
        match n {
            1 => Some(LegalAffairsAgencyOrdinance),
            2 => Some(PrimeMinistersOfficeOrdinance),
            3 => Some(EconomicStabilityHeadquartersOrdinance),
            4 => Some(MinistryOfHomeAffairsOrdinance),
            5 => Some(MinistryOfJusticeOrdinance),
            6 => Some(MinistryOfForeignAffairsOrdinance),
            7 => Some(MinistryOfFinanceOrdinance),
            8 => Some(MinistryOfEducationOrdinance),
            9 => Some(MinistryOfHealthAndWelfareOrdinance),
            10 => Some(MinistryOfAgricultureAndForestryAndFisheriesOrdinance),
            11 => Some(MinistryOfInternationalTradeAndIndustryOrdinance),
            12 => Some(MinistryOfTransportOrdinance),
            13 => Some(MinistryOfPostsAndTelecommunicationsOrdinance),
            14 => Some(MinistryOfLaborOrdinance),
            15 => Some(MinistryOfConstructionOrdinance),
            16 => Some(PriceAgencyOrdinance),
            17 => Some(MinistryOfAgricultureAndForestryOrdinance),
            18 => Some(TelecommunicationsMinisterialOrdinance),
            19 => Some(CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance),
            20 => Some(RadioRegulatoryCommissionRules),
            21 => Some(CentralLaborRelationsCommissionRules),
            22 => Some(FairTradeCommissionRules),
            23 => Some(NationalPublicSafetyCommissionRegulations),
            24 => Some(PollutionAdjustmentCommitteeRules),
            25 => Some(PublicSafetyReviewCommitteeRules),
            _ => None,
        }
    }

    fn start() -> Date {
        Date::new_ad(1949, 6, 1)
    }

    fn end() -> Date {
        Date::new_ad(2001, 1, 5)
    }

    fn from_name(name: &str) -> Vec<Self> {
        let mut v = Vec::new();
        if name.contains("法務庁") {
            v.push(Self::LegalAffairsAgencyOrdinance)
        }
        if name.contains("総理庁") {
            v.push(Self::PrimeMinistersOfficeOrdinance)
        }
        if name.contains("経済安定本部") {
            v.push(Self::EconomicStabilityHeadquartersOrdinance)
        }
        if name.contains("自治省") {
            v.push(Self::MinistryOfHomeAffairsOrdinance)
        }
        if name.contains("法務省") {
            v.push(Self::MinistryOfJusticeOrdinance)
        }
        if name.contains("外務省") {
            v.push(Self::MinistryOfForeignAffairsOrdinance)
        }
        if name.contains("大蔵省") {
            v.push(Self::MinistryOfFinanceOrdinance)
        }
        if name.contains("文部省") {
            v.push(Self::MinistryOfEducationOrdinance)
        }
        if name.contains("厚生省") {
            v.push(Self::MinistryOfHealthAndWelfareOrdinance)
        }
        if name.contains("農林水産省") {
            v.push(Self::MinistryOfAgricultureAndForestryAndFisheriesOrdinance)
        }
        if name.contains("通商産業省") {
            v.push(Self::MinistryOfInternationalTradeAndIndustryOrdinance)
        }
        if name.contains("運輸省") {
            v.push(Self::MinistryOfTransportOrdinance)
        }
        if name.contains("郵政省") {
            v.push(Self::MinistryOfPostsAndTelecommunicationsOrdinance)
        }
        if name.contains("労働省") {
            v.push(Self::MinistryOfLaborOrdinance)
        }
        if name.contains("建設省") {
            v.push(Self::MinistryOfConstructionOrdinance)
        }
        if name.contains("物価庁") {
            v.push(Self::PriceAgencyOrdinance)
        }
        if name.contains("農林省") {
            v.push(Self::MinistryOfAgricultureAndForestryOrdinance)
        }
        if name.contains("電気通信省") {
            v.push(Self::TelecommunicationsMinisterialOrdinance)
        }
        if name.contains("中央省庁等改革推進本部") {
            v.push(Self::CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance)
        }
        if name.contains("電波監理委員会") {
            v.push(Self::RadioRegulatoryCommissionRules)
        }
        if name.contains("中央労働委員会") {
            v.push(Self::CentralLaborRelationsCommissionRules)
        }
        if name.contains("公正取引委員会") {
            v.push(Self::FairTradeCommissionRules)
        }
        if name.contains("国家公安委員会") {
            v.push(Self::NationalPublicSafetyCommissionRegulations)
        }
        if name.contains("公害等調整委員会") {
            v.push(Self::PollutionAdjustmentCommitteeRules)
        }
        if name.contains("公安審査委員会") {
            v.push(Self::PublicSafetyReviewCommitteeRules)
        }
        v
    }
}

/// M6時（2001年1月16日〜）での府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum M6Ministry {
    /// 内閣官房令
    CabinetSecretariatOrdinance,
    /// 総理庁令
    PrimeMinistersOfficeOrdinance,
    /// 復興庁令
    ReconstructionAgencyOrdinance,
    /// 自治省令
    MinistryOfHomeAffairsOrdinance,
    /// 法務省令
    MinistryOfJusticeOrdinance,
    /// 外務省令
    MinistryOfForeignAffairsOrdinance,
    /// 財務省令
    MinistryOfFinanceOrdinance,
    /// 文部科学省令
    MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance,
    /// 厚生労働省令
    MinistryOfHealthAndLaborAndWelfareOrdinance,
    /// 農林水産省令
    MinistryOfAgricultureAndForestryAndFisheriesOrdinance,
    /// 経済産業省令
    MinistryOfEconomyAndTradeAndIndustryOrdinance,
    /// 国土交通省令
    MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance,
    /// 環境省令
    MinistryOfTheEnvironmentOrdinance,
    /// 防衛省令
    MinistryOfDefenseOrdinance,
    /// デジタル庁令
    DigitalAgencyOrdinance,
    /// 特定個人情報保護委員会規則
    SpecificPersonalInformationProtectionCommissionRules,
    /// 運輸安全委員会規則
    JapanTransportSafetyBoardRegulations,
    /// 原子力規制委員会規則
    NuclearRegulationAuthorityRegulations,
    /// 中央労働委員会規則
    CentralLaborRelationsCommissionRules,
    /// 公正取引委員会規則
    FairTradeCommissionRules,
    /// 国家公安委員会規則
    NationalPublicSafetyCommissionRegulations,
    /// 公害等調整委員会規則
    PollutionAdjustmentCommitteeRules,
    /// 公安審査委員会規則
    PublicSafetyReviewCommitteeRules,
    /// カジノ管理委員会規則
    CasinoManagementCommitteeRules,
}

impl MinistryContents for M6Ministry {
    fn to_int(&self) -> usize {
        use M6Ministry::*;
        match self {
            CabinetSecretariatOrdinance => 1,
            PrimeMinistersOfficeOrdinance => 2,
            ReconstructionAgencyOrdinance => 3,
            MinistryOfHomeAffairsOrdinance => 4,
            MinistryOfJusticeOrdinance => 5,
            MinistryOfForeignAffairsOrdinance => 6,
            MinistryOfFinanceOrdinance => 7,
            MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance => 8,
            MinistryOfHealthAndLaborAndWelfareOrdinance => 9,
            MinistryOfAgricultureAndForestryAndFisheriesOrdinance => 10,
            MinistryOfEconomyAndTradeAndIndustryOrdinance => 11,
            MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance => 12,
            MinistryOfTheEnvironmentOrdinance => 13,
            MinistryOfDefenseOrdinance => 14,
            DigitalAgencyOrdinance => 15,
            SpecificPersonalInformationProtectionCommissionRules => 18,
            JapanTransportSafetyBoardRegulations => 19,
            NuclearRegulationAuthorityRegulations => 20,
            CentralLaborRelationsCommissionRules => 21,
            FairTradeCommissionRules => 22,
            NationalPublicSafetyCommissionRegulations => 23,
            PollutionAdjustmentCommitteeRules => 24,
            PublicSafetyReviewCommitteeRules => 25,
            CasinoManagementCommitteeRules => 26,
        }
    }

    fn from_int(n: usize) -> Option<Self> {
        use M6Ministry::*;
        match n {
            1 => Some(CabinetSecretariatOrdinance),
            2 => Some(PrimeMinistersOfficeOrdinance),
            3 => Some(ReconstructionAgencyOrdinance),
            4 => Some(MinistryOfHomeAffairsOrdinance),
            5 => Some(MinistryOfJusticeOrdinance),
            6 => Some(MinistryOfForeignAffairsOrdinance),
            7 => Some(MinistryOfFinanceOrdinance),
            8 => Some(MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance),
            9 => Some(MinistryOfHealthAndLaborAndWelfareOrdinance),
            10 => Some(MinistryOfAgricultureAndForestryAndFisheriesOrdinance),
            11 => Some(MinistryOfEconomyAndTradeAndIndustryOrdinance),
            12 => Some(MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance),
            13 => Some(MinistryOfTheEnvironmentOrdinance),
            14 => Some(MinistryOfDefenseOrdinance),
            15 => Some(DigitalAgencyOrdinance),
            18 => Some(SpecificPersonalInformationProtectionCommissionRules),
            19 => Some(JapanTransportSafetyBoardRegulations),
            20 => Some(NuclearRegulationAuthorityRegulations),
            21 => Some(CentralLaborRelationsCommissionRules),
            22 => Some(FairTradeCommissionRules),
            23 => Some(NationalPublicSafetyCommissionRegulations),
            24 => Some(PollutionAdjustmentCommitteeRules),
            25 => Some(PublicSafetyReviewCommitteeRules),
            26 => Some(CasinoManagementCommitteeRules),
            _ => None,
        }
    }

    fn start() -> Date {
        Date::new_ad(2001, 1, 6)
    }

    /// 施行中なのでMAX
    fn end() -> Date {
        Date::new_ad(usize::MAX, 12, 31)
    }

    fn from_name(name: &str) -> Vec<Self> {
        let mut v = Vec::new();
        if name.contains("内閣官房") {
            v.push(Self::CabinetSecretariatOrdinance)
        }
        if name.contains("総理庁") {
            v.push(Self::PrimeMinistersOfficeOrdinance)
        }
        if name.contains("復興庁") {
            v.push(Self::MinistryOfHomeAffairsOrdinance)
        }
        if name.contains("自治省") {
            v.push(Self::MinistryOfHomeAffairsOrdinance)
        }
        if name.contains("法務省") {
            v.push(Self::MinistryOfJusticeOrdinance)
        }
        if name.contains("外務省") {
            v.push(Self::MinistryOfForeignAffairsOrdinance)
        }
        if name.contains("財務省") {
            v.push(Self::MinistryOfFinanceOrdinance)
        }
        if name.contains("文部科学省") {
            v.push(Self::MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance)
        }
        if name.contains("厚生労働省") {
            v.push(Self::MinistryOfHealthAndLaborAndWelfareOrdinance)
        }
        if name.contains("農林水産省") {
            v.push(Self::MinistryOfAgricultureAndForestryAndFisheriesOrdinance)
        }
        if name.contains("経済産業省") {
            v.push(Self::MinistryOfEconomyAndTradeAndIndustryOrdinance)
        }
        if name.contains("国土交通省") {
            v.push(Self::MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance)
        }
        if name.contains("環境省") {
            v.push(Self::MinistryOfTheEnvironmentOrdinance)
        }
        if name.contains("防衛省") {
            v.push(Self::MinistryOfDefenseOrdinance)
        }
        if name.contains("デジタル庁") {
            v.push(Self::DigitalAgencyOrdinance)
        }
        if name.contains("特定個人情報保護委員会") {
            v.push(Self::SpecificPersonalInformationProtectionCommissionRules)
        }
        if name.contains("運輸安全委員会") {
            v.push(Self::JapanTransportSafetyBoardRegulations)
        }
        if name.contains("原子力規制委員会") {
            v.push(Self::NuclearRegulationAuthorityRegulations)
        }
        if name.contains("中央労働委員会") {
            v.push(Self::CentralLaborRelationsCommissionRules)
        }
        if name.contains("公正取引委員会") {
            v.push(Self::FairTradeCommissionRules)
        }
        if name.contains("国家公安委員会") {
            v.push(Self::NationalPublicSafetyCommissionRegulations)
        }
        if name.contains("公害等調整委員会") {
            v.push(Self::PollutionAdjustmentCommitteeRules)
        }
        if name.contains("公安審査委員会") {
            v.push(Self::PublicSafetyReviewCommitteeRules)
        }
        if name.contains("カジノ管理委員会") {
            v.push(Self::CasinoManagementCommitteeRules)
        }
        v
    }
}

/// 府・省
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Ministry {
    /// 1869年7月8日〜1943年10月31日
    M1(Vec<M1Ministry>),
    /// 1943年11月1日〜1945年11月31日
    M2(Vec<M2Ministry>),
    /// 1945年12月1日〜1947年5月2日
    M3(Vec<M3Ministry>),
    /// 1947年5月3日〜1949年5月31日
    M4(Vec<M4Ministry>),
    /// 1949年6月1日〜2001年1月15日
    M5(Vec<M5Ministry>),
    /// 2001年1月16日〜
    M6(Vec<M6Ministry>),
}

impl Ministry {
    pub fn to_id_str(&self) -> String {
        match self {
            Self::M1(l) => format!("M1{}", M1Ministry::to_id_str(l)),
            Self::M2(l) => format!("M2{}", M2Ministry::to_id_str(l)),
            Self::M3(l) => format!("M3{}", M3Ministry::to_id_str(l)),
            Self::M4(l) => format!("M4{}", M4Ministry::to_id_str(l)),
            Self::M5(l) => format!("M5{}", M5Ministry::to_id_str(l)),
            Self::M6(l) => format!("M6{}", M6Ministry::to_id_str(l)),
        }
    }

    pub fn from_id_str(s: &str) -> Result<Self, String> {
        if &s[0..=0] == "M" {
            let ministry = match &s[1..=1] {
                "1" => {
                    let n = usize::from_str_radix(&s[2..=8], 16)
                        .map_err(|_| String::from("unexpected string"))?;
                    let byte_s = format!("{n:028b}");
                    let l = M1Ministry::from_id_str(&byte_s)?;
                    Ministry::M1(l)
                }
                "2" => {
                    let n = usize::from_str_radix(&s[2..=8], 16)
                        .map_err(|_| String::from("unexpected string"))?;
                    let byte_s = format!("{n:028b}");
                    let l = M2Ministry::from_id_str(&byte_s)?;
                    Ministry::M2(l)
                }
                "3" => {
                    let n = usize::from_str_radix(&s[2..=8], 16)
                        .map_err(|_| String::from("unexpected string"))?;
                    let byte_s = format!("{n:028b}");
                    let l = M3Ministry::from_id_str(&byte_s)?;
                    Ministry::M3(l)
                }
                "4" => {
                    let n = usize::from_str_radix(&s[2..=8], 16)
                        .map_err(|_| String::from("unexpected string"))?;
                    let byte_s = format!("{n:028b}");
                    let l = M4Ministry::from_id_str(&byte_s)?;
                    Ministry::M4(l)
                }
                "5" => {
                    let n = usize::from_str_radix(&s[2..=8], 16)
                        .map_err(|_| String::from("unexpected string"))?;
                    let byte_s = format!("{n:028b}");
                    let l = M5Ministry::from_id_str(&byte_s)?;
                    Ministry::M5(l)
                }
                "6" => {
                    let n = usize::from_str_radix(&s[2..=8], 16)
                        .map_err(|_| String::from("unexpected string"))?;
                    let byte_s = format!("{n:028b}");
                    let l = M6Ministry::from_id_str(&byte_s)?;
                    Ministry::M6(l)
                }
                _ => return Err(String::from("unexpected string")),
            };
            Ok(ministry)
        } else {
            Err(String::from("unexpected string"))
        }
    }

    pub fn from_name(name: &str) -> Result<Self, String> {
        let err_msg = String::from("Unexpected input");
        let re = Regex::new(r"(?<wareki>(明治|大正|昭和|平成|令和)[一|二|三|四|五|六|七|八|九|十|百|1|2|3|4|5|6|7|8|9|0|１|２|３|４|５|６|７|８|９|０]+)年([一|二|三|四|五|六|七|八|九|十|百|1|2|3|4|5|6|7|8|9|0|１|２|３|４|５|６|７|８|９|０]+月)?([一|二|三|四|五|六|七|八|九|十|百|1|2|3|4|5|6|7|8|9|0|１|２|３|４|５|６|７|８|９|０]+日)?(?<ministry>.+)(令|規則)").unwrap();
        if let Some(caps) = re.captures(name) {
            let ministry_s = &caps["ministry"];
            let wareki_s = &caps["wareki"];
            let wareki = Wareki::from_text(wareki_s).ok_or(err_msg.clone())?;
            if M1Ministry::applicable_wareki(wareki) {
                let l = M1Ministry::from_name(ministry_s);
                Ok(Ministry::M1(l))
            } else if M2Ministry::applicable_wareki(wareki) {
                let l = M2Ministry::from_name(ministry_s);
                Ok(Ministry::M2(l))
            } else if M3Ministry::applicable_wareki(wareki) {
                let l = M3Ministry::from_name(ministry_s);
                Ok(Ministry::M3(l))
            } else if M4Ministry::applicable_wareki(wareki) {
                let l = M4Ministry::from_name(ministry_s);
                Ok(Ministry::M4(l))
            } else if M5Ministry::applicable_wareki(wareki) {
                let l = M5Ministry::from_name(ministry_s);
                Ok(Ministry::M5(l))
            } else if M6Ministry::applicable_wareki(wareki) {
                let l = M6Ministry::from_name(ministry_s);
                Ok(Ministry::M6(l))
            } else {
                Err(err_msg.clone())
            }
        } else {
            Err(err_msg)
        }
    }
}

/// 機関名
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Institution {
    /// 会計検査院
    BoardOfAudit,
    /// 海上保安庁
    CoastGuard,
    /// 日本学術会議
    ScienceCouncilOfJapan,
    /// 土地調整委員会
    LandAdjustmentCommittee,
    /// 金融再生委員会
    FinancialReconstructionCommittee,
    /// 首都圏整備委員会
    MetropolitanAreaDevelopmentCommittee,
    /// 地方財政委員会
    LocalFinanceCommittee,
    /// 司法試験管理委員会
    BarExaminationManagementCommittee,
    /// 公認会計士管理委員会
    CertifiedPublicAccountantManagementCommittee,
    /// 外資委員会
    ForeignInvestmentCommittee,
    /// 文化財保護委員会
    CulturalPropertiesProtectionCommittee,
    /// 日本ユネスコ国内委員会
    JapaneseNationalCommissionForUNESCO,
    /// 最高裁判所
    SupremeCourt,
    /// 衆議院
    HouseOfRepresentatives,
    /// 参議院
    HouseOfCouncilors,
    /// 船員中央労働委員会
    SeafarersCentralLaborCommittee,
    /// 司法試験管理委員会
    /// 電波監理委員会
    RadioRegulatoryCommission,
    /// カジノ管理委員会
    CasinoManagementCommittee,
}

impl Institution {
    pub fn to_int(&self) -> usize {
        use Institution::*;
        match self {
            BoardOfAudit => 1,
            CoastGuard => 2,
            ScienceCouncilOfJapan => 3,
            LandAdjustmentCommittee => 4,
            FinancialReconstructionCommittee => 5,
            MetropolitanAreaDevelopmentCommittee => 6,
            LocalFinanceCommittee => 7,
            BarExaminationManagementCommittee => 8,
            CertifiedPublicAccountantManagementCommittee => 9,
            ForeignInvestmentCommittee => 10,
            CulturalPropertiesProtectionCommittee => 11,
            JapaneseNationalCommissionForUNESCO => 12,
            SupremeCourt => 13,
            HouseOfRepresentatives => 14,
            HouseOfCouncilors => 15,
            SeafarersCentralLaborCommittee => 16,
            RadioRegulatoryCommission => 18,
            CasinoManagementCommittee => 19,
        }
    }

    pub fn from_int(n: usize) -> Option<Self> {
        use Institution::*;
        match n {
            1 => Some(BoardOfAudit),
            2 => Some(CoastGuard),
            3 => Some(ScienceCouncilOfJapan),
            4 => Some(LandAdjustmentCommittee),
            5 => Some(FinancialReconstructionCommittee),
            6 => Some(MetropolitanAreaDevelopmentCommittee),
            7 => Some(LocalFinanceCommittee),
            8 => Some(BarExaminationManagementCommittee),
            9 => Some(CertifiedPublicAccountantManagementCommittee),
            10 => Some(ForeignInvestmentCommittee),
            11 => Some(CulturalPropertiesProtectionCommittee),
            12 => Some(JapaneseNationalCommissionForUNESCO),
            13 => Some(SupremeCourt),
            14 => Some(HouseOfRepresentatives),
            15 => Some(HouseOfCouncilors),
            16 => Some(SeafarersCentralLaborCommittee),
            17 => Some(BarExaminationManagementCommittee),
            18 => Some(RadioRegulatoryCommission),
            19 => Some(CasinoManagementCommittee),
            _ => None,
        }
    }

    /// 「会計検査院規則」などから導き出す
    pub fn from_name(name: &str) -> Option<Self> {
        if name.contains("会計検査院") {
            Some(Self::BoardOfAudit)
        } else if name.contains("海上保安庁") {
            Some(Self::CoastGuard)
        } else if name.contains("日本学術会議") {
            Some(Self::ScienceCouncilOfJapan)
        } else if name.contains("土地調整委員会") {
            Some(Self::LandAdjustmentCommittee)
        } else if name.contains("金融再生委員会") {
            Some(Self::FinancialReconstructionCommittee)
        } else if name.contains("首都圏整備委員会") {
            Some(Self::MetropolitanAreaDevelopmentCommittee)
        } else if name.contains("地方財政委員会") {
            Some(Self::LocalFinanceCommittee)
        } else if name.contains("司法試験管理委員会") {
            Some(Self::BarExaminationManagementCommittee)
        } else if name.contains("公認会計士管理委員会") {
            Some(Self::CertifiedPublicAccountantManagementCommittee)
        } else if name.contains("外資委員会") {
            Some(Self::ForeignInvestmentCommittee)
        } else if name.contains("文化財保護委員会") {
            Some(Self::CulturalPropertiesProtectionCommittee)
        } else if name.contains("日本ユネスコ国内委員会") {
            Some(Self::JapaneseNationalCommissionForUNESCO)
        } else if name.contains("最高裁判所") {
            Some(Self::SupremeCourt)
        } else if name.contains("衆議院") {
            Some(Self::HouseOfRepresentatives)
        } else if name.contains("参議院") {
            Some(Self::HouseOfCouncilors)
        } else if name.contains("船員中央労働委員会") {
            Some(Self::SeafarersCentralLaborCommittee)
        } else if name.contains("電波監理委員会") {
            Some(Self::RadioRegulatoryCommission)
        } else if name.contains("カジノ管理委員会") {
            Some(Self::CasinoManagementCommittee)
        } else {
            None
        }
    }
}

/// 法令IDの詳細 <https://elaws.e-gov.go.jp/file/LawIdNamingConvention.pdf> を参照
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LawType {
    /// 憲法
    Constitution,
    /// 法律
    Act { rippou_type: RippouType, num: usize },
    /// 政令
    CabinetOrder { efficacy: LawEfficacy, num: usize },
    /// 勅令
    ImperialOrder { efficacy: LawEfficacy, num: usize },
    /// 太政官布告
    DajokanFukoku { efficacy: LawEfficacy, num: usize },
    /// 太政官達
    DajokanTasshi { efficacy: LawEfficacy, num: usize },
    /// 太政官布達
    DajokanHutatsu { efficacy: LawEfficacy, num: usize },
    /// 府省令
    MinistryOrder { ministry: Ministry, num: usize },
    /// 人事院規則
    Jinjin {
        /// 規則の分類
        kind: usize,
        /// 規則の分類中の連番
        kind_serial_number: usize,
        /// 改正規則の連番
        amendment_serial_number: usize,
    },
    /// 機関の規則
    Regulation {
        institution: Institution,
        num: usize,
    },
    /// 内閣総理大臣決定の行政機関の規則
    PrimeMinisterDecision {
        /// 決定月
        month: usize,
        /// 決定日
        day: usize,
        /// 同一決定日内の連番
        num: usize,
    },
}

impl LawType {
    pub fn to_id_str(&self) -> String {
        use LawType::*;
        match self {
            Constitution => String::from("CONSTITUTION"),
            Act { rippou_type, num } => match &rippou_type {
                RippouType::Kakuhou => format!("AC0000000{num:03}"),
                RippouType::Syuin => format!("AC1000000{num:03}"),
                RippouType::Sanin => format!("AC0100000{num:03}"),
            },
            CabinetOrder { efficacy, num } => match &efficacy {
                LawEfficacy::Law => format!("CO1000000{num:03}"),
                LawEfficacy::CabinetOrder => format!("CO0000000{num:03}"),
            },
            ImperialOrder { efficacy, num } => match &efficacy {
                LawEfficacy::Law => format!("IO1000000{num:03}"),
                LawEfficacy::CabinetOrder => format!("IO0000000{num:03}"),
            },
            DajokanFukoku { efficacy, num } => match &efficacy {
                LawEfficacy::Law => format!("DF1000000{num:03}"),
                LawEfficacy::CabinetOrder => format!("DF0000000{num:03}"),
            },
            DajokanTasshi { efficacy, num } => match &efficacy {
                LawEfficacy::Law => format!("DT1000000{num:03}"),
                LawEfficacy::CabinetOrder => format!("DT0000000{num:03}"),
            },
            DajokanHutatsu { efficacy, num } => match &efficacy {
                LawEfficacy::Law => format!("DH1000000{num:03}"),
                LawEfficacy::CabinetOrder => format!("DH0000000{num:03}"),
            },
            MinistryOrder { ministry, num } => format!("{}{num:03}", ministry.to_id_str()),
            Jinjin {
                kind,
                kind_serial_number,
                amendment_serial_number,
            } => format!("RJNJ{kind:02}{kind_serial_number:03}{amendment_serial_number:03}"),
            Regulation { institution, num } => format!("R{:>08}{num:03}", institution.to_int()),
            PrimeMinisterDecision { month, day, num } => format!("RPMD{month:02}{day:02}{num:04}"),
        }
    }

    pub fn from_id_str(s: &str) -> Option<Self> {
        use LawType::*;
        if s == "CONSTITUTION" {
            Some(Constitution)
        } else if &s[0..=1] == "AC" {
            let rippou_type_s = &s[2..=8].parse::<usize>().ok()?;
            let rippou_type = if *rippou_type_s == 0 {
                RippouType::Kakuhou
            } else if *rippou_type_s == 1000000 {
                RippouType::Syuin
            } else if *rippou_type_s == 100000 {
                RippouType::Sanin
            } else {
                return None;
            };
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(Act { rippou_type, num })
        } else if &s[0..=1] == "CO" {
            let efficacy_s = &s[2..=8].parse::<usize>().ok()?;
            let efficacy = if *efficacy_s == 0 {
                LawEfficacy::CabinetOrder
            } else if *efficacy_s == 1000000 {
                LawEfficacy::Law
            } else {
                return None;
            };
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(CabinetOrder { efficacy, num })
        } else if &s[0..=1] == "IO" {
            let efficacy_s = &s[2..=8].parse::<usize>().ok()?;
            let efficacy = if *efficacy_s == 0 {
                LawEfficacy::CabinetOrder
            } else if *efficacy_s == 1000000 {
                LawEfficacy::Law
            } else {
                return None;
            };
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(ImperialOrder { efficacy, num })
        } else if &s[0..=1] == "DF" {
            let efficacy_s = &s[2..=8].parse::<usize>().ok()?;
            let efficacy = if *efficacy_s == 0 {
                LawEfficacy::CabinetOrder
            } else if *efficacy_s == 1000000 {
                LawEfficacy::Law
            } else {
                return None;
            };
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(DajokanFukoku { efficacy, num })
        } else if &s[0..=1] == "DT" {
            let efficacy_s = &s[2..=8].parse::<usize>().ok()?;
            let efficacy = if *efficacy_s == 0 {
                LawEfficacy::CabinetOrder
            } else if *efficacy_s == 1000000 {
                LawEfficacy::Law
            } else {
                return None;
            };
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(DajokanTasshi { efficacy, num })
        } else if &s[0..=1] == "DH" {
            let efficacy_s = &s[2..=8].parse::<usize>().ok()?;
            let efficacy = if *efficacy_s == 0 {
                LawEfficacy::CabinetOrder
            } else if *efficacy_s == 1000000 {
                LawEfficacy::Law
            } else {
                return None;
            };
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(DajokanHutatsu { efficacy, num })
        } else if &s[0..=0] == "M" {
            let ministry = Ministry::from_id_str(s).ok()?;
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(MinistryOrder { ministry, num })
        } else if &s[0..=3] == "RJNJ" {
            let kind = s[4..=5].parse::<usize>().ok()?;
            let kind_serial_number = s[6..=8].parse::<usize>().ok()?;
            let amendment_serial_number = s[9..=11].parse::<usize>().ok()?;
            Some(Jinjin {
                kind,
                kind_serial_number,
                amendment_serial_number,
            })
        } else if &s[0..=3] == "RPMD" {
            let month = s[4..=5].parse::<usize>().ok()?;
            let day = s[6..=7].parse::<usize>().ok()?;
            let num = s[8..=11].parse::<usize>().ok()?;
            Some(PrimeMinisterDecision { month, day, num })
        } else if &s[0..=0] == "R" {
            let institution_s = &s[1..=8].parse::<usize>().ok()?;
            let institution = Institution::from_int(*institution_s)?;
            let num = s[9..=11].parse::<usize>().ok()?;
            Some(Regulation { institution, num })
        } else {
            None
        }
    }
}

/// 法令ID： <https://elaws.e-gov.go.jp/file/LawIdNamingConvention.pdf>を参照
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LawId {
    pub wareki: Wareki,
    pub law_type: LawType,
}

impl LawId {
    pub fn to_id_str(&self) -> String {
        format!(
            "{}{:02}{}",
            self.wareki.era.to_number(),
            self.wareki.year,
            self.law_type.to_id_str()
        )
    }
    pub fn from_id_str(s: &str) -> Option<Self> {
        let era_n = &s[0..=0].parse::<usize>().ok()?;
        let era = Era::from_number(*era_n)?;
        let year = &s[1..=2].parse::<usize>().ok()?;
        let type_str = &s[3..=14];
        let law_type = LawType::from_id_str(type_str)?;
        Some(Self {
            wareki: Wareki::new(era, *year),
            law_type,
        })
    }
}

#[test]
fn check_from_str_law_id() {
    let s = "325M50001000004";
    let law_id = LawId::from_id_str(s).unwrap();
    assert_eq!(
        law_id,
        LawId {
            wareki: Wareki::new(Era::Showa, 25),
            law_type: LawType::MinistryOrder {
                ministry: Ministry::M5(vec![
                    M5Ministry::MinistryOfPostsAndTelecommunicationsOrdinance
                ]),
                num: 4
            }
        }
    );
    assert_eq!(law_id.to_id_str(), s);
}

#[test]
fn check_from_str_law_id_2() {
    let s = "345AC0000000089";
    let law_id = LawId::from_id_str(s).unwrap();
    assert_eq!(
        law_id,
        LawId {
            wareki: Wareki::new(Era::Showa, 45),
            law_type: LawType::Act {
                rippou_type: RippouType::Kakuhou,
                num: 89
            }
        }
    );
    assert_eq!(law_id.to_id_str(), s);
}

#[test]
fn check_from_str_law_id_3() {
    let s = "505M60000400060";
    let law_id = LawId::from_id_str(s).unwrap();
    assert_eq!(
        law_id,
        LawId {
            wareki: Wareki::new(Era::Reiwa, 5),
            law_type: LawType::MinistryOrder {
                ministry: Ministry::M6(vec![
                    M6Ministry::MinistryOfEconomyAndTradeAndIndustryOrdinance
                ]),
                num: 60
            }
        }
    );
    assert_eq!(law_id.to_id_str(), s);
}

#[test]
fn check_from_str_law_id_4() {
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
}

#[test]
fn check_from_str_law_id_lst() {
    let v = vec![
        "325M50001000004",
        "345AC0000000089",
        "505M60000400060",
        "505M60000040019",
        "326R00000011009",
    ];
    for s in v.iter() {
        let law_id = LawId::from_id_str(s).unwrap();
        let s2 = law_id.to_id_str();
        assert_eq!(s, &s2);
    }
}
