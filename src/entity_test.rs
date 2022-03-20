#[cfg(test)]
mod test_strings {
    use crate::entity::*;
    use serde_json::Error;

    #[test]
    fn test_short_name() {
        let s1 = "\"test name\"";
        // ShortName
        let r1: Result<ShortName, Error> = serde_json::from_str(s1);
        assert!(r1.is_ok(), "failed to deserialize");
        let res1 = r1.unwrap();
        assert_eq!("test name", res1.0);
        let s2 = serde_json::to_string(&res1);
        assert!(s2.is_ok(), "failed to serialize");
        let res2 = s2.unwrap();
        assert_eq!(s1, res2);
        let s = r#"["item1","item 2"]"#;
        let r: Result<Vec<ShortName>, Error> = serde_json::from_str(s);
        assert!(r.is_ok(), "failed to deserialize");
        let res = r.unwrap();
        assert_eq!("item1", res[0].0);
        assert_eq!("item 2", res[1].0);
        let s2 = serde_json::to_string(&res);
        assert!(s2.is_ok(), "failed to serialize");
        let res = s2.unwrap();
        assert_eq!(s, res);

        // AuthorName
        let r1: Result<AuthorName, Error> = serde_json::from_str(s1);
        assert!(r1.is_ok(), "failed to deserialize");
        let res1 = r1.unwrap();
        assert_eq!("test name", res1.0);
        let s2 = serde_json::to_string(&res1);
        assert!(s2.is_ok(), "failed to serialize");
        let res2 = s2.unwrap();
        assert_eq!(s1, res2);
        let s = r#"["item1","item 2"]"#;
        let r: Result<Vec<AuthorName>, Error> = serde_json::from_str(s);
        assert!(r.is_ok(), "failed to deserialize");
        let res = r.unwrap();
        assert_eq!("item1", res[0].0);
        assert_eq!("item 2", res[1].0);
        let s2 = serde_json::to_string(&res);
        assert!(s2.is_ok(), "failed to serialize");
        let res = s2.unwrap();
        assert_eq!(s, res);

        // AuthorUrl
        let r1: Result<AuthorUrl, Error> = serde_json::from_str(s1);
        assert!(r1.is_ok(), "failed to deserialize");
        let res1 = r1.unwrap();
        assert_eq!("test name", res1.0);
        let s2 = serde_json::to_string(&res1);
        assert!(s2.is_ok(), "failed to serialize");
        let res2 = s2.unwrap();
        assert_eq!(s1, res2);
        let s = r#"["item1","item 2"]"#;
        let r: Result<Vec<AuthorUrl>, Error> = serde_json::from_str(s);
        assert!(r.is_ok(), "failed to deserialize");
        let res = r.unwrap();
        assert_eq!("item1", res[0].0);
        assert_eq!("item 2", res[1].0);
        let s2 = serde_json::to_string(&res);
        assert!(s2.is_ok(), "failed to serialize");
        let res = s2.unwrap();
        assert_eq!(s, res);

        // Title
        let r1: Result<Title, Error> = serde_json::from_str(s1);
        assert!(r1.is_ok(), "failed to deserialize");
        let res1 = r1.unwrap();
        assert_eq!("test name", res1.0);
        let s2 = serde_json::to_string(&res1);
        assert!(s2.is_ok(), "failed to serialize");
        let res2 = s2.unwrap();
        assert_eq!(s1, res2);
        let s = r#"["item1","item 2"]"#;
        let r: Result<Vec<Title>, Error> = serde_json::from_str(s);
        assert!(r.is_ok(), "failed to deserialize");
        let res = r.unwrap();
        assert_eq!("item1", res[0].0);
        assert_eq!("item 2", res[1].0);
        let s2 = serde_json::to_string(&res);
        assert!(s2.is_ok(), "failed to serialize");
        let res = s2.unwrap();
        assert_eq!(s, res);

    }

    #[test]
    #[should_panic]
    fn test_short_name_empty() {
        let s1 = "\"\"";
        assert_eq!(s1.len(), 2);
        let r1: Result<ShortName, Error> = serde_json::from_str(s1);
        assert!(r1.is_ok(), "failed to deserialize");
        let res1 = r1.unwrap();
        assert_eq!("", res1.0);

    }

    #[test]
    #[should_panic]
    fn test_short_name_empty_ser() {
        let s1 = "\"\"";
        let item = ShortName::new("".into());
        let s = serde_json::to_string(&item);
        assert!(s.is_ok(), "failed to serialize");
        let r = s.unwrap();
        assert_eq!(s1, r);
    }
}

#[cfg(test)]
mod test_fields {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let s1 = r#"["blockquote","ul"]"#;
        let d1: Result<Vec<Tag>, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1[0].0, "blockquote", "tag not deserialized");
        assert_eq!(d1[1].0, "ul", "tag not deserialized");
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, s1);
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = r#""class""#;
        let d1: Result<Tag, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1.0, "class" , "attribute not deserialized");
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, s1);
    }
}

