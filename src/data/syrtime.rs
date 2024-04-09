use std::collections::HashMap;



pub(super) struct Blocs (HashMap<SyrDate, SyrDuration>);

struct SyrDate(time::Date);
struct SyrDuration(u64);