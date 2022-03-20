use std::collections::HashMap;
use serde::{Deserialize, de::Deserializer, Serialize};

/// Account name, helps users with several accounts remember which they are currently using.
/// Displayed to the user above the "Edit/Publish" button on Telegra.ph, other users don't see
/// this name.
#[derive(Deserialize, Serialize, )]
pub struct ShortName(
    #[serde(deserialize_with="de_short_name")]
    pub String
);
impl ShortName {
    pub fn new(short_name: String) -> Self {
        assert!(!short_name.is_empty(), "short name required");
        Self (short_name)
    }

}

fn de_short_name<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de> 
{
    let s = <&str>::deserialize(deserializer)?;
    assert!(!s.is_empty(), "short name required and non-empty");
    Ok(s.into())
}

/// Default author name used when creating new articles.
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorName(
    #[serde(deserialize_with="de_author_name")]
    pub String
);
impl AuthorName {
    pub fn new(author_name: String) -> Self {
        assert!(author_name.len() < 128, "author name no more than 128");
        Self (author_name)
    }

}

fn de_author_name<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de> 
{
    let s = <&str>::deserialize(deserializer)?;
    assert!(s.len() < 128, "author name no more than 128");
    Ok(s.into())
}

/// Profile link, opened when users click on the author's name below the title. Can be any
/// link, not necessarily to a Telegram profile or channel.
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorUrl(
    #[serde(deserialize_with="de_author_url")]
    pub String
);
impl AuthorUrl{
    pub fn new(author_url: String) -> Self {
        assert!(author_url.len() < 512, "author url no more than 512");
        Self (author_url)
    }
}

fn de_author_url<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de> 
{
    let s = <&str>::deserialize(deserializer)?;
    assert!(s.len() < 512, "author url no more than 512");
    Ok(s.into())
}

const FIELDS: [&str; 5] = ["short_name", "author_name", "author_url", "auth_url", "page_count"];
#[derive(Deserialize, Serialize)]
pub struct Fields(
    #[serde(deserialize_with="de_fields")]
    pub Vec<String>
);
impl Fields {
    pub fn new(fields: Vec<String>) -> Self {
        for field  in fields.iter() {
            let field = field as &str;
            assert!(FIELDS.contains(&field), "Invalid field: {} out of {:?}", field, FIELDS);
        }
        if fields.is_empty() {
            return Self( vec!["short_name".into(), "author_name".into(), "author_url".into()] );
        }
        Self (fields)
    }
}

fn de_fields<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de> 
{
    let s = <Vec<&str>>::deserialize(deserializer)?;
    s.iter().for_each(|e| {
        assert!(FIELDS.contains(&e), "Invalid field: {} out of {:?}", e, FIELDS);
    }
        );
    let s = s.into_iter().map(|e| e.to_string()).collect();
    Ok(s)
}

/// Title of the page.
#[derive(Deserialize, Serialize, Debug)]
pub struct Title(
    #[serde(deserialize_with="de_title")]
    pub String
);
impl Title{
    pub fn new(title: String) -> Self {
        assert!(!title.is_empty(), "title required");
        assert!(title.len() < 256, "title no more than 256");
        Self (title)
    }

}

fn de_title<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de> 
{
    let s = <&str>::deserialize(deserializer)?;
    assert!(!s.is_empty(), "title required");
    assert!(s.len() < 256, "title no more than 256");
    Ok(s.into())
}

/// Content of the page. 
#[derive(Deserialize, Serialize, Debug)]
pub struct Content(pub Vec<Node>);
impl Content {
    pub fn new(content: Vec<Node>) -> Self {
        assert!(!content.is_empty(), "content required");
        assert!(content.len() < 64 * 1024, "content length is no more than 64*1024");
        Self (content)
    }

}

/// Limits the number of pages to be retrieved.
#[derive(Deserialize, Serialize)]
pub struct Limit(
    #[serde(deserialize_with="de_limit")]
    pub u8
);
impl Limit {
    pub fn new() -> Self {
        Self ( 50 )
    }

    pub fn with_val(limit: u8) -> Self {
        assert!(limit < 201, "maximal items for each page are no more than 200");
        Self(limit)
    }
}