#[cfg(test)]
mod test_limit {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_less_200() {
        let s1 = "200";
        let d1: Result<Limit, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 201, "exceed over 200");
        assert_eq!(d1.0, 200);
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, "200");

        let s2 = "20";
        let d2: Result<Limit, Error> = serde_json::from_str(s2);
        assert!(d2.is_ok(), "failed to deserialized");
        let d2 = d2.unwrap();
        assert!(d2.0 < 201, "exceed over 200");
        assert_eq!(d2.0, 20);
        let ser2 = serde_json::to_string(&d2);
        assert!(ser2.is_ok(), "failed to serialized");
        let ser2 = ser2.unwrap();
        assert_eq!(ser2, "20");
    }

    #[test]
    #[should_panic]
    fn test_over_200() {
        let s1 = "203";
        let d1: Result<Limit, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 203, "exceed over 200");
        assert_eq!(d1.0, 203);
    }
}

#[cfg(test)]
mod test_year {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let s1 = "2003";
        let d1: Result<Year, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 2101, "exceed over 2100");
        assert_eq!(d1.0, 2003);
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, "2003");
    }

    #[test]
    #[should_panic]
    fn test_less() {
        let s2 = "20";
        let d2: Result<Year, Error> = serde_json::from_str(s2);
        assert!(d2.is_ok(), "failed to deserialized");
        let d2 = d2.unwrap();
        assert!(d2.0 < 2101, "exceed over 200");
        assert_eq!(d2.0, 20);
        let ser2 = serde_json::to_string(&d2);
        assert!(ser2.is_ok(), "failed to serialized");
        let ser2 = ser2.unwrap();
        assert_eq!(ser2, "20");
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = "2103";
        let d1: Result<Year, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 2101, "exceed over 2100");
        assert_eq!(d1.0, 2103);
    }
}

#[cfg(test)]
mod test_month {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let s1 = "7";
        let d1: Result<Month, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 13, "exceed over 13");
        assert_eq!(d1.0, 7);
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, "7");
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = "103";
        let d1: Result<Year, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 13, "exceed over 13");
        assert_eq!(d1.0, 103);
    }
}

#[cfg(test)]
mod test_day {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let s1 = "31";
        let d1: Result<Day, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 32, "exceed over 32");
        assert_eq!(d1.0, 31);
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, "31");
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = "103";
        let d1: Result<Day, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 32, "exceed over 32");
        assert_eq!(d1.0, 103);
    }
}

#[cfg(test)]
mod test_hour {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let s1 = "24";
        let d1: Result<Hour, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 25, "exceed over 25");
        assert_eq!(d1.0, 24);
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, "24");
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = "30";
        let d1: Result<Hour, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0 < 25, "exceed over 25");
        assert_eq!(d1.0, 30);
    }
}

#[cfg(test)]
mod test_attrs {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let val1 = "https://test.com/";
        let val2 = "https://test.com/?data=test.img";
        let s1 = r#"{"href":"https://test.com/","src":"https://test.com/?data=test.img"}"#;
        let s2 = r#"{"src":"https://test.com/?data=test.img","href":"https://test.com/"}"#;
        let d1: Result<Attrs, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0.get("href").is_some() , "attribute not deserialized");
        assert!(d1.0.get("src").is_some() , "attribute not deserialized");
        assert_eq!(d1.0.get("href").unwrap(), val1, "attribute value not preserved");
        assert_eq!(d1.0.get("src").unwrap(), val2, "attribute value not preserved");
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert!([s1, s2].contains(&(&ser1 as &str)));
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = r#"{"_href":"https://test.com/?data=test.img"}"#;
        let d1: Result<Attrs, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.0.get("_href").is_some() , "attribute not deserialized");
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, s1);
    }
}

#[cfg(test)]
mod test_tag {
    use crate::entity::*;
    use serde_json::Error;
    #[test]
    fn test_normal() {
        let s1 = r#"["blockquote","ul"]"#;
        let d1: Result<Vec<Tag>, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1[0].0, "blockquote", "tag not deserialized");
        assert_eq!(d1[1].0, "ul", "tag not deserialized");
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, s1);
    }

    #[test]
    #[should_panic]
    fn test_over() {
        let s1 = r#""class""#;
        let d1: Result<Tag, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1.0, "class" , "attribute not deserialized");
        let ser1 = serde_json::to_string(&d1);
        assert!(ser1.is_ok(), "failed to serialized");
        let ser1 = ser1.unwrap();
        assert_eq!(ser1, s1);
    }
}

#[cfg(test)]
mod test_account {
    use crate::entity::*;
    use serde_json::Error;

