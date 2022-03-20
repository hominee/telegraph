#[cfg(test)]
mod test_create_account {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_edit_account_info {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_get_account_info {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_revoke_access_token {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_create_page {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_edit_page {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_get_page {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_get_page_list {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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

#[cfg(test)]
mod test_get_views {
    use crate::methods::*;
    use crate::entity::*;
    use std::pin::Pin;
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