fn de_limit<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de> 
{
    let s = <u8>::deserialize(deserializer)?;
    assert!(s < 201, "maximal items for each page are no more than 200");
    Ok(s)
}

/// Required if month is passed. If passed, the number of page views for the requested year will be
/// returned.
#[derive(Deserialize, Serialize)]
pub struct Year(
    #[serde(deserialize_with="de_year")]
    pub u16
);
impl Year {
    pub fn new(year: u16) -> Self {
        assert!(year > 1999, "year range: 2000 - 2100");
        assert!(year < 2101, "year range: 2000 - 2100" );
        Self(year)
    }
}

fn de_year<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de> 
{
    let year = <u16>::deserialize(deserializer)?;
    assert!(year > 1999, "year range: 2000 - 2100");
    assert!(year < 2101, "year range: 2000 - 2100" );
    Ok(year)
}


/// Required if day is passed. If passed, the number of page views for the requested month will be
/// returned.
#[derive(Deserialize, Serialize)]
pub struct Month(
    #[serde(deserialize_with="de_month")]
    pub u8
);
impl Month {
    pub fn new(month: u8) -> Self {
        assert!(month > 0, "month range: 0 - 13");
        assert!(month < 13, "month range: 0 - 13" );
        Self(month)
    }
}

fn de_month<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de> 
{
    let month = <u8>::deserialize(deserializer)?;
    assert!(month > 0, "month range: 0 - 13");
    assert!(month < 13, "month range: 0 - 13" );
    Ok(month)
}


/// Required if day is passed. If passed, the number of page views for the requested month will be
/// returned.
#[derive(Deserialize, Serialize)]
pub struct Day(
    #[serde(deserialize_with="de_day")]
    pub u8
);
impl Day {
    pub fn new(day: u8) -> Self {
        assert!(day > 0, "day range: 1 - 31");
        assert!(day < 32, "day range: 1 - 31" );
        Self(day)
    }
}

fn de_day<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de> 
{
    let day = <u8>::deserialize(deserializer)?;
    assert!(day > 0, "day range: 1 - 31");
    assert!(day < 32, "day range: 1 - 31" );
    Ok(day)
}

/// If passed, the number of page views for the requested hour will be returned.
#[derive(Deserialize, Serialize)]
pub struct Hour(
    #[serde(deserialize_with="de_hour")]
    pub u8
);
impl Hour {
    pub fn new(hour: u8) -> Self {
        assert!(hour < 25, "hour range: 0 - 24" );
        Self(hour)
    }
}

fn de_hour<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de> 
{
    let hour = <u8>::deserialize(deserializer)?;
    assert!(hour < 25, "hour range: 0 - 24" );
    Ok(hour)
}


const KEYS: [&str; 2] = ["href", "src",];
/// Optional. Attributes of the DOM element. Key of object represents name of attribute, value
/// represents value of attribute. Available attributes: href, src.
#[derive(Deserialize, Serialize)]
pub struct Attrs(
    #[serde(deserialize_with="de_attrs")]
    pub HashMap<String, String>
);
impl Attrs {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, val: String) {
        assert!(KEYS.contains(&(&key as &str)), "Invalid key: {} out of {:?}", key, KEYS);
        self.0.insert(key, val);
    }
}

fn de_attrs<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where
        D: Deserializer<'de> 
{
    let attrs = <HashMap<String, String>>::deserialize(deserializer)?;
    for key in attrs.keys() {
        assert!(KEYS.contains(&(&key as &str)), "Invalid key: {} out of {:?}", key, KEYS);
    }
    Ok(attrs)
}

const TAGS: [&str; 24] = ["a", "aside", "b", "blockquote", "br", "code", "em", "figcaption", "figure", "h3", "h4", "hr", "i", "iframe", "img", "li", "ol", "p", "pre", "s", "strong", "u", "ul", "video"];
/// Name of the DOM element. Available tags: a, aside, b, blockquote, br, code, em, figcaption,
/// figure, h3, h4, hr, i, iframe, img, li, ol, p, pre, s, strong, u, ul, video.
#[derive(Deserialize, Serialize)]
pub struct Tag(
    #[serde(deserialize_with="de_tag")]
    pub String
);
impl Tag {
    pub fn new(tag: String) -> Self {
        assert!(TAGS.contains(&(&tag as &str)), "Invalid tag: {} out of {:?}", tag, TAGS);
        Self(tag)
    }
}

