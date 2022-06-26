use serde::Deserialize;
use anyhow::Result;
use serde_xml_rs::from_str;
use log::info;
use reqwest::{Client, ClientBuilder};

#[derive(Debug, Deserialize, PartialEq, Default)]
struct Input {
    #[serde(rename = "type")]
    input_type : String,
    #[serde(rename = "$value")]
    value : String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Works {
    #[serde(rename = "$value")]
    pub works : Vec<Work>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Eq, PartialOrd, Ord)]
pub struct Work {
    author: String,
    editions: String,
    format: String,
    holdings: String,
    hyr: String,
    itemtype: String,
    lyr: String,
    owi:String,
    schemes: String,
    pub title:String,
    wi:String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
struct Response {
    code : i64,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
struct Classify {
    input: Input,
    response: Response,
    #[serde(rename = "workCount")]
    work_count: Option<i64>,
    works: Option<Works>,
}

pub struct OclcClient {
    client : Client,
}

impl OclcClient {
    pub fn new() -> Result<Self> {
        info!("building client");
        let client = ClientBuilder::new().build()?;
        info!("built client");
        Ok(OclcClient {
            client,
        })
    }

    pub fn new_with_client(client : Client) -> Self {
        OclcClient {
            client,
        }
    }

    pub async fn lookup(&self, isbn : String) -> Result<Option<Works>> {
        let uri = format!("http://classify.oclc.org/classify2/Classify?isbn={}&summary=true", isbn);
        info!("looking up {}", uri);
        let response = self.client.get(uri).send().await?.text().await?;
        info!("got: {}", response);
        let classify : Classify = from_str(&response.to_string())?;
        info!("got classify: {:?}", classify);
        if classify.response.code == 101 {
            Ok(None)
        } else {
            Ok(classify.works)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_not_found() {
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
    fn parse_found_code_4() {
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

    #[tokio::test]
    async fn not_found() -> Result<()> {
        let client = OclcClient::new()?;
        let result = client.lookup("foo".to_string()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn found() -> Result<()> {
        let client = OclcClient::new()?;
        let result = client.lookup("0679442723".to_string()).await?;
        assert_eq!(result.is_some(), true);
        let mut works = result.unwrap().works;
        works.sort();
        let mut expected = vec! [
            Work {
                author:"Dibdin, Michael".to_string(),
                editions:"66".to_string(),
                format:"Book".to_string(),
                holdings:"1278".to_string(),
                hyr:"2020".to_string(),
                itemtype:"itemtype-book".to_string(),
                lyr:"1996".to_string(),
                owi:"570898".to_string(),
                schemes:"DDC LCC".to_string(),
                title:"Così fan tutti : an Aurelio Zen mystery".to_string(),
                wi:"570898".to_string(),
            },
            Work {
                author: "Dibdin, Michael".to_string(),
                editions: "1".to_string(),
                format: "Book".to_string(),
                holdings: "2".to_string(),
                hyr: "1997".to_string(),
                itemtype: "itemtype-book".to_string(),
                lyr: "1996".to_string(),
                owi: "10033458423".to_string(),
                schemes: "DDC".to_string(),
                title: "Cosi fan tutti".to_string(),
                wi: "10033458423".to_string(),
            },
        ];
        expected.sort();
        assert_eq!(works, expected);
        Ok(())
    }
}
