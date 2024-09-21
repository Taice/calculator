pub fn get_result_from_string(input: &str) -> Option<f64> {
    let no_spaces = rm_whitespace(input);
    for char in no_spaces.chars() {
        match char as u8 {
            b'+' | b'-' | b'*' | b'/' | b'\0' | b'0'..=b'9' | b'(' | b')' | b'.' => (),
            _ => return None,
        }
    }

    let mut expr: String = input.to_string();
    expr = strip_brackets(expr)?;
    expr = strip_md(expr)?;
    strip_as(expr)
}

fn strip_brackets(mut input: String) -> Option<String> {
    if let Some((mut start, mut end)) = get_bracket(&input) {
        while (start, end) != (0, 0) {
            let bracket: &str = &input[start..end];
            let bracket_string = strip_md(bracket.to_string())?;
            let bracket_result = strip_as(bracket_string)?;

            if start > 0 && end < input.len() {
                input.replace_range((start - 1)..=end, &bracket_result.to_string());
            } else {
                return None;
            }
            (start, end) = get_bracket(&input)?;
        }
        Some(input)
    } else {
        None
    }
}

fn strip_md(mut input: String) -> Option<String> {
    if let Some((mut start, mut end, mut division)) = get_md(&input) {
        while (start, end) != (0, 0) {
            let md: &str = &input[start..end];
            let num1 = get_left_num(md);
            let num2 = get_right_num(md);
            let num: f64 = if division { num1 / num2 } else { num1 * num2 };

            input.replace_range(start..end, &num.to_string());
            (start, end, division) = get_md(&input)?;
        }
    }

    Some(input)
}

fn strip_as(mut input: String) -> Option<f64> {
    if let Some((mut start, mut end, mut minus)) = get_as(&input) {
        while (start, end) != (0, 0) {
            let md: &str = &input[start..end];
            let num1 = get_left_num(md);
            let num2 = get_right_num(md);

            let num: f64 = if minus { num1 - num2 } else { num1 + num2 };

            input.replace_range(start..end, &num.to_string());
            (start, end, minus) = get_as(&input)?;
        }
    } else {
        return None;
    }

    if input.is_empty() {
        return None;
    }

    if let Ok(num) = input.parse() {
        Some(num)
    } else {
        None
    }
}

fn get_bracket(input: &str) -> Option<(usize, usize)> {
    let (mut started, mut ended) = (false, false);
    let (mut start, mut end) = (0, 0);

    for (index, ch) in input.chars().enumerate() {
        if ch == '(' {
            started = true;
            start = index + 1;
        }
        if ch == ')' {
            if !started {
                return None;
            }
            end = index;
            ended = true;
            break;
        }
    }

    if ended || !started {
        Some((start, end))
    } else {
        None
    }
}

fn get_md(input: &str) -> Option<(usize, usize, bool)> {
    let (mut start, mut end, mut division, mut started) = (0, 0, false, false);
    let mut sign = false;

    for (index, ch) in input.chars().enumerate() {
        match ch {
            '*' | '/' => {
                if sign {
                    return None;
                }
                if started {
                    end = index;
                    break;
                }
                if ch == '/' {
                    division = true;
                }
                started = true;
                end = input.len();
                sign = true;
            }
            '+' | '-' => {
                if sign {
                    return None;
                }
                if started {
                    end = index;
                    break;
                } else {
                    start = index + 1;
                }
                sign = true;
            }
            _ => {
                sign = false;
                continue;
            }
        }
    }

    if started {
        Some((start, end, division))
    } else {
        Some((0, 0, false))
    }
}

