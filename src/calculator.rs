fn precedence(operator: char) -> i32 {
    match operator {
        '+' | '-' => 1,
        '*' |'/' => 2,
        _ => 0,
    }
}

fn apply_operator(value_a: f64, value_b: f64, operator: char) -> f64 {
    match operator {
        '+' => value_a + value_b,
        '-' => value_a - value_b,
        '*' => value_a * value_b,
        '/' => value_a / value_b,
        _ => 0.0,
    }
}

pub fn evaluate(expression: &str) -> f64 {
    let mut values: Vec<f64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let mut i = 0;
    let chars: Vec<char> = expression.chars().collect();

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_digit(10) {
            let mut num = String::new();
            while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                num.push(chars[i]);
                i += 1;
            }
            values.push(num.parse::<f64>().unwrap());
            continue;
        }

        if c == '(' {
            ops.push(c);
        } else if c == ')' {
            while !ops.is_empty() && *ops.last().unwrap() != '(' {
                let val2 = values.pop().unwrap();
                let val1 = values.pop().unwrap();
                let op = ops.pop().unwrap();
                values.push(apply_operator(val1, val2, op));
            }
            ops.pop();
        } else if "+-*/".contains(c) {
            while !ops.is_empty() && precedence(*ops.last().unwrap()) >= precedence(c) {
                let val2 = values.pop().unwrap();
                let val1 = values.pop().unwrap();
                let op = ops.pop().unwrap();
                values.push(apply_operator(val1, val2, op));
            }
            ops.push(c);
        }
        i += 1;
    }

    while !ops.is_empty() {
        let val2 = values.pop().unwrap();
        let val1 = values.pop().unwrap();
        let op = ops.pop().unwrap();
        values.push(apply_operator(val1, val2, op));
    }
    values[0]
}