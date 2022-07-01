use crate::models::*;
use anyhow::Result;
use log::info;
use reqwest::{Client, ClientBuilder};
use serde_xml_rs::from_str;

pub struct OclcClient {
    client: Client,
}

impl OclcClient {
    pub fn new() -> Result<Self> {
        info!("building client");
        let client = ClientBuilder::new().build()?;
        info!("built client");
        Ok(OclcClient { client })
    }

    pub fn new_with_client(client: Client) -> Self {
        OclcClient { client }
    }

    pub async fn lookup(&self, stdnbr: String) -> Result<Option<Works>> {
        let uri = format!(
            "http://classify.oclc.org/classify2/Classify?stdnbr={}&summary=true",
            stdnbr
        );
        info!("looking up {}", uri);
        let response = self.client.get(uri).send().await?.text().await?;
        info!("got: {}", response);
        let classify: Classify = from_str(&response.to_string())?;
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
        let mut expected = vec![
            Work {
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
            },
            Work {
                author: "Dibdin, Michael".to_string(),
                editions: "1".to_string(),
                format: "Book".to_string(),
                holdings: "2".to_string(),
                hyr: Some("1997".to_string()),
                itemtype: "itemtype-book".to_string(),
                lyr: Some("1996".to_string()),
                owi: "10033458423".to_string(),
                schemes: Some("DDC".to_string()),
                title: "Cosi fan tutti".to_string(),
                wi: Some("10033458423".to_string()),
            },
        ];
        expected.sort();
        assert_eq!(works, expected);
        Ok(())
    }
}
