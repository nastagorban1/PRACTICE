fn invert_the_case(s: String) -> String {
    let mut result = String::with_capacity(s.len());     //with_capacity виділяє пам для нового рядка
    for c in s.chars() {
        if c.is_uppercase() {
            result.extend(c.to_lowercase());
        } else {
            result.extend(c.to_uppercase());
        }
    }
    result
}
#[test]
fn test() {
   let data =
       [
           ("Hello", "hELLO"),
           ("Привет", "пРИВЕТ"),
       ];


   data
       .iter()
       .for_each(|(a, b)| {
           assert_eq!(
               invert_the_case(a.to_string()),
               b.to_string()
           );
           assert_eq!(
               invert_the_case(b.to_string()),
               a.to_string()
           );
       });


}