    #[test]
    fn test_normal() {
        let s1 = r#"{"short_name":"short name","author_name":"author name","access_token":"1234567","page_count":3}"#;
        let d1: Result<Account, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.author_url.is_none());
        assert!(d1.auth_url.is_none());
        assert_eq!(d1.short_name.0, "short name");
        assert_eq!(d1.author_name.as_ref().unwrap().0, "author name");
        assert_eq!(d1.access_token.as_ref().unwrap(), "1234567");
        assert_eq!(d1.page_count.as_ref().unwrap(), &3);
        let ser = serde_json::to_string(&d1);
        assert!(ser.is_ok());
        let ser = ser.unwrap();
        assert_eq!(ser, s1, "failed to serialized");
    }

    #[test]
    #[should_panic]
    fn test_no_short_name() {
        let s1 = r#"{"author_name":"author name","access_token":"1234567","page_count":3}"#;
        let d1: Result<Account, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert!(d1.author_url.is_none());
        let ser = serde_json::to_string(&d1);
        assert!(ser.is_ok());
        let ser = ser.unwrap();
        assert_eq!(ser, s1, "failed to serialized");
    }
}

#[cfg(test)]
mod test_page_list {
    use crate::entity::*;
    use serde_json::Error;

    #[test]
    fn test_normal() {
        let s1 = r#"{"total_count":3,"pages":[]}"#;
        let d1: Result<PageList, Error> = serde_json::from_str(s1);
        dbg!(&d1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1.total_count, 3);
        assert_eq!(d1.pages.len(), 0);
        let ser = serde_json::to_string(&d1);
        assert!(ser.is_ok());
        let ser = ser.unwrap();
        assert_eq!(ser, s1, "failed to serialized");
    }
}

#[cfg(test)]
mod test_page {
    use crate::entity::*;
    use serde_json::Error;

    #[test]
    fn test_normal() {
        let s1 = r#"{"path":"page path","url":"https://test.com/","title":"title","description":"","views":1000,"content":[{"String":"text node"},{"NodeElement":{"tag":"h3","attrs":{"src":"https://test.com/?data=test.img"}}}]}"#;
        let d1: Result<Page, Error> = serde_json::from_str(s1);
        dbg!(&d1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1.path, "page path");
        assert_eq!(d1.url, "https://test.com/");
        assert_eq!(d1.title.0, "title");
        assert_eq!(d1.description, "");
        let item = &d1.content.as_ref().unwrap();
        if let Node::String(ref e) = item[0] {
            assert_eq!(e, "text node");
        }
        if let Node::NodeElement(ref e) = item[1] {
            assert_eq!(e.tag, "h3");
            assert_eq!(e.attrs.as_ref().unwrap().get("src").unwrap(), "https://test.com/?data=test.img");
        }
        let ser = serde_json::to_string(&d1);
        assert!(ser.is_ok());
        let ser = ser.unwrap();
        assert_eq!(ser.len(), s1.len(), "failed to serialized");
    }
}

#[cfg(test)]
mod test_page_views {
    use crate::entity::*;
    use serde_json::Error;

    #[test]
    fn test_normal() {
        let s1 = r#"{"views":3}"#;
        let d1: Result<PageViews, Error> = serde_json::from_str(s1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1.views, 3);
        let ser = serde_json::to_string(&d1);
        assert!(ser.is_ok());
        let ser = ser.unwrap();
        assert_eq!(ser, s1, "failed to serialized");
    }
}

#[cfg(test)]
mod test_node_element {
    use crate::entity::*;
    use serde_json::Error;

    #[test]
    fn test_normal() {
        let s1 = r#"{"tag":"a","attrs":{"href":"https://test.com/"},"children":[{"String":"A text node"},{"NodeElement":{"tag":"figure","attrs":{"src":"https://test.com/?data=test.img"}}}]}"#;
        let d1: Result<NodeElement, Error> = serde_json::from_str(s1);
        dbg!(&d1);
        assert!(d1.is_ok(), "failed to deserialized");
        let d1 = d1.unwrap();
        assert_eq!(d1.tag, "a");
        assert_eq!(d1.attrs.as_ref().unwrap().get("href").unwrap(), "https://test.com/");
        let item = &d1.children.as_ref().unwrap();
        if let Node::String(ref e) = item[0] {
            assert_eq!(e, "A text node");
        }

        if let Node::NodeElement(ref e) = item[1] {
            assert_eq!(e.tag, "figure");
            assert_eq!(e.attrs.as_ref().unwrap().get("src").unwrap(), "https://test.com/?data=test.img");
        }
        let ser = serde_json::to_string(&d1);
        assert!(ser.is_ok());
        let ser = ser.unwrap();
        assert_eq!(ser, s1, "failed to serialized");
    }
}