fn de_tag<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de> 
{
    let tag = <&str>::deserialize(deserializer)?;
    assert!(TAGS.contains(&(&tag as &str)), "Invalid tag: {} out of {:?}", tag, TAGS);
    Ok(tag.into())
}

/// This object represents a Telegraph account.
#[derive(Deserialize, Serialize)]
pub struct Account {
    pub short_name: ShortName,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<AuthorUrl>,
    /// Optional. Only returned by the createAccount and revokeAccessToken method. Access token of
    /// the Telegraph account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// Optional. URL to authorize a browser on telegra.ph and connect it to a Telegraph account.
    /// This URL is valid for only one use and for 5 minutes only.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    /// Optional. Number of pages belonging to the Telegraph account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<u32>,
}
impl Account {
    pub fn new(short_name: String) -> Self {
        Self {
            short_name: ShortName::new(short_name),
            author_name: None,
            author_url: None,
            access_token: None,
            auth_url: None, 
            page_count: None,
        }
    }
}

/// This object represents a list of Telegraph articles belonging to an account. Most recently
/// created articles first.
#[derive(Deserialize, Serialize, Debug)]
pub struct PageList {
    /// Total number of pages belonging to the target Telegraph account.
    pub total_count: u32,
    /// Requested pages of the target Telegraph account.
    pub pages: Vec<Page>,
}
impl PageList {
    pub fn new() -> Self {
        Self {
            total_count: 0,
            pages: vec![]
        }
    }
}

/// This object represents a page on Telegraph.
#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    /// Path to the page.
    pub path: String,
    /// URL of the page.
    pub url: String,
    pub title: Title,
    /// Description of the page.
    pub description: String,
    /// Optional. Name of the author, displayed below the title.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,
    /// Optional. Profile link, opened when users click on the author's name below the title.  Can
    /// be any link, not necessarily to a Telegram profile or channel.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<AuthorUrl>,
    /// Optional. Image URL of the page.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Optional. Content of the page.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Node>>,
    /// Number of page views for the page.
    pub views: u32,
    /// Optional. Only returned if access_token passed. True, if the target Telegraph account can
    /// edit the page.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
}
impl Page {
    pub fn new(path: String, url: String, title: String, description: String, content: Vec<Node>) -> Self {
        Self {
            path, url, description, 
            title: Title::new(title),
            author_name: None,
            author_url: None,
            image_url: None, 
            content: Some(content),
            views: 0,
            can_edit: None
        }
    }
}

/// This object represents the number of page views for a Telegraph article.
#[derive(Deserialize, Serialize)]
pub struct PageViews {
    /// Number of page views for the target page.
    pub views: u32,
}
impl PageViews {
    pub fn new() -> Self {
        Self {
            views: 0
        }
    }
}

/// This abstract object represents a DOM Node. It can be a String which represents a DOM text node
/// or a NodeElement object.
#[derive(Deserialize, Serialize, Debug)]
pub enum Node {
    String(String),
    NodeElement(NodeElement),
}
impl Node {
    pub fn len(&self) -> usize {
        match self {
            Node::String(n) => n.len(),
            Node::NodeElement(n) => n.len()
        }
    }
}

/// This object represents a DOM element node.
#[derive(Deserialize, Serialize, Debug)]
pub struct NodeElement {
    /// Name of the DOM element. Available tags: a, aside, b, blockquote, br, code, em, figcaption,
    /// figure, h3, h4, hr, i, iframe, img, li, ol, p, pre, s, strong, u, ul, video.
    pub tag: String,
    /// Optional. Attributes of the DOM element. Key of object represents name of attribute, value
    /// represents value of attribute. Available attributes: href, src.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<HashMap<String, String>>,
    /// Optional. List of child nodes for the DOM element.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}
impl NodeElement {
    pub fn len(&self) -> usize {
        let attr = format!("{:?}", self.attrs);
        let child_len = self.children.iter().fold(0, |acc, child| acc + child.len());
        self.tag.len() + attr.len() + child_len
    }
}

