use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Input {
    #[serde(rename = "type")]
    pub input_type : String,
    #[serde(rename = "$value")]
    pub value : String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Works {
    #[serde(rename = "$value")]
    pub works : Vec<Work>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Eq, PartialOrd, Ord)]
pub struct Work {
    pub author: String,
    pub editions: String,
    pub format: String,
    pub holdings: String,
    pub hyr: String,
    pub itemtype: String,
    pub lyr: String,
    pub owi:String,
    pub schemes: String,
    pub title:String,
    pub wi:String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Response {
    pub code : i64,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Classify {
    pub input: Input,
    pub response: Response,
    #[serde(rename = "workCount")]
    pub work_count: Option<i64>,
    pub works: Option<Works>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn not_found() {
        let result = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><classify xmlns="http://classify.oclc.org"><input type="isbn">foo</input><response code="101"/></classify>"#;
        let classify : Classify = from_str(result).unwrap();
        assert_eq!(classify, Classify {
            input: Input {
                input_type: "isbn".to_string(),
                value: "foo".to_string(),
            },
            response: Response {
                code: 101,
            },
            ..Classify::default()
        });
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
        let classify : Classify = from_str(result).unwrap();
        assert_eq!(classify, Classify {
            input: Input {
                input_type: "isbn".to_string(),
                value: "foo".to_string(),
            },
            response: Response {
                code: 4,
            },
            work_count: Some(2),
            works: Some(Works {
                works: vec! [
                       Work {
                           author: "Dibdin, Michael".to_string(),
                           editions: "66".to_string(),
                           format: "Book".to_string(),
                           holdings: "1278".to_string(),
                           hyr:"2020".to_string(),
                           itemtype: "itemtype-book".to_string(),
                           lyr:"1996".to_string(),
                           owi:"570898".to_string(),
                           schemes:"DDC LCC".to_string(),
                           title:"Così fan tutti : an Aurelio Zen mystery".to_string(),
                           wi:"570898".to_string(),
                       },
                ],
            }),
        });
    }
}
