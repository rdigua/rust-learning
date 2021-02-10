pub fn build_proverb(list: &[&str]) -> String {
    let mut r = String::new();
    if list.is_empty() {
        return r;
    }
    if list.len() == 1 {
        r.push_str("And all for the want of a ");
        r.push_str(list[0]);
        r.push_str(".");
        return r.to_string();
    }
    let l = list.len() - 1;
    for i in 0..list.len() {
        if i == l {
            r.push_str("And all for the want of a ");
            r.push_str(list[0]);
            r.push_str(".");
            break;
        }
        r.push_str("For want of a ");
        r.push_str(list[i]);
        r.push_str(" the ");
        r.push_str(list[i + 1]);
        r.push_str(" was lost.\n");
    }
    r.to_string()
}
