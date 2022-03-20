use crate::entity::*;

/// Use this method to create a new Telegraph account. Most users only need one account, but this
/// can be useful for channel administrators who would like to keep individual author names and
/// profile links for each of their channels. On success, returns an Account object with the
/// regular fields and an additional access_token field.
///
/// - short_name (String, 1-32 characters)
///     Required. Account name, helps users with several accounts remember which they are
///     currently using. Displayed to the user above the "Edit/Publish" button on Telegra.ph,
///     other users don't see this name.
/// - author_name (String, 0-128 characters)
///     Default author name used when creating new articles.
/// - author_url (String, 0-512 characters)
///     Default profile link, opened when users click on the author's name
///     below the title. Can be any link, not necessarily to a Telegram profile
///     or channel.
/// - Sample request
///     <https://api.telegra.ph/createAccount?short_name=Sandbox&author_name=Anonymous>
pub struct CreateAccount {
    pub short_name: ShortName,
    pub author_name: Option<AuthorName>,
    pub author_url: Option<AuthorUrl>,
}

impl CreateAccount {
    pub fn new(short_name: String) -> Self {
        assert!(!short_name.is_empty(), "short name is required");
        Self {
            short_name: ShortName::new(short_name),
            author_name: None,
            author_url: None,
        }
    }

    pub fn with_raw(
        short_name: String,
        author_name: Option<String>,
        author_url: Option<String>,
    ) -> Self {
        let author_name = match author_name {
            Some(name) => Some(AuthorName::new(name)),
            None => None
        };
        let author_url = match author_url {
            Some(url) => Some(AuthorUrl::new(url)),
            None => None
        };
        Self {
            short_name: ShortName::new(short_name),
            author_name,
            author_url,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut CreateAccount) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> ) -> Ret<Account> {
        f(self).await
    }
}

/// Use this method to update information about a Telegraph account. Pass only the parameters that
/// you want to edit. On success, returns an Account object with the default fields.
///
/// - access_token (String)
///     Required. Access token of the Telegraph account.
/// - short_name (String, 1-32 characters)
///     New account name.
/// - author_name (String, 0-128 characters)
///     New default author name used when creating new articles.
/// - author_url (String, 0-512 characters)
///     New default profile link, opened when users click on the author's name below the title. Can be any link, not necessarily to a Telegram profile or channel.
///
/// - Sample request
///     <https://api.telegra.ph/editAccountInfo?access_token=b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb&short_name=Sandbox&author_name=Anonymous>
pub struct EditAccountInfo {
    pub access_token: String,
    pub short_name: ShortName,
    pub author_name: Option<AuthorName>,
    pub author_url: Option<AuthorUrl>,
}
use std::pin::Pin;
use std::future::Future;
pub type Ret<T> = Result<T, Box<dyn std::error::Error>> ;

impl EditAccountInfo {
    pub fn new(access_token: String, short_name: String) -> Self {
        Self {
            access_token,
            short_name: ShortName::new(short_name),
            author_name: None,
            author_url: None,
        }
    }

    pub fn with_raw(
        access_token: String,
        short_name: String,
        author_name: Option<String>,
        author_url: Option<String>,
    ) -> Self {
        let author_name = match author_name {
            Some(name) => Some(AuthorName(name)),
            None => None
        };
        let author_url = match author_url {
            Some(url) => Some(AuthorUrl(url)),
            None => None
        };
        Self {
            access_token,
            short_name: ShortName::new(short_name),
            author_name,
            author_url,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut EditAccountInfo) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> ) -> Ret<Account> {
        f(self).await
    }
}

/// Use this method to get information about a Telegraph account. Returns an Account object on
/// success.
/// - access_token (String)
///     Required. Access token of the Telegraph account.
/// - fields (Array of String, default = [“short_name”,“author_name”,“author_url”])
///     List of account fields to return. Available fields: short_name, author_name, author_url, auth_url, page_count.
///
/// - Sample request
///     <https://api.telegra.ph/getAccountInfo?access_token=b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb&fields=["short_name","page_count"]>
pub struct GetAccountInfo {
    pub access_token: String,
    pub fields: Fields,
}

impl GetAccountInfo {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            fields: Fields::new(vec![]),
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut GetAccountInfo) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> ) -> Ret<Account> {
        f(self).await
    }
}

/// Use this method to revoke access_token and generate a new one, for example, if the user would
/// like to reset all connected sessions, or you have reasons to believe the token was compromised.
/// On success, returns an Account object with new access_token and auth_url fields.
///
/// - access_token (String)
///     Required. Access token of the Telegraph account.
///
/// - Sample request
///     <https://api.telegra.ph/revokeAccessToken?access_token=b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb>
pub struct RevokeAccessToken {
    pub access_token: String,
}

impl RevokeAccessToken {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut RevokeAccessToken) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> ) -> Ret<Account> {
        f(self).await
    }
}


/// Use this method to create a new Telegraph page. On success, returns a Page object.
///
/// - access_token (String)
///     Required. Access token of the Telegraph account.
/// - title (String, 1-256 characters)
///     Required. Page title.
/// - author_name (String, 0-128 characters)
///     Author name, displayed below the article's title.
/// - author_url (String, 0-512 characters)
///     Profile link, opened when users click on the author's name below the title. Can be any link, not necessarily to a Telegram profile or channel.
/// - content (Array of Node, up to 64 KB)
///     Required. Content of the page.
/// - return_content (Boolean, default = false)
///     If true, a content field will be returned in the Page object (see: Content format).
///
/// - Sample request
///     <https://api.telegra.ph/createPage?access_token=b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb&title=Sample+Page&author_name=Anonymous&content=[{"tag":"p","children":["Hello,+world!"]}]&return_content=true>
pub struct CreatePage {
    pub access_token: String,
    pub title: Title,
    pub author_name: AuthorName,
    pub author_url: AuthorUrl,
    pub content: Content,
    pub return_content: bool,
}

