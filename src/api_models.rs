use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Input {
    #[serde(rename = "type")]
    pub input_type: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Works {
    #[serde(rename = "$value")]
    pub works: Vec<Work>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Eq, PartialOrd, Ord)]
pub struct Work {
    pub author: String,
    pub editions: String,
    pub format: String,
    pub holdings: String,
    pub hyr: Option<String>,
    pub itemtype: String,
    pub lyr: Option<String>,
    pub owi: String,
    pub schemes: Option<String>,
    pub title: String,
    pub wi: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Response {
    pub code: i64,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Classify {
    pub input: Input,
    pub response: Response,
    #[serde(rename = "workCount")]
    pub work_count: Option<i64>,
    pub work: Option<Work>,
    pub works: Option<Works>,
    pub recommendations: Option<Recommendations>,
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RecommendationData {
    MostPopular(RecommendationStat),
    MostRecent(RecommendationStat),
    LatestEdition(RecommendationStat),
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct RecommendationDataHolder {
    #[serde(rename = "$value")]
    recommendations: Vec<RecommendationData>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Recommendations {
    ddc: RecommendationDataHolder,
    lcc: RecommendationDataHolder,
}

#[derive(Debug, Deserialize, PartialEq, Default, Hash, Eq)]
pub struct RecommendationStat {
    holdings: String,
    nsfa: Option<String>,
    sfa: Option<String>,
    sf2: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn not_found() {
        let result = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><classify xmlns="http://classify.oclc.org"><input type="isbn">foo</input><response code="101"/></classify>"#;
        let classify: Classify = from_str(result).unwrap();
        assert_eq!(
            classify,
            Classify {
                input: Input {
                    input_type: "isbn".to_string(),
                    value: "foo".to_string(),
                },
                response: Response { code: 101 },
                ..Classify::default()
            }
        );
    }

    #[test]
    fn found_code_0() {
        let result = r#"
<classify xmlns="http://classify.oclc.org">
  <response code="0"/>
  <!--Classify is a product of OCLC Online Computer Library Center: http://classify.oclc.org-->
  <work author="foo" editions="28" eholdings="197" format="Book" holdings="2183" itemtype="itemtype-book" owi="47289247" title="FooBar">value</work>
  <orderBy>thold desc</orderBy>
  <input type="isbn">bar</input>
  <recommendations>
    <ddc>
      <mostPopular holdings="2260" nsfa="306.20973" sfa="306.20973"/>
      <mostRecent holdings="2" sfa="304.60973"/>
      <latestEdition holdings="2257" sf2="23" sfa="306.20973"/>
    </ddc>
    <lcc>
      <mostPopular holdings="2342" nsfa="JC599.U5" sfa="JC599.U5"/>
      <mostRecent holdings="2342" sfa="JC599.U5"/>
    </lcc>
  </recommendations>
</classify>
        "#;
        let classify: Classify = from_str(result).unwrap();
        assert_eq!(
            classify,
            Classify {
                response: Response { code: 0 },
                input: Input {
                    input_type: "isbn".to_string(),
                    value: "bar".to_string(),
                },
                work: Some(Work {
                    author: "foo".to_string(),
                    editions: "28".to_string(),
                    format: "Book".to_string(),
                    holdings: "2183".to_string(),
                    hyr: None,
                    itemtype: "itemtype-book".to_string(),
                    lyr: None,
                    owi: "47289247".to_string(),
                    schemes: None,
                    title: "FooBar".to_string(),
                    wi: None,
                }),
                recommendations: Some(Recommendations {
                    ddc: RecommendationDataHolder {
                        recommendations: vec![
                            RecommendationData::MostPopular(RecommendationStat {
                                holdings: "2260".to_string(),
                                nsfa: Some("306.20973".to_string()),
                                sfa: Some("306.20973".to_string()),
                                ..RecommendationStat::default()
                            }),
                            RecommendationData::MostRecent(RecommendationStat {
                                holdings: "2".to_string(),
                                sfa: Some("304.60973".to_string()),
                                ..RecommendationStat::default()
                            }),
                            RecommendationData::LatestEdition(RecommendationStat {
                                holdings: "2257".to_string(),
                                sf2: Some("23".to_string()),
                                sfa: Some("306.20973".to_string()),
                                ..RecommendationStat::default()
                            }),
                        ],
                    },
                    lcc: RecommendationDataHolder {
                        recommendations: vec![
                            RecommendationData::MostPopular(RecommendationStat {
                                holdings: "2342".to_string(),
                                nsfa: Some("JC599.U5".to_string()),
                                sfa: Some("JC599.U5".to_string()),
                                ..RecommendationStat::default()
                            }),
                            RecommendationData::MostRecent(RecommendationStat {
                                holdings: "2342".to_string(),
                                sfa: Some("JC599.U5".to_string()),
                                ..RecommendationStat::default()
                            }),
                        ],
                    },
                }),
                ..Classify::default()
            }
        );
    }

    #[test]
    fn found_code_4() {
        let result = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<classify xmlns="http://classify.oclc.org">
  <response code="4"/>
  <!--Classify is a product of OCLC Online Computer Library Center: http://classify.oclc.org-->
  <workCount>2</workCount>
  <start>0</start>
  <maxRecs>25</maxRecs>
  <orderBy>thold desc</orderBy>
  <input type="isbn">foo</input>
  <works>
    <work author="Dibdin, Michael" editions="66" format="Book" holdings="1278" hyr="2020" itemtype="itemtype-book" lyr="1996" owi="570898" schemes="DDC LCC" title="Così fan tutti : an Aurelio Zen mystery" wi="570898"/>
  </works>
</classify>

        "#;
        let classify: Classify = from_str(result).unwrap();
        assert_eq!(
            classify,
            Classify {
                input: Input {
                    input_type: "isbn".to_string(),
                    value: "foo".to_string(),
                },
                response: Response { code: 4 },
                work_count: Some(2),
                works: Some(Works {
                    works: vec![Work {
                        author: "Dibdin, Michael".to_string(),
                        editions: "66".to_string(),
                        format: "Book".to_string(),
                        holdings: "1278".to_string(),
                        hyr: Some("2020".to_string()),
                        itemtype: "itemtype-book".to_string(),
                        lyr: Some("1996".to_string()),
                        owi: "570898".to_string(),
                        schemes: Some("DDC LCC".to_string()),
                        title: "Così fan tutti : an Aurelio Zen mystery".to_string(),
                        wi: Some("570898".to_string()),
                    },],
                }),
                ..Classify::default()
            }
        );
    }
}
