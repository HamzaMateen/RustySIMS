pub fn strip_right(text: &mut String) {
    if text.ends_with("\n") || text.ends_with("\r") {
        text.pop();
        strip_right(text);
    }
    ()
}

pub fn print_welcome_msg(title: &str, sub_title: &str, sep_width: usize) {
    let mut pad: i32;
    let mut left_pad: String;
    let mut extra_pad: i32;
    let mut right_pad: String;

    let title_line_width = title.chars().count();
    let sub_title_line_width = sub_title.chars().count();

    if sep_width < title_line_width && sep_width < sub_title_line_width {
        println!("{:<sep_width$}", "");
        println!("{}\n{}", title, sub_title);
        println!("{:<sep_width$}", "");

        ();
    };
    if sep_width > title_line_width {
        // padding for title
        pad = (sep_width - title_line_width) as i32 / 2;
        left_pad = " ".repeat(pad as usize);

        // let's see if the length of title string is even or odd
        // and accordingly decide to include the extra separator or not
        extra_pad = if (sep_width - title_line_width) % 2 == 0 {
            1
        } else {
            0
        };

        right_pad = " ".repeat((pad + extra_pad) as usize);

        println!("{:-<width$}", "", width = sep_width);
        println!("{}{}{}", left_pad, title, right_pad);
    };

    if sep_width > sub_title_line_width {
        pad = (sep_width - sub_title_line_width) as i32 / 2;
        left_pad = " ".repeat(pad as usize);

        // let's see if the length of title string is even or odd
        // and accordingly decide to include the extra separator or not
        extra_pad = if (sep_width - sub_title_line_width) % 2 == 0 {
            1
        } else {
            0
        };

        right_pad = " ".repeat((pad + extra_pad) as usize);

        println!("{}{}{}", left_pad, sub_title, right_pad);
        println!("{:-<width$}", "", width = sep_width);
    }
}