impl CreatePage {
    pub fn new(access_token: String, title: String, content: Content, return_content: bool) -> Self {
        Self {
            access_token,
            title: Title::new(title),
            author_name: AuthorName::new("".into()),
            author_url: AuthorUrl::new("".into()),
            content,
            return_content,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut CreatePage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> ) -> Ret<Page> {
        f(self).await
    }
}

/// Use this method to edit an existing Telegraph page. On success, returns a Page object.
///
/// - access_token (String)
///     Required. Access token of the Telegraph account.
/// - path (String)
///     Required. Path to the page.
/// - title (String, 1-256 characters)
///     Required. Page title.
/// - content (Array of Node, up to 64 KB)
///     Required. Content of the page.
/// - author_name (String, 0-128 characters)
///     Author name, displayed below the article's title.
/// - author_url (String, 0-512 characters)
///     Profile link, opened when users click on the author's name below the title. Can be any link, not necessarily to a Telegram profile or channel.
/// - return_content (Boolean, default = false)
///     If true, a content field will be returned in the Page object.
///
/// - Sample request
///     <https://api.telegra.ph/editPage/Sample-Page-12-15?access_token=b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb&title=Sample+Page&author_name=Anonymous&content=[{"tag":"p","children":["Hello,+world!"]}]&return_content=true>
pub struct EditPage {
    pub access_token: String,
    pub path: String,
    pub title: Title,
    pub content: Content,
    pub author_name: AuthorName,
    pub author_url: AuthorUrl,
    pub return_content: bool,
}

impl EditPage {
    pub fn new(access_token: String, path: String, title: String, content: Content, return_content: bool) -> Self {
        Self {
            access_token,
            title: Title::new(title),
            path,
            author_name: AuthorName::new("".into()),
            author_url: AuthorUrl::new("".into()),
            content,
            return_content,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut EditPage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> ) -> Ret<Page> {
        f(self).await
    }
}

/// Use this method to get a Telegraph page. Returns a Page object on success.
/// 
/// - path (String)
///    Required. Path to the Telegraph page (in the format Title-12-31, i.e. everything that comes after <http://telegra.ph/>).
/// - return_content (Boolean, default = false)
///    If true, content field will be returned in Page object.
///
/// - Sample request
///    <https://api.telegra.ph/getPage/Sample-Page-12-15?return_content=true>
pub struct GetPage {
    pub path: String,
    pub return_content: bool,
}

impl GetPage {
    pub fn new( path: String, return_content: bool) -> Self {
        Self {
            path,
            return_content,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut GetPage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> ) -> Ret<Page> {
        f(self).await
    }
}

/// Use this method to get a list of pages belonging to a Telegraph account. Returns a PageList object, sorted by most recently created pages first.
///
/// - access_token (String)
///     Required. Access token of the Telegraph account.
/// - offset (Integer, default = 0)
///     Sequential number of the first page to be returned.
/// - limit (Integer, 0-200, default = 50)
///     Limits the number of pages to be retrieved.
///
/// - Sample request
///     <https://api.telegra.ph/getPageList?access_token=b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb&limit=3>
pub struct GetPageList {
    pub access_token: String,
    pub offset: u32,
    pub limit: Limit,
}

impl GetPageList {
    pub fn new(access_token: String, offset: u32, ) -> Self {
        Self {
            access_token,
            offset,
            limit: Limit::new(),
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut GetPageList) -> Pin<Box<dyn Future<Output = Ret<PageList>> + 'a>>> ) -> Ret<PageList> {
        f(self).await
    }
}

/// Use this method to get the number of views for a Telegraph article. Returns a PageViews object on success. By default, the total number of page views will be returned.
///
/// - path (String)
///     Required. Path to the Telegraph page (in the format Title-12-31, where 12 is the month and 31 the day the article was first published).
/// - year (Integer, 2000-2100)
///     Required if month is passed. If passed, the number of page views for the requested year will be returned.
/// - month (Integer, 1-12)
///     Required if day is passed. If passed, the number of page views for the requested month will be returned.
/// - day (Integer, 1-31)
///     Required if hour is passed. If passed, the number of page views for the requested day will be returned.
/// - hour (Integer, 0-24)
///     If passed, the number of page views for the requested hour will be returned.
///
/// - Sample request
///     <https://api.telegra.ph/getViews/Sample-Page-12-15?year=2016&month=12>
pub struct GetViews {
    pub path: String,
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub day: Option<Day>,
    pub hour: Option<Hour>,
}

impl GetViews {
    pub fn new(path: String, year: u16, ) -> Self {
        Self {
            path,
            year: Some(Year::new(year)),
            month: None,
            day: None,
            hour: None,
        }
    }

    pub async fn run(&mut self, f: Box<dyn for<'a> Fn(&'a mut GetViews) -> Pin<Box<dyn Future<Output = Ret<PageViews>> + 'a>>> ) -> Ret<PageViews> {
        f(self).await
    }
}

