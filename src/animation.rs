
type AnimationBuilder = Vec<(String, String)>;

#[derive(Debug)]
struct Animation {
    index: usize,
    frames: Vec<(String, String)>,
}

impl Animation {
    fn construct(builder: AnimationBuilder, max_focus_len: usize, min_focus_len: usize) -> Self {
        let max_frame_len = builder.iter().map(|(l, r)| l.len() + r.len()).max().unwrap_or(0_usize) + max_focus_len;

        Self {
            index: 0,
            frames: builder.into_iter()
                    .map(|(l, r)| {
                        let num_repeats = max_frame_len - min_focus_len - l.len() - r.len();
                        println!("{}", num_repeats);
                        ("\r".to_string() + l.as_str(), r + " ".repeat(num_repeats).as_str())
                    })
                    .collect()
        }
    }
}

pub fn test() {
    let builder: AnimationBuilder = vec![
        ("abcd".to_string(), "efgh".to_string()),
        ("123".to_string(), "456".to_string()),
    ];

    let mut anim = Animation::construct(builder, 6, 2);
    println!("{:#?}", anim);
}