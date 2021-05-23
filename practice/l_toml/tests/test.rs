use l_toml;
use toml;
#[test]

/*
fn test_toml_file(){
    let so:Option<String>=l_toml::reader1("re.toml");    
    assert_eq!(Some(r##"title = "xfplay"

[bibizyz]
website = "http://www.qszy9.com/"
baseurl = "http://www.qszy9.com/"
listreg = "align=\"left\" height=\"29\" width=\"459\"><a href=\"(.+?)\" target=\"_blank\">(.+?)</a></td>"
elementreg = "<li><input name=\"copy_sel\" type=\"checkbox\" value=\"(xfplay://.+?)\"><a href="
showreg = "<tr><td align=\"left\" height=\"29\"><font style=\"font:bold 14px '微软雅黑';padding-left:10px\">(.+?)先锋影音 地址</font></td></tr>"##.to_string()),so);
    /*
 running 1 test
test test_toml_file ... FAILED

failures:

---- test_toml_file stdout ----
thread 'test_toml_file' panicked at 'assertion failed: `(left == right)`
  left: `Some("\ntitle = \"xfplay\"\n\n[bibizyz]\nwebsite = \"http://www.qszy9.com/\"\nbaseurl = \"http://www.qszy9.com/\"\nlistreg = \"align=\\\"left\\\" height=\\\"29\\\" width=\\\"459\\\"><a href=\\\"(.+?)\\\" target=\\\"_blank\\\">(.+?)</a></td>\"\nelementreg = \"<li><input name=\\\"copy_sel\\\" type=\\\"checkbox\\\" value=\\\"(xfplay://.+?)\\\"><a href=\"\nshowreg = \"<tr><td align=\\\"left\\\" height=\\\"29\\\"><font style=\\\"font:bold 14px '微软雅黑';padding-left:10px\\\">(.+?)先锋影音 地址</font></td></tr>\"\n    ")`,
 right: `Some("title = \"xfplay\"\n\n[bibizyz]\nwebsite = \"http://www.qszy9.com/\"\nbaseurl = \"http://www.qszy9.com/\"\nlistreg = \"align=\\\"left\\\" height=\\\"29\\\" width=\\\"459\\\"><a href=\\\"(.+?)\\\" target=\\\"_blank\\\">(.+?)</a></td>\"\nelementreg = \"<li><input name=\\\"copy_sel\\\" type=\\\"checkbox\\\" value=\\\"(xfplay://.+?)\\\"><a href=\"\nshowreg = \"<tr><td align=\\\"left\\\" height=\\\"29\\\"><font style=\\\"font:bold 14px '微软雅黑';padding-left:10px\\\">(.+?)先锋影音 地址</font></td></tr>\"")`', tests\test.rs:7:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_toml_file

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--test test'
    */
}
*/
fn test_toml(){
    let so:Option<String>=l_toml::reader("re.toml");
    let config: l_toml::Config = toml::from_str(&so.unwrap()).unwrap();
    assert_eq!(Some("xfplay".to_string()),config.title,);
    assert_eq!("http://www.qszy9.com/".to_string(),config.bibizyz.website,);
    assert_eq!(Some("http://www.qszy9.com/".to_string()),config.bibizyz.baseurl,);
    assert_eq!(Some("align=\"left\" height=\"29\" width=\"459\"><a href=\"(.+?)\" target=\"_blank\">(.+?)</a></td>".to_string()),config.bibizyz.listreg,);
    assert_eq!(Some("<li><input name=\"copy_sel\" type=\"checkbox\" value=\"(xfplay://.+?)\"><a href=".to_string()),config.bibizyz.elementreg,);
    assert_eq!(Some("<tr><td align=\"left\" height=\"29\"><font style=\"font:bold 14px '微软雅黑';padding-left:10px\">(.+?)先锋影音 地址</font></td></tr>".to_string()),config.bibizyz.showreg,);
}

