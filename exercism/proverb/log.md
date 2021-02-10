# Proverb solution 

## Another person solution

```
pub fn build_proverb(list: &[&str]) -> String {
    let mut retval = String::new();

    if list.len() == 0 {
        return retval;
    }
    for i in 0..list.len() - 1 {
        retval.push_str("For want of a ");
        retval.push_str(list[i]);
        retval.push_str(" the ");
        retval.push_str(list[i + 1]);
        retval.push_str(" was lost.\n");
    }
    retval.push_str("And all for the want of a ");
    retval.push_str(list[0]);
    retval.push_str(".");
    retval
}
```

```
pub fn build_proverb(list: &[&str]) -> String
{

    if matches!(list.len(), 0) { return String::new(); };

    let line = String::from("For want of a {first} the {second} was lost.");
    let end = String::from("And all for the want of a {first}.");
    let mut words = list.windows(2).peekable();

    let mut result: Vec<String> = vec![];
    while words.peek().is_some()
    {
        let current = words.next().unwrap();
        let new_line = line.replace("{first}", current[0]).replace("{second}", current[1]);
        result.push(new_line);
    }

    result.push(end.replace("{first}", list[0]));
    return result.join("\n");
}
```

```
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.len() == 0{
        return proverb;
    }

    let length = list.len() - 1;

    for (index,word) in list.iter().enumerate(){
        if index == length{
            proverb += &format!("And all for the want of a {}.", list[0])
        }
        else{
            proverb += &format!("For want of a {} the {} was lost.\n", word, list[index+1])
        }
    }
    proverb
}
```

```
pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() { return String::new(); }

    let text = list
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
        .collect::<Vec<_>>()
        .concat();
    format!("{}And all for the want of a {}.", text, &list[0])
}
```
---

## My:


```
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
```