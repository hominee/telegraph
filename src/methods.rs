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

#[cfg(test)]
mod test_create_account {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = CreateAccount::new("bruce".into());
        let mut account_ = CreateAccount::with_raw("bruce".into(), None, None);
        assert_eq!(account.short_name.0, account_.short_name.0);
        assert_eq!(account.author_name.is_none(), account_.author_name.is_none());
        assert_eq!(account.author_url.is_none(), account_.author_url.is_none());
        account.author_url = Some(AuthorUrl::new("https://github.com/homelyguy/".into()));
        account_.author_url = Some(AuthorUrl::new("https://github.com/homelyguy/".into()));
        assert_eq!(account.author_url.as_ref().unwrap().0, account_.author_url.as_ref().unwrap().0);

        account.author_name = Some(AuthorName::new("captain".into()));
        account_.author_name = Some(AuthorName::new("captain".into()));
        assert_eq!(account.author_name.as_ref().unwrap().0, account_.author_name.as_ref().unwrap().0);

        let f: Box<dyn for<'a> Fn(&'a mut CreateAccount) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut CreateAccount| Box::pin(async move { Ok(Account::new("captain".into())) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
        let f: Box<dyn for<'a> Fn(&'a mut CreateAccount) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut CreateAccount| Box::pin( futures::future::ready(Ok(Account::new("captain".into()))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
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

#[cfg(test)]
mod test_edit_account_info {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = EditAccountInfo::new("1234567".into(),"bruce".into());
        let mut account_ = EditAccountInfo::with_raw("1234567".into(),"bruce".into(), None, None);
        assert_eq!(account.access_token, account_.access_token);
        assert_eq!(account.short_name.0, account_.short_name.0);
        assert_eq!(account.author_name.is_none(), account_.author_name.is_none());
        assert_eq!(account.author_url.is_none(), account_.author_url.is_none());
        account.author_url = Some(AuthorUrl::new("https://github.com/homelyguy/".into()));
        account_.author_url = Some(AuthorUrl::new("https://github.com/homelyguy/".into()));
        assert_eq!(account.author_url.as_ref().unwrap().0, account_.author_url.as_ref().unwrap().0);

        account.author_name = Some(AuthorName::new("captain".into()));
        account_.author_name = Some(AuthorName::new("captain".into()));
        assert_eq!(account.author_name.as_ref().unwrap().0, account_.author_name.as_ref().unwrap().0);

        let f: Box<dyn for<'a> Fn(&'a mut EditAccountInfo) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut EditAccountInfo| Box::pin(async move { Ok(Account::new("captain".into())) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
        let f: Box<dyn for<'a> Fn(&'a mut EditAccountInfo) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut EditAccountInfo| Box::pin( futures::future::ready(Ok(Account::new("captain".into()))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
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

#[cfg(test)]
mod test_get_account_info {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = GetAccountInfo::new("1234567".into());
        assert_eq!(account.access_token, "1234567".to_owned());
        dbg!(&account.fields.0);
        assert!(!account.fields.0.is_empty());
        account.fields.0.push("auth_url".into());
        assert_eq!(account.fields.0.len(), 4);
        assert!(account.fields.0.contains(&"short_name".into()));
        assert!(account.fields.0.contains(&"auth_url".into()));

        let f: Box<dyn for<'a> Fn(&'a mut GetAccountInfo) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut GetAccountInfo| Box::pin(async move { Ok(Account::new("captain".into())) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
        let f: Box<dyn for<'a> Fn(&'a mut GetAccountInfo) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut GetAccountInfo| Box::pin( futures::future::ready(Ok(Account::new("captain".into()))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
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

#[cfg(test)]
mod test_revoke_access_token {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = RevokeAccessToken::new("1234567".into());
        assert_eq!(account.access_token, "1234567".to_owned());

        let f: Box<dyn for<'a> Fn(&'a mut RevokeAccessToken) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut RevokeAccessToken| Box::pin(async move { Ok(Account::new("captain".into())) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
        let f: Box<dyn for<'a> Fn(&'a mut RevokeAccessToken) -> Pin<Box<dyn Future<Output = Ret<Account>> + 'a>>> = Box::new(move |_accnt: &mut RevokeAccessToken| Box::pin( futures::future::ready(Ok(Account::new("captain".into()))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.short_name.0, "captain");
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

#[cfg(test)]
mod test_create_page {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = CreatePage::new("1234567".into(),"title".into(), Content::new(vec![Node::String("text node".into())]), false);
        assert_eq!(account.access_token, "1234567".to_owned());
        assert_eq!(account.title.0, "title".to_owned());
        assert_eq!(account.return_content, false);
        account.content.0.push(Node::String("text node 2".to_owned()));
        let item = &account.content.0;
        assert_eq!(item.len(), 2);
        if let Node::String(ref s) = item[0] {
            assert_eq!(s, "text node");
        }

        let f: Box<dyn for<'a> Fn(&'a mut CreatePage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> = Box::new(move |_accnt: &mut CreatePage| Box::pin(async move { Ok(Page::new("path".into(), "https://example.com/".into(), "title".into(), "description".into(), vec![Node::String("text node".into())])) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.path, "path".to_owned());
        assert_eq!(res.url, "https://example.com/".to_owned());
        assert_eq!(res.title.0, "title".to_owned());
        assert_eq!(res.views, 0);
        assert_eq!(res.description, "description".to_owned());
        assert!(res.author_name.is_none());
        assert!(res.author_url.is_none());
        assert!(res.image_url.is_none());
        assert!(res.content.as_ref().unwrap().len() == 1);
        assert!(res.can_edit.is_none());
        let f: Box<dyn for<'a> Fn(&'a mut CreatePage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> = Box::new(move |_accnt: &mut CreatePage| Box::pin( futures::future::ready(Ok(Page::new("path".into(), "https://example.com/".into(), "title".into(), "description".into(), vec![Node::String("text node".into())]))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.path, "path".to_owned());
        assert_eq!(res.url, "https://example.com/".to_owned());
        assert_eq!(res.title.0, "title".to_owned());
        assert_eq!(res.views, 0);
        assert_eq!(res.description, "description".to_owned());
        assert!(res.author_name.is_none());
        assert!(res.author_url.is_none());
        assert!(res.image_url.is_none());
        assert!(res.content.as_ref().unwrap().len() == 1);
        assert!(res.can_edit.is_none());
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

#[cfg(test)]
mod test_edit_page {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = EditPage::new("1234567".into(), "path".into(), "title".into(), Content::new(vec![Node::String("text node".into())]), false);
        assert_eq!(account.access_token, "1234567".to_owned());
        assert_eq!(account.path, "path".to_owned());
        assert_eq!(account.title.0, "title".to_owned());
        assert_eq!(account.return_content, false);
        account.content.0.push(Node::String("text node 2".to_owned()));
        let item = &account.content.0;
        assert_eq!(item.len(), 2);
        if let Node::String(ref s) = item[0] {
            assert_eq!(s, "text node");
        }

        let f: Box<dyn for<'a> Fn(&'a mut EditPage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> = Box::new(move |_accnt: &mut EditPage| Box::pin(async move { Ok(Page::new("path".into(), "https://example.com/".into(), "title".into(), "description".into(), vec![Node::String("text node".into())])) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.path, "path".to_owned());
        assert_eq!(res.url, "https://example.com/".to_owned());
        assert_eq!(res.title.0, "title".to_owned());
        assert_eq!(res.views, 0);
        assert_eq!(res.description, "description".to_owned());
        assert!(res.author_name.is_none());
        assert!(res.author_url.is_none());
        assert!(res.image_url.is_none());
        assert!(res.content.as_ref().unwrap().len() == 1);
        assert!(res.can_edit.is_none());
        let f: Box<dyn for<'a> Fn(&'a mut EditPage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> = Box::new(move |_accnt: &mut EditPage| Box::pin( futures::future::ready(Ok(Page::new("path".into(), "https://example.com/".into(), "title".into(), "description".into(), vec![Node::String("text node".into())]))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.path, "path".to_owned());
        assert_eq!(res.url, "https://example.com/".to_owned());
        assert_eq!(res.title.0, "title".to_owned());
        assert_eq!(res.views, 0);
        assert_eq!(res.description, "description".to_owned());
        assert!(res.author_name.is_none());
        assert!(res.author_url.is_none());
        assert!(res.image_url.is_none());
        assert!(res.content.as_ref().unwrap().len() == 1);
        assert!(res.can_edit.is_none());
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

#[cfg(test)]
mod test_get_page {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = GetPage::new( "path".into(),  false);
        assert_eq!(account.path, "path".to_owned());
        assert_eq!(account.return_content, false);

        let f: Box<dyn for<'a> Fn(&'a mut GetPage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> = Box::new(move |_accnt: &mut GetPage| Box::pin(async move { Ok(Page::new("path".into(), "https://example.com/".into(), "title".into(), "description".into(), vec![Node::String("text node".into())])) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.path, "path".to_owned());
        assert_eq!(res.url, "https://example.com/".to_owned());
        assert_eq!(res.title.0, "title".to_owned());
        assert_eq!(res.views, 0);
        assert_eq!(res.description, "description".to_owned());
        assert!(res.author_name.is_none());
        assert!(res.author_url.is_none());
        assert!(res.image_url.is_none());
        assert!(res.content.as_ref().unwrap().len() == 1);
        assert!(res.can_edit.is_none());
        let f: Box<dyn for<'a> Fn(&'a mut GetPage) -> Pin<Box<dyn Future<Output = Ret<Page>> + 'a>>> = Box::new(move |_accnt: &mut GetPage| Box::pin( futures::future::ready(Ok(Page::new("path".into(), "https://example.com/".into(), "title".into(), "description".into(), vec![Node::String("text node".into())]))) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.path, "path".to_owned());
        assert_eq!(res.url, "https://example.com/".to_owned());
        assert_eq!(res.title.0, "title".to_owned());
        assert_eq!(res.views, 0);
        assert_eq!(res.description, "description".to_owned());
        assert!(res.author_name.is_none());
        assert!(res.author_url.is_none());
        assert!(res.image_url.is_none());
        assert!(res.content.as_ref().unwrap().len() == 1);
        assert!(res.can_edit.is_none());
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

#[cfg(test)]
mod test_get_page_list {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = GetPageList::new( "1234567".into(), 0 );
        assert_eq!(account.access_token, "1234567".to_owned());
        assert_eq!(account.offset, 0);

        let f: Box<dyn for<'a> Fn(&'a mut GetPageList) -> Pin<Box<dyn Future<Output = Ret<PageList>> + 'a>>> = Box::new(move |_accnt: &mut GetPageList| Box::pin(async move { Ok(PageList::new()) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.total_count, 0);
        assert!(res.pages.is_empty());

        let f: Box<dyn for<'a> Fn(&'a mut GetPageList) -> Pin<Box<dyn Future<Output = Ret<PageList>> + 'a>>> = Box::new(move |_accnt: &mut GetPageList| Box::pin( futures::future::ready(Ok(PageList::new())) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.total_count, 0);
        assert!(res.pages.is_empty());
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

#[cfg(test)]
mod test_get_views {
    use crate::methods::*;
    use std::future::Future ;
    use futures::executor::block_on;

    #[test]
    fn test_normal() {
        let mut account = GetViews::new( "path".into(), 2022 );
        assert_eq!(account.path, "path".to_owned());
        assert_eq!(account.year.as_ref().unwrap().0, 2022);

        let f: Box<dyn for<'a> Fn(&'a mut GetViews) -> Pin<Box<dyn Future<Output = Ret<PageViews>> + 'a>>> = Box::new(move |_accnt: &mut GetViews| Box::pin(async move { Ok(PageViews::new()) }));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.views, 0);

        let f: Box<dyn for<'a> Fn(&'a mut GetViews) -> Pin<Box<dyn Future<Output = Ret<PageViews>> + 'a>>> = Box::new(move |_accnt: &mut GetViews| Box::pin( futures::future::ready(Ok(PageViews::new())) ));
        let res = block_on(account.run(f));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.views, 0);
    }
}