fn get_as(input: &str) -> Option<(usize, usize, bool)> {
    let (mut start, mut end, mut subtraction, mut started) = (0, 0, false, false);
    let mut sign = false;

    for (index, ch) in input.chars().enumerate() {
        match ch {
            '+' | '-' => {
                if sign {
                    return None;
                }
                if started {
                    end = index;
                    break;
                }
                if ch == '-' {
                    subtraction = true;
                }
                started = true;
                end = input.len();
                sign = true;
            }
            '*' | '/' => {
                if sign {
                    return None;
                }
                if started {
                    end = index;
                    break;
                } else {
                    start = index + 1;
                }
                sign = true;
            }
            _ => {
                sign = false;
                continue;
            }
        }
    }

    if started {
        Some((start, end, subtraction))
    } else {
        Some((0, 0, false))
    }
}

pub fn get_right_num(expression: &str) -> f64 {
    let mut start = expression.len();
    let mut did = false;

    for (index, ch) in expression.chars().enumerate() {
        if ch == '/' || ch == '*' || ch == '+' || ch == '-' {
            did = true;
            start = index + 1;
        }
    }

    if !did {
        return expression.parse().unwrap();
    }

    let num = &expression[start..expression.len()];
    num.parse::<f64>().unwrap()
}

fn get_left_num(expression: &str) -> f64 {
    let mut end = 0;
    let mut did = false;

    for (index, ch) in expression.chars().enumerate() {
        if ch == '/' || ch == '*' || ch == '+' || ch == '-' {
            did = true;
            end = index;
            break;
        }
    }

    if !did {
        return expression.parse().unwrap();
    }

    let num = &expression[0..end];
    num.parse::<f64>().unwrap()
}

fn rm_whitespace(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        match ch {
            ' ' => (),
            _ => result += &ch.to_string(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bracket_works() {
        let s1 = "shabadooo(145+452)iensenpinpb(1442388)";
        let (t1s, t1e) = get_bracket(s1).unwrap();
        assert_eq!(&s1[t1s..t1e], "145+452");

        let s2 = "inetiersntiersntiersntifsryunwfrnfwpynpwnfuybfwnybnwu";
        assert_eq!(get_bracket(s2), Some((0, 0)));

        let s3 = ")45+2()";
        assert_eq!(get_bracket(s3), None);
    }

    #[test]
    fn get_md_works() {
        let s = "234832+234948/4838*243";
        let (start, end, division) = get_md(s).unwrap();
        assert_eq!(&s[start..end], "234948/4838");
        assert!(division);
    }

    #[test]
    fn get_as_works() {
        let s = "234832+234948/4838*243";
        let (start, end, subtraction) = get_as(s).unwrap();
        assert_eq!(&s[start..end], "234832+234948");
        assert!(!subtraction);
    }

    #[test]
    fn get_left_num_works() {
        assert_eq!(get_left_num("43.4525252/485835295285994"), 43.4525252);
        assert_eq!(get_left_num("9328429838952/34.3984858"), 9328429838952.0);
    }

    #[test]
    fn get_right_num_works() {
        assert_eq!(get_right_num("9328429838952/34.3984858"), 34.3984858);
    }

    #[test]
    fn strip_brackets_works() {
        assert_eq!(
            strip_brackets("834834+100293904*8340030*(405*4+2/1)+(8/4*2)".to_string()),
            Some("834834+100293904*8340030*1622+4".to_string())
        );
        assert_eq!(strip_brackets("())".to_string()), None);
    }

    #[test]
    fn strip_md_works() {
        assert_eq!(
            strip_md("304*208+48/12".to_string()).unwrap(),
            "63232+4".to_string()
        );
    }

    #[test]
    fn strip_as_works() {
        assert_eq!(
            strip_as("405+2-4+5+6+2-2-4-5-1".to_string()).unwrap(),
            (405 + 2 - 4 + 5 + 6 + 2 - 2 - 4 - 5 - 1) as f64
        );
    }

    #[test]
    fn result_works() {
        assert_eq!(
            get_result_from_string("4+2*8/(4/2)*1+4*(3+4)-5.0*(2/8)").unwrap(),
            38.75
        );
    }
}
