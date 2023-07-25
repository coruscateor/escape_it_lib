use std::str::Chars;

//https://doc.rust-lang.org/reference/tokens.html

pub fn escape(input: Chars<'_>, capacity: Option<usize>, shrink_to_fit: bool) -> String
{

    let mut output;

    match capacity
    {
        Some(val) =>
        {

            output = String::with_capacity(val);

        },
        None =>
        {

            output = String::new();

        }

    }

    for val in input
    {

        match val
        {

            '\n' =>
            {

                output.push_str("\\n");

            },
            '\r' =>
            {

                output.push_str("\\r");

            },
            '\t' =>
            {

                output.push_str("\\t");

            },
            '\\' =>
            {

                output.push_str("\\");

            },
            '\0' =>
            {

                output.push_str("\\0");

            },
            '\'' =>
            {

                output.push_str("\\'");

            }
            '\"' =>
            {

                output.push_str("\\\"");

            },
            _ =>
            {

                output.push(val);

            }

        }

    }

    if shrink_to_fit
    {

        output.shrink_to_fit();

    }

    output

}

pub fn escape_str(input: &str, shrink_to_fit: bool) -> String
{

    escape(input.chars(), Some(input.len() * 2), shrink_to_fit)

}

pub fn escape_string(input: &String, shrink_to_fit: bool) -> String
{

    escape(input.chars(), Some(input.len() * 2), shrink_to_fit)

}
