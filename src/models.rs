use crate::api_models::*;
use thiserror::Error;

pub type Result<A> = std::result::Result<A, ClassifyError>;

#[derive(Debug, PartialEq)]
pub enum ClassifyResult {
    SingleWorkSummary(Box<SingleWorkSummary>),
    //SingleWorkDetail(SingleWorkDetail),
    MultiWork(MultiWork),
}

#[derive(Error, Debug)]
pub enum ClassifyError {
    #[error("No input. The method requires an input argument.")]
    NoInput,
    #[error("Invalid input. The standard number argument is invalid.")]
    InvalidInput,
    #[error("Unexpected error.")]
    UnexpectedError,
    #[error("Unexpected response code.")]
    UnexpectedResponseCode,
    #[error("Parsing Error: {0}")]
    XmlParsingError(#[from] serde_xml_rs::Error),
    #[error("Network Error: {0}")]
    IoError(#[from] reqwest::Error),
}

#[derive(Debug, Default, PartialEq)]
pub struct ClassificationRecommendation {
    most_popular: Vec<String>,
    most_recent: Vec<String>,
    latest_edition: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Recommendations {
    pub dewie_decimal: ClassificationRecommendation,
    pub library_of_congress: ClassificationRecommendation,
}

#[derive(Debug, PartialEq)]
pub struct SingleWorkSummary {
    pub work: Work,
    pub recommendations: Recommendations,
}

#[derive(Debug, PartialEq)]
pub struct MultiWork {
    pub works: Vec<Work>,
}

fn push_if_some<A>(c: &mut Vec<A>, opts: Vec<Option<A>>) {
    for opt in opts.into_iter().flatten() {
        c.push(opt)
    }
}

impl From<crate::api_models::RecommendationDataHolder> for ClassificationRecommendation {
    fn from(recommendations: crate::api_models::RecommendationDataHolder) -> Self {
        let mut result = ClassificationRecommendation::default();
        for recommendation in recommendations.recommendations {
            match recommendation {
                RecommendationData::MostPopular(most_popular) => push_if_some(
                    &mut result.most_popular,
                    vec![most_popular.nsfa, most_popular.sfa],
                ),
                RecommendationData::MostRecent(most_recent) => push_if_some(
                    &mut result.most_recent,
                    vec![most_recent.nsfa, most_recent.sfa],
                ),
                RecommendationData::LatestEdition(latest_edition) => push_if_some(
                    &mut result.latest_edition,
                    vec![latest_edition.nsfa, latest_edition.sfa],
                ),
            }
        }
        result
    }
}

impl From<crate::api_models::Recommendations> for Recommendations {
    fn from(recommendations: crate::api_models::Recommendations) -> Self {
        Recommendations {
            dewie_decimal: recommendations.ddc.into(),
            library_of_congress: recommendations.lcc.into(),
        }
    }
}

impl From<Classify> for SingleWorkSummary {
    fn from(classify: Classify) -> Self {
        SingleWorkSummary {
            work: classify.work.unwrap(),
            recommendations: classify.recommendations.unwrap().into(),
        }
    }
}

impl From<Classify> for MultiWork {
    fn from(classify: Classify) -> Self {
        MultiWork {
            works: classify.works.unwrap().works,
        }
    }
}

impl From<Classify> for Result<Option<ClassifyResult>> {
    fn from(classify: Classify) -> Self {
        match classify.response.code {
            100 => Err(ClassifyError::NoInput),
            101 => Err(ClassifyError::InvalidInput),
            102 => Ok(None),
            0 => Ok(Some(ClassifyResult::SingleWorkSummary(Box::new(
                classify.into(),
            )))),
            4 => Ok(Some(ClassifyResult::MultiWork(classify.into()))),
            _ => Err(ClassifyError::UnexpectedResponseCode),
        }
    }
}
